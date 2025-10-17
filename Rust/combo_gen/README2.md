ComboGen Optimem

A high-performance combination generator in Rust. Supports large combination sets with memory optimization, resume support, optional compression, and multi-threading.

Features

Generate all possible combinations for a custom charset and length.

Multi-threaded generation for maximum CPU usage.

Memory-efficient batching (optimem approach) with customizable buffer size.

Resume generation after interruption.

Optional gzip compression for huge combo lists.

Memory-only mode for small sets to avoid file I/O.

Verbose and dry-run modes for monitoring.

Support custom charsets (ASCII or UTF-8).

Installation

Clone the repository:

git clone https://github.com/chamath-adithya/combo_gen.git
cd combo_gen/Rust/combo_gen


Add dependencies:

cargo add indicatif flate2 num_cpus


Build release version:

cargo build --release

Usage
cargo run --release -- <length> [OPTIONS]

Options
Flag	Description
--threads N	Number of threads to use (default = CPU cores).
--limit N	Stop after generating N combinations.
--output path	Output file path (default = combos.txt).
--charset custom	Custom charset as a string (default = printable ASCII).
--batch N	Buffer size in bytes (default = 64 KB).
--resume path	Resume from a previous generation.
--compress gzip	Enable gzip compression of output.
--memory	Keep combinations in memory only (no file output).
--verbose	Print per-thread progress.
--dry-run	Generate combinations without writing to disk.
Examples
Generate 100,000 combos of length 8
cargo run --release -- 8 --limit 100000

Generate all combos of length 6 using 4 threads
cargo run --release -- 6 --threads 4

Generate combos with a custom charset and save to file
cargo run --release -- 5 --charset "abc123" --output mycombos.txt

Resume a previous interrupted generation
cargo run --release -- 8 --resume resume.txt --limit 500000

Generate combos in memory only (no file output)
cargo run --release -- 4 --memory --verbose

Generate combos with gzip compression
cargo run --release -- 8 --limit 100000 --compress gzip --output combos.gz

Dry-run mode (simulate generation without output)
cargo run --release -- 6 --dry-run --verbose

Performance Tips

Use --threads to match your CPU cores for maximum speed.

Increase --batch size for faster writes to disk on large datasets.

Use --compress gzip for huge outputs to save storage.

Resume support avoids losing progress on large runs.

Notes

Generating all 8-character ASCII printable combos is ~6.1 billion combinations — ensure your system has enough CPU and disk capacity.

For UTF-8 charsets, memory usage can grow rapidly; adjust batch size accordingly.

Use --memory only for small sets (<1M combos) to avoid memory exhaustion.
