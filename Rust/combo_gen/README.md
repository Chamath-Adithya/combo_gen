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

Performance Tips

Use --threads equal to your CPU cores for maximum speed.

Adjust --batch for larger buffer sizes to reduce I/O.

Resume generation for very large outputs to avoid starting over.

Use compressed output (--compress gzip) to save disk space for huge datasets.

License

MIT License © 2025 Chamath Adithya
