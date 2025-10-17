# ComboGen 🚀 - Comprehensive Guide & Cheatsheet

A **high-performance** Rust tool for generating combinations, optimized for speed, flexibility, and scalability.

---

## 📦 Installation & Build

### Prerequisites

* Rust 1.70+ ([Install Rust](https://rustup.rs/))

### Clone & Build

```bash
git clone https://github.com/chamath-adithya/combo_gen.git
cd combo_gen/Rust/combo_gen

# Standard build
cargo build --release

# Ultra-fast build with maximum optimizations
RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=fat" cargo build --release
```

Binaries are located in `target/release/`:

* `combo_gen_fixed` ✅ Fixed (stable)
* `combo_gen_optimized` ⚡ Optimized (balanced)
* `combo_gen_ultra` 🚀 Ultra-Fast (max speed)
* `combo_gen` 🧩 Unified entry point

---

## 🚀 Quick Start

### Basic Usage

```bash
# Generate 100k combinations of length 8
cargo run --release -- 8 --limit 100000

# Generate all combinations of length 4 using 8 threads
cargo run --release -- 4 --threads 8

# Custom charset
cargo run --release -- 5 --charset "abc123" --output custom.txt
```

### Advanced Usage

```bash
# Resume interrupted generation
cargo run --release -- 8 --resume resume.txt --limit 500000

# Memory-only mode (no file output)
cargo run --release -- 4 --memory --verbose

# With gzip compression
cargo run --release -- 8 --limit 100000 --compress gzip --output combos.gz

# Dry-run (test speed without writing)
cargo run --release -- 6 --dry-run --verbose
```

---

## 📖 Command Line Options

```
cargo run --release -- <length> [OPTIONS]
```

| Option             | Description               | Default         |
| ------------------ | ------------------------- | --------------- |
| `<length>`         | Length of combinations    | Required        |
| `--threads N`      | Number of threads         | CPU cores       |
| `--limit N`        | Stop after N combinations | All             |
| `--output path`    | Output file path          | combos.txt      |
| `--charset custom` | Custom charset            | ASCII printable |
| `--batch N`        | Buffer size (bytes)       | 2 MB            |
| `--resume path`    | Resume from file          | None            |
| `--compress gzip`  | Enable gzip compression   | Off             |
| `--memory`         | Keep in memory only       | Off             |
| `--verbose`        | Show detailed progress    | Off             |
| `--dry-run`        | Generate without writing  | Off             |

---

## 💡 Cheatsheet: Commands & Scenarios

### 1️⃣ Small-Scale Generation (Educational Demo)

```bash
# Generate all 3-length combos from charset "abc"
cargo run --bin combo_gen_fixed --release -- 3 --charset "abc"
```

* Total combinations: 27
* Ideal for teaching combinatorial growth

### 2️⃣ Password Generation Simulation

```bash
cargo run --release -- 8 --limit 100000 --threads 8 --output passwords.txt
```

* Shows importance of long passwords
* Benchmarks: Fixed ~42s, Optimized ~16s, Ultra ~11s

### 3️⃣ Test Data Creation (QA)

```bash
cargo run --release -- 5 --charset "0123456789" --limit 1000000 --dry-run
```

* Generates numeric 5-length combos quickly
* Useful for testing input fields, IDs, etc.

### 4️⃣ Resuming Interrupted Jobs

```bash
# Start generation
cargo run --release -- 6 --limit 500000 --resume resume.txt
```

* Resume safely after crash
* Atomic counters ensure thread safety

### 5️⃣ Memory-Only Mode

```bash
cargo run --release -- 4 --memory --verbose
```

* Store combos in RAM only
* Print first few samples
* Useful for piping into other programs

### 6️⃣ Compressed Output

```bash
cargo run --release -- 8 --limit 100000 --compress gzip --output archive.gz
```

* Saves disk space
* Ultra mode uses fast compression
* Speed penalty ~20-40%

### 7️⃣ Dry-Run Benchmarking

```bash
cargo run --release -- 6 --limit 1000000 --dry-run --threads 16
```

* Tests throughput without I/O overhead
* Helps optimize thread count and buffer size

---

## ⚙️ Performance Tuning

* **Thread Count**: `--threads N` to match CPU cores
* **Buffer Size**: `--batch N` 1-2 MB for CPU-bound, 4-8 MB for disk-bound
* **Build Flags**:

```bash
RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=fat" cargo build --release
```

---

## 📝 Technical Details

* **Algorithm**: Base-N conversion, odometer pattern
* **Optimizations**:

  * Loop unrolling (lengths 1-8)
  * Batched atomic operations
  * Large buffers for cache efficiency
  * Thread-safe resume
* **Complexity**:

  * Time: O(charset_len ^ length)
  * Space: O(batch_size + threads*buffer)
  * Disk: O(charset_len ^ length * (length + 1))

---

## 📈 Scalability

| Combos   | Time (16 cores) | Recommended Version |
| -------- | --------------- | ------------------- |
| < 1M     | Seconds         | Any                 |
| 1M-100M  | Minutes         | Optimized+          |
| 100M-10B | Hours           | Ultra-Fast          |
| > 10B    | Days            | Ultra-Fast + Resume |

---

## 🤝 Contributing

Areas for improvement:

* SIMD intrinsics
* GPU acceleration
* Distributed generation
* Custom allocators
* Additional output formats

---

## 🚀 Quick Reference

```bash
# Simple
cargo run --release -- 6 --limit 100000

# Maximum performance
RUSTFLAGS="-C target-cpu=native" cargo build --release
./target/release/combo_gen 8 --threads $(nproc)

# Resume large job
cargo run --release -- 10 --resume state.txt --output big.txt

# Benchmark system
./benchmark.sh
```

---

**Made with ❤️ and Rust** | **Optimized for Speed** | **Production Ready**

