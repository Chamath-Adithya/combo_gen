ComboGen Optimem

A high-performance, memory-optimized, resumable combination generator written in Rust.

It can generate all possible combinations of a given length from a character set, optionally save to compressed files, resume after interruption, and use multiple threads for maximum throughput.

Features

Multi-threaded generation for maximum speed.

Optimized memory usage using per-thread buffers (optimem approach).

Resume support: safely continue after interruption.

Optional compressed output (gzip).

Memory-only mode for small sets to avoid I/O overhead.

Unicode/UTF-8 charset support.

CLI flags for batch size, verbosity, dry-run, and more.

Progress bar with ETA and throughput.

Build
Using Cargo
cargo build --release

Using Rustc
rustc -C opt-level=3 combo_gen_optimem.rs -o combo_gen_optimem

Usage
./combo_gen_optimem <length> [options]

Options
Flag	Description
--threads N	Number of threads to use (default: number of CPU cores).
--limit N	Limit the number of combinations generated.
--output path	Output file path (default: combos.txt).
--charset <string>	Custom character set (UTF-8 allowed).
--batch N	Buffer size per thread in bytes (default: 64 KB).
--resume path	Path to resume file. Saves last generated index for safe resuming.
--compress gzip	Compress output using gzip.
--memory	Memory-only mode, skips file writing.
--verbose	Show detailed thread progress.
--dry-run	Generate combos without writing to disk (good for benchmarking).
Examples
Generate all 8-character combos with default settings
./combo_gen_optimem 8

Generate first 1000 combos of length 5
./combo_gen_optimem 5 --limit 1000

Use 8 threads and save output to file
./combo_gen_optimem 6 --threads 8 --output combos.txt

Use a custom charset
./combo_gen_optimem 4 --charset "abc123!@"

Resume generation after interruption
./combo_gen_optimem 8 --resume resume.txt --limit 1000000

Generate compressed output
./combo_gen_optimem 8 --compress gzip --output combos.gz

Dry-run with verbose output
./combo_gen_optimem 5 --dry-run --verbose

Memory-only mode for small sets
./combo_gen_optimem 3 --memory

Quick Start / Benchmark Guide

These examples are rough estimates for a modern multi-core laptop (8 threads, SSD, 16 GB RAM) using the default charset of 94 ASCII characters.

Length	Total Combos	Approx. Time	Notes
4	78,074	<1 sec	Small, can fit in memory easily
5	7,737,809	~1-2 sec	Fast, multi-threaded
6	738,000,000	~20-40 sec	SSD recommended
7	69,343,957,000	~10-15 min	Use --resume and/or --compress gzip
8	6,531,000,000,000	hours-days	Use --resume, --compress gzip, and large batch buffer

Tips:

Use --threads equal to your CPU cores for maximum speed.

Use --batch to increase buffer size if generating huge outputs.

Use --resume to safely continue after interruptions.

Use --compress gzip to save disk space for large files.

Use --dry-run for quick throughput tests without writing to disk.

License

MIT License © 2025 Chamath Adithya
