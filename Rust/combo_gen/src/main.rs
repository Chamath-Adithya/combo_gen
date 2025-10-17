// combo_gen.rs
// Build: rustc -C opt-level=3 combo_gen.rs -o combo_gen
// Or with Cargo: cargo build --release
//
// Examples:
//  ./combo_gen 8 --limit 1000
//  ./combo_gen 4 --threads 8 --output combos.txt
//  ./combo_gen 3 --limit 10 --output -
//  ./combo_gen 8 --limit 500000 --output /dev/null

use std::env;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};

fn default_charset() -> Vec<u8> {
    (33u8..=126u8).collect() // 94 printable ASCII
}

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

fn index_to_digits(mut index: u64, base: u64, len: usize) -> Vec<u32> {
    let mut digits = vec![0u32; len];
    for pos in (0..len).rev() {
        digits[pos] = (index % base) as u32;
        index /= base;
    }
    digits
}

#[inline]
fn odometer_increment(digits: &mut [u32], base: u32) -> bool {
    let mut pos = digits.len();
    while pos > 0 {
        pos -= 1;
        let v = digits[pos].wrapping_add(1);
        if v < base {
            digits[pos] = v;
            return false;
        } else {
            digits[pos] = 0;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: {} <length> [--threads N] [--limit N] [--output path] [--charset custom]",
            args[0]
        );
        return;
    }

    let length: usize = args[1].parse().expect("length must be integer");

    // defaults
    let mut threads = num_cpus::get();
    let mut limit: Option<u64> = None;
    let mut output_path = String::from("/dev/null");
    let mut charset = default_charset();

    // parse flags
    let mut i = 2usize;
    while i < args.len() {
        match args[i].as_str() {
            "--threads" => {
                i += 1;
                threads = args[i].parse().expect("threads must be integer");
            }
            "--limit" => {
                i += 1;
                limit = Some(args[i].parse().expect("limit must be integer"));
            }
            "--output" => {
                i += 1;
                output_path = args[i].clone();
            }
            "--charset" => {
                i += 1;
                charset = args[i].as_bytes().to_vec();
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let base = charset.len() as u64;
    let total = match pow_u64(base, length) {
        Some(v) => v,
        None => {
            eprintln!("Total combinations overflow u64 — try smaller length/charset.");
            std::process::exit(1);
        }
    };

    println!("Charset size: {}", base);
    println!("Code length: {}", length);
    println!("Total combinations: {}", total);
    println!("Threads: {}", threads);

    let effective_total = limit.map_or(total, |l| l.min(total));
    println!(
        "Limit: {}",
        limit.map_or_else(|| "(none)".to_string(), |v| v.to_string())
    );
    println!("Output path: {}", output_path);

    if effective_total == 0 {
        println!("Nothing to do (limit=0).");
        return;
    }

    // open writer
    let writer: Arc<Mutex<Box<dyn Write + Send>>> = if output_path == "-" {
        Arc::new(Mutex::new(Box::new(io::stdout()) as Box<dyn Write + Send>))
    } else {
        Arc::new(Mutex::new(Box::new(BufWriter::with_capacity(
            1 << 20,
            File::create(&output_path).expect("Failed to open output file"),
        ))))
    };

    // setup progress bar
    let pb = ProgressBar::new(effective_total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent}% ({pos}/{len}) ETA:{eta}")
            .unwrap()
            .progress_chars("█░"),
    );

    let produced = Arc::new(AtomicU64::new(0));
    let start_time = Instant::now();

    let mut per_thread = effective_total / threads as u64;
    let mut remainder = effective_total % threads as u64;
    if per_thread == 0 {
        threads = effective_total as usize;
        per_thread = 1;
        remainder = 0;
        println!("Adjusted threads to {}", threads);
    }

    println!("Starting generation...\n");

    let mut handles = Vec::with_capacity(threads);
    let mut start_index = 0u64;

    for _ in 0..threads {
        let count = per_thread + if remainder > 0 { remainder -= 1; 1 } else { 0 };
        if count == 0 {
            break;
        }
        let start = start_index;
        start_index += count;

        let charset_local = charset.clone();
        let writer_clone = Arc::clone(&writer);
        let produced_clone = Arc::clone(&produced);
        let pb_clone = pb.clone();

        handles.push(thread::spawn(move || {
            let mut digits = index_to_digits(start, base, length);
            let base_u32 = base as u32;
            let mut buf = Vec::with_capacity(1 << 16);
            let mut local_count = 0u64;

            for _ in 0..count {
                for &d in &digits {
                    buf.push(charset_local[d as usize]);
                }
                buf.push(b'\n');
                local_count += 1;

                if buf.len() >= 32 * 1024 {
                    let mut w = writer_clone.lock().unwrap();
                    let _ = w.write_all(&buf);
                    buf.clear();
                }

                odometer_increment(&mut digits, base_u32);

                if local_count % 10_000 == 0 {
                    pb_clone.inc(10_000);
                }
            }

            if !buf.is_empty() {
                let mut w = writer_clone.lock().unwrap();
                let _ = w.write_all(&buf);
            }

            produced_clone.fetch_add(local_count, Ordering::Relaxed);
        }));
    }

    for h in handles {
        let _ = h.join();
    }

    {
        let mut w = writer.lock().unwrap();
        let _ = w.flush();
    }

    pb.finish_with_message("✅ Done!");
    let elapsed = start_time.elapsed().as_secs_f64();
    let total_done = produced.load(Ordering::Relaxed);

    println!("\nDone. Produced: {}", total_done);
    println!("Elapsed: {:.3} s", elapsed);
    println!("Throughput: {:.2} combos/sec", total_done as f64 / elapsed);
}

