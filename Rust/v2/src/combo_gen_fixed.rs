// combo_gen_optimem.rs - Fixed version
// Build: cargo build --release

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

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <length> [--threads N] [--limit N] [--output path] [--charset custom] [--batch N] [--resume path] [--compress gzip|none] [--memory] [--verbose] [--dry-run]", args[0]);
        return;
    }

    let length: usize = args[1].parse().expect("length must be integer");
    if length == 0 {
        eprintln!("Error: length must be greater than 0");
        return;
    }

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

    // Validate charset
    if charset.is_empty() {
        eprintln!("Error: charset cannot be empty");
        return;
    }

    // Validate threads
    if threads == 0 {
        threads = 1;
    }

    let base = charset.len() as u64;
    let total = match pow_u64(base, length) {
        Some(v) => v,
        None => { eprintln!("Total combinations overflow u64 – try smaller length/charset."); return; }
    };

    let effective_total = limit.map_or(total, |l| l.min(total));
    if effective_total == 0 { println!("Nothing to do."); return; }

    println!("Charset size: {}", base);
    println!("Code length: {}", length);
    println!("Total combinations: {}", total);
    println!("Threads: {}", threads);
    println!("Effective total: {}", effective_total);
    println!("Output path: {}", if memory_only || dry_run { "(none)" } else { &output_path });
    if compress { println!("Compression: gzip"); }
    if dry_run { println!("Mode: Dry-run (no output)"); }
    if memory_only { println!("Mode: Memory-only (no file output)"); }

    // Resume support
    let start_index = if let Some(ref resume) = resume_file {
        if Path::new(resume).exists() {
            let resume_str = std::fs::read_to_string(resume).unwrap_or_else(|_| "0".to_string());
            let idx = resume_str.trim().parse().unwrap_or(0);
            if idx > 0 {
                println!("Resuming from index: {}", idx);
            }
            idx
        } else { 0 }
    } else { 0 };

    if start_index >= effective_total {
        println!("Resume index {} >= effective total {}. Nothing to do.", start_index, effective_total);
        return;
    }

    // Progress bar
    let remaining = effective_total - start_index;
    let pb = ProgressBar::new(remaining);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent}% ({pos}/{len}) ETA:{eta}")
            .unwrap()
            .progress_chars("█▓▒░ ")
    );

    let produced = Arc::new(AtomicU64::new(0));
    let resume_counter = Arc::new(AtomicU64::new(start_index));
    let start_time = Instant::now();

    // Adjust threads for small limits
    let mut per_thread = remaining / threads as u64;
    let mut remainder = remaining % threads as u64;
    if per_thread == 0 {
        threads = remaining as usize;
        per_thread = 1;
        remainder = 0;
    }

    // Setup output writer
    let output_arc: Option<Arc<Mutex<Box<dyn Write + Send>>>> = if memory_only || dry_run {
        None
    } else {
        let file = File::create(&output_path).expect("Failed to create output file");
        let writer: Box<dyn Write + Send> = if compress {
            Box::new(BufWriter::with_capacity(batch_size, GzEncoder::new(file, Compression::default())))
        } else {
            Box::new(BufWriter::with_capacity(batch_size, file))
        };
        Some(Arc::new(Mutex::new(writer)))
    };

    // Storage for memory-only mode
    let memory_storage: Option<Arc<Mutex<Vec<Vec<u8>>>>> = if memory_only {
        Some(Arc::new(Mutex::new(Vec::new())))
    } else {
        None
    };

    let mut handles = Vec::with_capacity(threads);
    let mut current_index = start_index;

    for tid in 0..threads {
        let count = per_thread + if remainder > 0 { remainder -= 1; 1 } else { 0 };
        if count == 0 { break; }
        let start = current_index;
        current_index += count;

        let charset_local = charset.clone();
        let produced_clone = Arc::clone(&produced);
        let resume_counter_clone = Arc::clone(&resume_counter);
        let pb_clone = pb.clone();
        let output_clone = output_arc.clone();
        let memory_clone = memory_storage.clone();
        let verbose_clone = verbose;
        let dry_run_clone = dry_run;
        let batch_size_clone = batch_size;

        handles.push(thread::spawn(move || {
            let mut digits = index_to_digits(start, base, length);
            let base_u32 = base as u32;
            let mut buf = Vec::with_capacity(batch_size_clone);
            let mut local_memory = if memory_clone.is_some() { Some(Vec::new()) } else { None };
            let mut local_count = 0u64;

            for _ in 0..count {
                // Generate combination
                let combo: Vec<u8> = digits.iter().map(|&d| charset_local[d as usize]).collect();
                
                if let Some(ref mut mem) = local_memory {
                    // Memory-only mode: store combinations
                    mem.push(combo.clone());
                } else if !dry_run_clone {
                    // Normal mode: write to buffer
                    buf.extend_from_slice(&combo);
                    buf.push(b'\n');
                }

                local_count += 1;

                // Flush buffer when full (file mode only)
                if !dry_run_clone && output_clone.is_some() && buf.len() >= batch_size_clone {
                    if let Some(ref out) = output_clone {
                        let mut w = out.lock().unwrap();
                        w.write_all(&buf).expect("Failed to write to output");
                    }
                    buf.clear();
                }

                // Update resume counter atomically
                resume_counter_clone.fetch_add(1, Ordering::Relaxed);

                // Increment progress bar every iteration for accuracy
                pb_clone.inc(1);

                // Increment odometer
                odometer_increment(&mut digits, base_u32);
            }

            // Flush remaining buffer
            if !dry_run_clone && !buf.is_empty() {
                if let Some(ref out) = output_clone {
                    let mut w = out.lock().unwrap();
                    w.write_all(&buf).expect("Failed to write final buffer");
                }
            }

            // Store memory data if in memory-only mode
            if let Some(ref mem_storage) = memory_clone {
                if let Some(local_mem) = local_memory {
                    let mut storage = mem_storage.lock().unwrap();
                    storage.extend(local_mem);
                }
            }

            produced_clone.fetch_add(local_count, Ordering::Relaxed);
            if verbose_clone { 
                println!("Thread {} completed: {} combinations", tid, local_count); 
            }
        }));
    }

    // Wait for all threads
    for h in handles { 
        h.join().expect("Thread panicked"); 
    }

    // Final flush and cleanup
    if let Some(out) = output_arc {
        let mut w = out.lock().unwrap();
        w.flush().expect("Failed to flush output");
    }

    // Save resume state
    if let Some(ref resume) = resume_file {
        let final_index = resume_counter.load(Ordering::Relaxed);
        std::fs::write(resume, final_index.to_string())
            .expect("Failed to write resume file");
        if verbose {
            println!("Resume state saved: {}", final_index);
        }
    }

    pb.finish_with_message("✅ Done!");
    let elapsed = start_time.elapsed().as_secs_f64();
    let total_done = produced.load(Ordering::Relaxed);

    println!("\nGenerated: {} combinations", total_done);
    if start_index > 0 {
        println!("Total processed: {} (resumed from {})", start_index + total_done, start_index);
    }
    println!("Elapsed: {:.3} s", elapsed);
    println!("Throughput: {:.2} combos/sec", total_done as f64 / elapsed);

    // Display memory storage info if applicable
    if let Some(storage) = memory_storage {
        let data = storage.lock().unwrap();
        println!("Stored in memory: {} combinations", data.len());
        if verbose && !data.is_empty() {
            println!("First 5 samples:");
            for (i, combo) in data.iter().take(5).enumerate() {
                println!("  {}: {}", i + 1, String::from_utf8_lossy(combo));
            }
        }
    }
}
