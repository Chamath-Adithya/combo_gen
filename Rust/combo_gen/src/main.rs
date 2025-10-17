// combo_gen.rs
// Build: rustc -O combo_gen.rs -o combo_gen
// Or with cargo: cargo build --release
//
// Usage examples:
//  ./combo_gen 8                # length=8, threads=auto, output=/dev/null (fast benchmark)
//  ./combo_gen 5 --limit 1000   # generate first 1000 combos of length 5 and exit
//  ./combo_gen 4 --threads 8 --output combos.txt --limit 100000

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn default_charset() -> Vec<u8> {
    // 94 printable ASCII characters from '!' (33) to '~' (126)
    (33u8..=126u8).collect()
}

// compute base^exp as u128 then check fits u64
fn pow_u64(base: u64, exp: usize) -> Option<u64> {
    let mut r: u128 = 1;
    for _ in 0..exp {
        r = r * (base as u128);
        if r > (u64::MAX as u128) {
            return None;
        }
    }
    Some(r as u64)
}

// Convert a u64 index into digits in base `base` for `len` positions.
// This uses div/mod and is only called once per thread to initialize the odometer.
fn index_to_digits(mut index: u64, base: u64, len: usize) -> Vec<u32> {
    let mut digits = vec![0u32; len];
    // fill from last to first
    for pos in (0..len).rev() {
        digits[pos] = (index % base) as u32;
        index /= base;
    }
    digits
}

// increment odometer digits (base `base`). Returns true if overflowed past final (i.e., wrapped).
#[inline]
fn odometer_increment(digits: &mut [u32], base: u32) -> bool {
    let mut pos = digits.len();
    while pos > 0 {
        pos -= 1;
        let v = digits[pos].wrapping_add(1);
        if v < base {
            digits[pos] = v;
            return false; // no wrap
        } else {
            digits[pos] = 0;
            // carry to next pos
        }
    }
    true // wrapped past the first position
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <length> [--threads N] [--limit N] [--output path] [--charset custom]", args[0]);
        eprintln!("Default: threads = number of logical cores, output = /dev/null, charset = 94 printable ASCII chars");
        return;
    }

    // parse basic positional
    let length: usize = args[1].parse().expect("length must be an integer");

    // defaults
    let mut threads = num_cpus::get();
    let mut limit: Option<u64> = None;
    let mut output_path = String::from("/dev/null");
    let mut charset = default_charset();

    // parse optional flags (simple)
    let mut i = 2usize;
    while i < args.len() {
        match args[i].as_str() {
            "--threads" => {
                i += 1;
                if i >= args.len() { panic!("--threads requires a value"); }
                threads = args[i].parse().expect("threads must be integer");
            }
            "--limit" => {
                i += 1;
                if i >= args.len() { panic!("--limit requires a value"); }
                limit = Some(args[i].parse().expect("limit must be integer"));
            }
            "--output" => {
                i += 1;
                if i >= args.len() { panic!("--output requires a value"); }
                output_path = args[i].clone();
            }
            "--charset" => {
                i += 1;
                if i >= args.len() { panic!("--charset requires a value"); }
                let s = &args[i];
                charset = s.as_bytes().to_vec();
            }
            other => {
                eprintln!("Unknown arg: {}", other);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let base = charset.len() as u64;
    if base < 2 {
        panic!("charset must contain at least 2 characters");
    }

    // compute total combinations
    let total = match pow_u64(base, length) {
        Some(v) => v,
        None => {
            eprintln!("Total combinations overflow u64 — choose smaller length or smaller charset.");
            std::process::exit(1);
        }
    };

    println!("Charset size: {}", base);
    println!("Code length: {}", length);
    println!("Total combinations: {}", total);
    println!("Threads: {}", threads);
    if let Some(l) = limit {
        println!("Limit: {}", l);
    } else {
        println!("Limit: (none) — full space");
    }
    println!("Output path: {}", output_path);

    // If limit is present and smaller than total, we'll use that effective_total
    let effective_total = if let Some(l) = limit {
        if l == 0 {
            println!("Limit is 0 → nothing to do.");
            return;
        }
        if l > total { total } else { l }
    } else {
        total
    };

    // Open output file (buffered)
    let file = match File::create(&output_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open output '{}': {}", output_path, e);
            std::process::exit(1);
        }
    };

    let writer = Arc::new(parking_lot::Mutex::new(BufWriter::with_capacity(1 << 20, file))); // 1MB buffer

    // partition ranges across threads (simple contiguous ranges)
    let mut per_thread = effective_total / (threads as u64);
    let mut remainder = effective_total % (threads as u64);

    if per_thread == 0 {
        // fewer combos than threads, reduce thread count
        threads = effective_total as usize;
        per_thread = 1;
        remainder = 0;
        println!("Adjusted threads to {}", threads);
    }

    // Shared atomic counter for global produced
    let produced = Arc::new(AtomicU64::new(0));

    println!("Starting generation... (this may be I/O bound depending on output target)");
    let start_time = Instant::now();

    let mut handles = Vec::with_capacity(threads);

    // track start index for each thread
    let mut start_index: u64 = 0;
    for t in 0..threads {
        let count_for_thread = per_thread + if remainder > 0 { remainder -= 1; 1 } else { 0 };
        if count_for_thread == 0 {
            break;
        }
        let s_idx = start_index;
        start_index += count_for_thread;

        let charset_local = charset.clone();
        let writer_clone = Arc::clone(&writer);
        let produced_clone = Arc::clone(&produced);

        let handle = thread::spawn(move || {
            // initialize odometer digits for this start index
            let mut digits = index_to_digits(s_idx, base, length);
            let base_u32 = base as u32;

            // build buffer for batched writes to reduce lock contention
            let mut local_buf = Vec::with_capacity(1 << 16); // 64KB local buffer
            let mut local_count: u64 = 0;

            for _ in 0..count_for_thread {
                // map digits to bytes
                for &d in &digits {
                    local_buf.push(charset_local[d as usize]);
                }
                local_buf.push(b'\n');

                local_count += 1;

                // if local buffer is big, flush to global writer
                if local_buf.len() >= 32 * 1024 {
                    // lock and write
                    let mut w = writer_clone.lock();
                    let _ = w.write_all(&local_buf);
                    local_buf.clear();
                }

                // increment
                let wrapped = odometer_increment(&mut digits, base_u32);
                if wrapped {
                    // we've wrapped the odometer — but since each thread has bounded count, this is fine
                }

                // optionally we could check for global limit, but partitioning avoids frequent atomics
            }

            // final flush of local buffer
            if !local_buf.is_empty() {
                let mut w = writer_clone.lock();
                let _ = w.write_all(&local_buf);
                local_buf.clear();
            }

            // update produced counter
            produced_clone.fetch_add(local_count, Ordering::Relaxed);
            // return local_count
            local_count
        });

        handles.push(handle);
    }

    // join threads and collect
    let mut total_produced: u64 = 0;
    for h in handles {
        match h.join() {
            Ok(cnt) => total_produced += cnt,
            Err(_) => eprintln!("Thread panicked"),
        }
    }

    // ensure writer flush
    {
        let mut w = writer.lock();
        let _ = w.flush();
    }

    let elapsed = start_time.elapsed();
    let secs = elapsed.as_secs_f64();

    println!("Done. Produced: {}", total_produced);
    println!("Elapsed: {:.3} s", secs);
    if secs > 0.0 {
        println!("Throughput: {:.3} combos/sec", (total_produced as f64) / secs);
    }
}

