// combo_gen_optimem.rs
// Build: rustc -C opt-level=3 combo_gen_optimem.rs -o combo_gen_optimem
// Or: cargo build --release

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use flate2::write::GzEncoder;
use flate2::Compression;

fn default_charset() -> Vec<u8> {
    (33u8..=126u8).collect() // printable ASCII
}

// Safe power with u64 overflow detection
fn pow_u64(base: u64, exp: usize) -> Option<u64> {
    let mut result: u128 = 1;
    for _ in 0..exp {
        result *= base as u128;
        if result > u64::MAX as u128 {
            return None;
        }
    }
    Some(result as u64)
}

// Convert linear index to digits in a given base
fn index_to_digits(mut index: u64, base: u64, len: usize) -> Vec<u32> {
    let mut digits = vec![0u32; len];
    for pos in (0..len).rev() {
        digits[pos] = (index % base) as u32;
        index /= base;
    }
    digits
}

// Odometer increment
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
        eprintln!("Usage: {} <length> [--threads N] [--limit N] [--output path] [--charset custom] [--batch N] [--resume path] [--compress gzip|none] [--memory] [--verbose] [--dry-run]", args[0]);
        return;
    }

    let length: usize = args[1].parse().expect("length must be integer");

    // Defaults
    let mut threads = num_cpus::get();
    let mut limit: Option<u64> = None;
    let mut output_path = String::from("combos.txt");
    let mut charset = default_charset();
    let mut batch_size: usize = 64 * 1024; // 64 KB buffer
    let mut resume_file: Option<String> = None;
    let mut compress = false;
    let mut memory_only = false;
    let mut verbose = false;
    let mut dry_run = false;

    // Parse flags
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--threads" => { i += 1; threads = args[i].parse().expect("threads must be integer"); }
            "--limit" => { i += 1; limit = Some(args[i].parse().expect("limit must be integer")); }
            "--output" => { i += 1; output_path = args[i].clone(); }
            "--charset" => { i += 1; charset = args[i].as_bytes().to_vec(); }
            "--batch" => { i += 1; batch_size = args[i].parse().expect("batch must be integer"); }
            "--resume" => { i += 1; resume_file = Some(args[i].clone()); }
            "--compress" => { i += 1; compress = matches!(args[i].as_str(), "gzip"); }
            "--memory" => { memory_only = true; }
            "--verbose" => { verbose = true; }
            "--dry-run" => { dry_run = true; }
            _ => { eprintln!("Unknown argument: {}", args[i]); std::process::exit(1); }
        }
        i += 1;
    }

    let base = charset.len() as u64;
    let total = match pow_u64(base, length) {
        Some(v) => v,
        None => { eprintln!("Total combinations overflow u64 — try smaller length/charset."); return; }
    };

    let effective_total = limit.map_or(total, |l| l.min(total));
    if effective_total == 0 { println!("Nothing to do."); return; }

    println!("Charset size: {}", base);
    println!("Code length: {}", length);
    println!("Total combinations: {}", total);
    println!("Threads: {}", threads);
    println!("Effective total: {}", effective_total);
    println!("Output path: {}", if memory_only { "(memory-only)" } else { &output_path });
    if compress { println!("Compression: gzip"); }

    // Resume support
    let start_index = if let Some(ref resume) = resume_file {
        if Path::new(resume).exists() {
            std::fs::read_to_string(resume).unwrap_or_else(|_| "0".to_string()).trim().parse().unwrap_or(0)
        } else { 0 }
    } else { 0 };

    // Progress bar
    let pb = ProgressBar::new(effective_total - start_index);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent}% ({pos}/{len}) ETA:{eta}")
            .unwrap()
            .progress_chars("█░")
    );

    let produced = Arc::new(AtomicU64::new(0));
    let start_time = Instant::now();

    // Adjust threads for small limits
    let mut per_thread = (effective_total - start_index) / threads as u64;
    let mut remainder = (effective_total - start_index) % threads as u64;
    if per_thread == 0 {
        threads = (effective_total - start_index) as usize;
        per_thread = 1;
        remainder = 0;
    }

    // Setup output writer
    let output_arc: Option<Arc<Mutex<Box<dyn Write + Send>>>> = if memory_only || dry_run {
        None
    } else {
        let file = File::create(&output_path).expect("Failed to open file");
        let writer: Box<dyn Write + Send> = if compress {
            Box::new(BufWriter::with_capacity(batch_size, GzEncoder::new(file, Compression::best())))
        } else {
            Box::new(BufWriter::with_capacity(batch_size, file))
        };
        Some(Arc::new(Mutex::new(writer)))
    };

    let mut handles = Vec::with_capacity(threads);
    let mut current_index = start_index;

    for _ in 0..threads {
        let count = per_thread + if remainder > 0 { remainder -= 1; 1 } else { 0 };
        if count == 0 { break; }
        let start = current_index;
        current_index += count;

        let charset_local = charset.clone();
        let produced_clone = Arc::clone(&produced);
        let pb_clone = pb.clone();
        let output_clone = output_arc.clone();
        let resume_clone = resume_file.clone();
        let verbose_clone = verbose;
        let dry_run_clone = dry_run;
        let batch_size_clone = batch_size;

        handles.push(thread::spawn(move || {
            let mut digits = index_to_digits(start, base, length);
            let base_u32 = base as u32;
            let mut buf = Vec::with_capacity(batch_size_clone);
            let mut local_count = 0u64;

            for _ in 0..count {
                for &d in &digits { buf.push(charset_local[d as usize]); }
                buf.push(b'\n');
                local_count += 1;

                if buf.len() >= batch_size_clone {
                    if let Some(ref out) = output_clone {
                        let mut w = out.lock().unwrap();
                        let _ = w.write_all(&buf);
                    }
                    buf.clear();
                }

                if let Some(ref resume) = resume_clone {
                    if local_count % 100_000 == 0 {
                        let _ = std::fs::write(resume, &(start + local_count).to_string());
                    }
                }

                odometer_increment(&mut digits, base_u32);
                if local_count % 50_000 == 0 { pb_clone.inc(50_000); }
            }

            if !buf.is_empty() {
                if let Some(ref out) = output_clone {
                    let mut w = out.lock().unwrap();
                    let _ = w.write_all(&buf);
                }
            }

            produced_clone.fetch_add(local_count, Ordering::Relaxed);
            if verbose_clone { println!("Thread done: {}", local_count); }
            if dry_run_clone { println!("Dry-run produced: {}", local_count); }
        }));
    }

    for h in handles { let _ = h.join(); }

    if let Some(out) = output_arc {
        let mut w = out.lock().unwrap();
        let _ = w.flush();
    }

    pb.finish_with_message("✅ Done!");
    let elapsed = start_time.elapsed().as_secs_f64();
    let total_done = produced.load(Ordering::Relaxed);

    println!("\nProduced: {}", total_done);
    println!("Elapsed: {:.3} s", elapsed);
    println!("Throughput: {:.2} combos/sec", total_done as f64 / elapsed);
}

