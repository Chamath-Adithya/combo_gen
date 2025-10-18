# Generate first 100,000 combos of length 8
cargo run --release -- 8 --limit 100000

# Generate all combos of length 6 using 4 threads
cargo run --release -- 6 --threads 4

# Generate combos of length 5 with a custom charset, save to file
cargo run --release -- 5 --charset "abc123" --output mycombos.txt

# Resume a previous interrupted generation from resume.txt
cargo run --release -- 8 --resume resume.txt --limit 500000

# Generate combos in memory only (no file output), verbose mode
cargo run --release -- 4 --memory --verbose

# Generate combos with gzip compression
cargo run --release -- 8 --limit 100000 --compress gzip --output combos.gz

# Dry-run mode (simulate generation without writing output), verbose
cargo run --release -- 6 --dry-run --verbose

