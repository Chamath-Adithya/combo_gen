// combo_gen_optimem.rs - Ultra-Fast SIMD-Ready Version
// Build: RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release

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
    (33u8..=126u8).collect()
}

fn pow_u64(base: u64, exp: usize) -> Option<u64> {
    base.checked_pow(exp as u32)
        .or_else(|| {
            let mut result: u128 = 1;
            for _ in 0..exp {
                result = result.checked_mul(base as u128)?;
                if result > u64::MAX as u128 {
                    return None;
                }
            }
            Some(result as u64)
        })
}

#[inline(always)]
fn index_to_digits(mut index: u64, base: u64, digits: &mut [u32]) {
    for pos in (0..digits.len()).rev() {
        digits[pos] = (index % base) as u32;
        index /= base;
    }
}

#[inline(always)]
fn odometer_increment(digits: &mut [u32], base: u32) -> bool {
    for pos in (0..digits.len()).rev() {
        digits[pos] += 1;
        if digits[pos] < base {
            return false;
        }
        digits[pos] = 0;
    }
    true
}

// Unrolled generation for common lengths
#[inline(always)]
fn generate_combo_fast(digits: &[u32], charset: &[u8], out: &mut Vec<u8>) {
    match digits.len() {
        1 => {
            out.push(charset[digits[0] as usize]);
            out.push(b'\n');
        }
        2 => {
            out.push(charset[digits[0] as usize]);
            out.push(charset[digits[1] as usize]);
            out.push(b'\n');
        }
        3 => {
            out.push(charset[digits[0] as usize]);
            out.push(charset[digits[1] as usize]);
            out.push(charset[digits[2] as usize]);
            out.push(b'\n');
        }
        4 => {
            out.push(charset[digits[0] as usize]);
            out.push(charset[digits[1] as usize]);
            out.push(charset[digits[2] as usize]);
            out.push(charset[digits[3] as usize]);
            out.push(b'\n');
        }
        5 => {
            out.push(charset[digits[0] as usize]);
            out.push(charset[digits[1] as usize]);
            out.push(charset[digits[2] as usize]);
            out.push(charset[digits[3] as usize]);
            out.push(charset[digits[4] as usize]);
            out.push(b'\n');
        }
        6 => {
            out.push(charset[digits[0] as usize]);
            out.push(charset[digits[1] as usize]);
            out.push(charset[digits[2] as usize]);
            out.push(charset[digits[3] as usize]);
            out.push(charset[digits[4] as usize]);
            out.push(charset[digits[5] as usize]);
            out.push(b'\n');
        }
        7 => {
            out.push(charset[digits[0] as usize]);
            out.push(charset[digits[1] as usize]);
            out.push(charset[digits[2] as usize]);
            out.push(charset[digits[3] as usize]);
            out.push(charset[digits[4] as usize]);
            out.push(charset[digits[5] as usize]);
            out.push(charset[digits[6] as usize]);
            out.push(b'\n');
        }
        8 => {
            out.push(charset[digits[0] as usize]);
            out.push(charset[digits[1] as usize]);
            out.push(charset[digits[2] as usize]);
            out.push(charset[digits[3] as usize]);
            out.push(charset[digits[4] as usize]);
            out.push(charset[digits[5] as usize]);
            out.push(charset[digits[6] as usize]);
            out.push(charset[digits[7] as usize]);
            out.push(b'\n');
        }
        _ => {
            for &d in digits {
                out.push(charset[d as usize]);
            }
            out.push(b'\n');
        }
    }
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

    let mut threads = num_cpus::get();
    let mut limit: Option<u64> = None;
    let mut output_path = String::from("combos.txt");
    let mut charset = default_charset();
    let mut batch_size: usize = 2 * 1024 * 1024; // 2MB for even better throughput
    let mut resume_file: Option<String> = None;
    let mut compress = false;
    let mut memory_only = false;
    let mut verbose = false;
    let mut dry_run = false;

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

    if charset.is_empty() {
        eprintln!("Error: charset cannot be empty");
        return;
    }
    if threads == 0 { threads = 1; }

    let base = charset.len() as u64;
    let total = match pow_u64(base, length) {
        Some(v) => v,
        None => { eprintln!("Total combinations overflow u64"); return; }
    };

    let effective_total = limit.map_or(total, |l| l.min(total));
    if effective_total == 0 { println!("Nothing to do."); return; }

    println!("╔═══════════════════════════════════════╗");
    println!("║      ComboGen Ultra-Fast Mode         ║");
    println!("╚═══════════════════════════════════════╝");
    println!("Charset size: {}", base);
    println!("Code length: {}", length);
    println!("Total combinations: {}", total);
    println!("Threads: {}", threads);
    println!("Effective total: {}", effective_total);
    println!("Buffer size: {} MB", batch_size / 1024 / 1024);
    println!("Output: {}", if memory_only || dry_run { "(none)" } else { &output_path });

    let start_index = if let Some(ref resume) = resume_file {
        if Path::new(resume).exists() {
            let idx = std::fs::read_to_string(resume).unwrap_or_else(|_| "0".to_string()).trim().parse().unwrap_or(0);
            if idx > 0 { println!("Resuming from: {}", idx); }
            idx
        } else { 0 }
    } else { 0 };

    if start_index >= effective_total {
        println!("Nothing to do (resume >= total)");
        return;
    }

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

    let mut per_thread = remaining / threads as u64;
    let mut remainder = remaining % threads as u64;
    if per_thread == 0 {
        threads = remaining as usize;
        per_thread = 1;
        remainder = 0;
    }

    let output_arc: Option<Arc<Mutex<Box<dyn Write + Send>>>> = if memory_only || dry_run {
        None
    } else {
        let file = File::create(&output_path).expect("Failed to create output file");
        let writer: Box<dyn Write + Send> = if compress {
            Box::new(BufWriter::with_capacity(batch_size * 2, GzEncoder::new(file, Compression::fast())))
        } else {
            Box::new(BufWriter::with_capacity(batch_size * 2, file))
        };
        Some(Arc::new(Mutex::new(writer)))
    };

    let memory_storage: Option<Arc<Mutex<Vec<Vec<u8>>>>> = if memory_only {
        Some(Arc::new(Mutex::new(Vec::with_capacity(remaining.min(10_000_000) as usize))))
    } else {
        None
    };

    let mut handles = Vec::with_capacity(threads);
    let mut current_index = start_index;

    const PROGRESS_BATCH: u64 = 50_000;
    const WRITE_THRESHOLD: usize = 1024 * 1024; // 1MB before write

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

        handles.push(thread::spawn(move || {
            let mut digits = vec![0u32; length];
            index_to_digits(start, base, &mut digits);
            let base_u32 = base as u32;
            
            let mut buf = Vec::with_capacity(batch_size + length + 1);
            let mut local_count = 0u64;
            let mut progress_acc = 0u64;

            if memory_clone.is_some() {
                let mut local_memory = Vec::with_capacity((count as usize).min(100_000));
                for _ in 0..count {
                    let combo: Vec<u8> = digits.iter().map(|&d| charset_local[d as usize]).collect();
                    local_memory.push(combo);
                    odometer_increment(&mut digits, base_u32);
                    local_count += 1;
                    progress_acc += 1;
                    if progress_acc >= PROGRESS_BATCH {
                        pb_clone.inc(progress_acc);
                        resume_counter_clone.fetch_add(progress_acc, Ordering::Relaxed);
                        progress_acc = 0;
                    }
                }
                let mut storage = memory_clone.as_ref().unwrap().lock().unwrap();
                storage.extend(local_memory);
            } else if dry_run {
                for _ in 0..count {
                    odometer_increment(&mut digits, base_u32);
                    local_count += 1;
                }
                pb_clone.inc(count);
            } else {
                for _ in 0..count {
                    generate_combo_fast(&digits, &charset_local, &mut buf);
                    local_count += 1;
                    progress_acc += 1;

                    if progress_acc >= PROGRESS_BATCH {
                        pb_clone.inc(progress_acc);
                        resume_counter_clone.fetch_add(progress_acc, Ordering::Relaxed);
                        progress_acc = 0;
                    }

                    if buf.len() >= WRITE_THRESHOLD {
                        if let Some(ref out) = output_clone {
                            let mut w = out.lock().unwrap();
                            let _ = w.write_all(&buf);
                        }
                        buf.clear();
                    }

                    odometer_increment(&mut digits, base_u32);
                }

                if !buf.is_empty() {
                    if let Some(ref out) = output_clone {
                        let mut w = out.lock().unwrap();
                        let _ = w.write_all(&buf);
                    }
                }
            }

            if progress_acc > 0 {
                pb_clone.inc(progress_acc);
                resume_counter_clone.fetch_add(progress_acc, Ordering::Relaxed);
            }

            produced_clone.fetch_add(local_count, Ordering::Relaxed);
            if verbose { println!("Thread {} done: {}", tid, local_count); }
        }));
    }

    for h in handles { h.join().expect("Thread panicked"); }

    if let Some(out) = output_arc {
        let mut w = out.lock().unwrap();
        w.flush().expect("Failed to flush");
    }

    if let Some(ref resume) = resume_file {
        let final_idx = resume_counter.load(Ordering::Relaxed);
        let _ = std::fs::write(resume, final_idx.to_string());
    }

    pb.finish_with_message("✅ Complete!");
    let elapsed = start_time.elapsed().as_secs_f64();
    let total_done = produced.load(Ordering::Relaxed);

    println!("\n╔═══════════════════════════════════════╗");
    println!("║          Performance Report           ║");
    println!("╚═══════════════════════════════════════╝");
    println!("Generated: {:>20}", format_number(total_done));
    if start_index > 0 {
        println!("Resumed from: {:>18}", format_number(start_index));
    }
    println!("Time: {:>25.3} s", elapsed);
    println!("Throughput: {:>17.2} M/s", total_done as f64 / elapsed / 1_000_000.0);
    
    let bytes_written = total_done * (length + 1) as u64;
    println!("Data written: {:>19}", format_bytes(bytes_written));
    println!("Write speed: {:>18.2} MB/s", bytes_written as f64 / elapsed / 1_048_576.0);
    println!("╚═══════════════════════════════════════╝");

    if let Some(storage) = memory_storage {
        let data = storage.lock().unwrap();
        println!("\nStored in memory: {} combinations", data.len());
        if verbose && !data.is_empty() {
            println!("Samples:");
            for (i, combo) in data.iter().take(5).enumerate() {
                println!("  {}: {}", i + 1, String::from_utf8_lossy(combo));
            }
        }
    }
}

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
