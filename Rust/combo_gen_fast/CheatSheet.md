Comprehensive Guide to ComboGen: Canvassing the Tool with Examples Across All Programs
Introduction
ComboGen is a powerful, multi-threaded Rust tool designed for generating exhaustive or limited combinations from a specified charset and length. It's useful for tasks like password generation, combinatorial testing, data simulation, and educational demonstrations of brute-force concepts. The project emphasizes performance, stability, and flexibility, with three core versions tailored to different priorities:

Fixed Version (combo_gen_fixed): Prioritizes correctness, bug fixes, and reliability. Ideal for beginners or when accuracy is paramount (e.g., small-scale generations where speed isn't critical).
Optimized Version (combo_gen_optimized): Balances stability with performance enhancements like larger buffers and batched updates. Great for medium-sized jobs where you want 2-3x speed gains without complexity.
Ultra-Fast Version (combo_gen_ultra): Maximizes throughput with advanced optimizations (e.g., loop unrolling, larger buffers, SIMD-ready code). Best for large-scale generations on high-end hardware, offering 3-5x speed over the fixed version.
Unified Entry Point (combo_gen): A wrapper that defaults to ultra-fast but allows switching via --version fixed|optimized|ultra. Convenient for testing without changing binaries.

This guide "canvasses" the tool—meaning we thoroughly explore its features, usage, and differences—using various examples. We'll cover setup, commands, and scenarios like password cracking simulations, test data creation, resume functionality, and more. All examples assume you've built the project with cargo build --release (use RUSTFLAGS="-C target-cpu=native" for ultra-fast max performance). Run specific versions with cargo run --bin <name> --release -- <args>, or the unified with cargo run --bin combo_gen --release -- <args> [--version <version>].
Benchmarks (from version_comparison.md) are referenced for performance insights. Outputs include progress bars, stats (e.g., throughput), and files like combos.txt (one combo per line).
Setup and Basic Usage
After updating Cargo.toml as provided, build all binaries:
textcargo build --release
This creates executables in target/release/. Direct invocation (e.g., ./target/release/combo_gen_fixed 5 --limit 1000) is faster than cargo run for repeated use.
Help is printed if arguments are missing:
textUsage: <program> <length> [--threads N] [--limit N] [--output path] [--charset custom] [--batch N] [--resume path] [--compress gzip|none] [--memory] [--verbose] [--dry-run]
Key flags:

<length>: Combo length (e.g., 5).
--threads N: Threads (default: CPU cores).
--limit N: Cap generations.
--charset "str": Custom characters (default: ! to ~, 94 chars).
--resume path: Resume from index in file.
--compress gzip: Gzip output.
--memory: Store in RAM only.
--dry-run: Simulate (no output, tests speed).
--verbose: Thread details.

Total combos = charset.len() ^ length (overflows error out).
Canvassing with Examples
We'll explore scenarios, showing commands for all programs (fixed, optimized, ultra, unified). Differences: Fixed is slowest but most accurate progress; optimized batches for speed; ultra unrolls loops for max throughput. Use --dry-run in examples to benchmark without I/O.
Example 1: Small-Scale Generation (Educational Demo of Combinatorial Explosion)
Scenario: Teach students about exponential growth by generating all 3-length combos from a small charset (e.g., "abc"). Total: 3^3 = 27. Useful for math classes or intro programming.
Commands (outputs to combos.txt with lines like "aaa\nabb\n..."):

Fixed: cargo run --bin combo_gen_fixed --release -- 3 --charset "abc"
Optimized: cargo run --bin combo_gen_optimized --release -- 3 --charset "abc"
Ultra: cargo run --bin combo_gen_ultra --release -- 3 --charset "abc"
Unified: cargo run --bin combo_gen --release -- 3 --charset "abc" --version optimized (switches version)

Expected Output (all versions similar, ~0.1s):
textCharset size: 3
Code length: 3
Total combinations: 27
Threads: <your cores>
Effective total: 27
...
Generated: 27 combinations
Elapsed: 0.100 s
Throughput: 270.00 combos/sec
Differences: Fixed updates progress every iteration (precise but overhead); ultra batches 50k (irrelevant for small N, but faster overall). Use --verbose to see thread completion.
Example 2: Password Generation Simulation (Security Testing)
Scenario: Simulate brute-force password cracking by generating 100k 8-length combos from default charset (94^8 ≈ 6e15 total, but limited). Useful for cybersecurity demos, showing why long passwords are secure.
Commands (limit to 100k for feasibility; use --threads 8 for parallelism):

Fixed: cargo run --bin combo_gen_fixed --release -- 8 --limit 100000 --threads 8 --output passwords.txt
Optimized: cargo run --bin combo_gen_optimized --release -- 8 --limit 100000 --threads 8 --output passwords.txt
Ultra: cargo run --bin combo_gen_ultra --release -- 8 --limit 100000 --threads 8 --output passwords.txt
Unified: cargo run --bin combo_gen --release -- 8 --limit 100000 --threads 8 --output passwords.txt --version fixed

Benchmark Insights (from version_comparison.md, similar setup):

Fixed: ~42s, 236K/s
Optimized: ~16s, 617K/s (2.6x faster)
Ultra: ~11s, 877K/s (3.7x faster)

Output snippet:
textGenerated: 100000 combinations
Throughput: <varies> combos/sec
Differences: Ultra's loop unrolling shines for length 8; optimized's 1MB buffer reduces I/O stalls vs. fixed's 64KB. Add --compress gzip --output passwords.gz to save space (slows by 20-40%).
Example 3: Test Data Creation (Software QA)
Scenario: Generate 1M 5-length combos from "0123456789" (10^5 = 100k total, but limit 1M caps at all). For QA testing input fields, like phone codes or IDs.
Commands:

Fixed: cargo run --bin combo_gen_fixed --release -- 5 --charset "0123456789" --limit 1000000 --dry-run (dry-run for speed test)
Optimized: cargo run --bin combo_gen_optimized --release -- 5 --charset "0123456789" --limit 1000000 --dry-run
Ultra: cargo run --bin combo_gen_ultra --release -- 5 --charset "0123456789" --limit 1000000 --dry-run
Unified: cargo run --bin combo_gen --release -- 5 --charset "0123456789" --limit 1000000 --dry-run --version ultra

Expected: All complete quickly (<1s for actual, dry-run measures pure gen speed).
Differences: Fixed: Accurate but slower progress; Optimized: Batched (10k) for less overhead; Ultra: 50k batches + unrolling = fastest. Throughput: Ultra ~769K/s vs. fixed ~192K/s (from benchmarks).
Example 4: Resuming Interrupted Jobs (Long-Running Tasks)
Scenario: Generating all 6-length combos (94^6 ≈ 7e11, but limit 500k). Interrupt (Ctrl+C), then resume. Useful for coupon code generation or large datasets.
Start:

Fixed: cargo run --bin combo_gen_fixed --release -- 6 --limit 500000 --resume resume.txt
(Interrupt after ~100k.)

Resume (same command re-runs, starts from index in resume.txt):

Same for all versions.

Differences: All use atomic counters for thread-safe resume (fixed from original buggy version). Ultra saves resume less often but accurately.
Output on resume:
textResuming from index: 100000
...
Total processed: 500000 (resumed from 100000)
Example 5: Memory-Only Mode (In-Memory Processing)
Scenario: Generate 4-length combos verbally for scripting (no file). Useful for piping to other tools or small sets.
Commands (stores in RAM, prints samples if --verbose):

Fixed: cargo run --bin combo_gen_fixed --release -- 4 --memory --verbose
Optimized: cargo run --bin combo_gen_optimized --release -- 4 --memory --verbose
Ultra: cargo run --bin combo_gen_ultra --release -- 4 --memory --verbose
Unified: cargo run --bin combo_gen --release -- 4 --memory --verbose --version optimized

Output:
textStored in memory: <total> combinations
First 5 samples:
  1: !!!!
  2: !!!"
  ...
Differences: Fixed/Optimized store Vec<Vec<u8>>; Ultra caps capacity for efficiency. RAM usage: ~ (length +1) * total.
Example 6: Compression for Large Files (Data Archiving)
Scenario: Generate 100k 8-length combos, compress for storage. Useful for archiving test datasets.
Commands:

Fixed: cargo run --bin combo_gen_fixed --release -- 8 --limit 100000 --compress gzip --output archive.gz
Optimized: cargo run --bin combo_gen_optimized --release -- 8 --limit 100000 --compress gzip --output archive.gz
Ultra: cargo run --bin combo_gen_ultra --release -- 8 --limit 100000 --compress gzip --output archive.gz
Unified: cargo run --bin combo_gen --release -- 8 --limit 100000 --compress gzip --output archive.gz

Differences: All use flate2; ultra uses fast compression level. Speed penalty: 20-40% slower, but file size ~30% smaller.
Example 7: Dry-Run for Benchmarking (Performance Tuning)
Scenario: Test throughput without I/O, varying threads. Useful for optimizing hardware configs.
Commands (e.g., length 6, limit 1M):

Fixed: cargo run --bin combo_gen_fixed --release -- 6 --limit 1000000 --dry-run --threads 16
Optimized: cargo run --bin combo_gen_optimized --release -- 6 --limit 1000000 --dry-run --threads 16
Ultra: cargo run --bin combo_gen_ultra --release -- 6 --limit 1000000 --dry-run --threads 16
Unified: cargo run --bin combo_gen --release -- 6 --limit 1000000 --dry-run --threads 16 --version ultra

From benchmarks: Ultra excels on multi-core (e.g., 4x speedup on length 8).
Conclusion
ComboGen canvasses combinatorial generation effectively across versions. Choose fixed for reliability, optimized for balance, ultra for speed. Experiment with unified for flexibility. For custom needs, modify the open-source code. See version_comparison.md for more metrics. Happy generating! 🎉