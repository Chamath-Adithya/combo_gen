# ComboGen 🚀 - Corrected Guide & Cheatsheet (Aligned with Cargo.toml)

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

* `n` ✅ Fixed (stable)
* `pro` ⚡ Optimized (balanced)
* `max` 🚀 Ultra-Fast (maximum speed)
* `combo_gen` 🧩 Unified entry point

---

## 🚀 Quick Start

### Basic Usage

```bash
# Generate 100k combinations of length 8 with ultra-fast
cargo run --bin max --release -- 8 --limit 100000

# Generate all combinations of length 4 using 8 threads
cargo run --bin pro --release -- 4 --threads 8

# Custom charset
cargo run --bin n --release -- 5 --charset "abc123" --output custom.txt
```

### Advanced Usage

```bash
# Resume interrupted generation
cargo run --bin max --release -- 8 --resume resume.txt --limit 500000

# Memory-only mode (no file output)
cargo run --bin pro --release -- 4 --memory --verbose

# With gzip compression
cargo run --bin max --release -- 8 --limit 100000 --compress gzip --output archive.gz

# Dry-run (test speed without writing)
cargo run --bin pro --release -- 6 --dry-run --verbose
```

---

## 📖 Command Line Options

```
cargo run --bin <version> --release -- <length> [OPTIONS]
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

### 1️⃣ Small-Scale Generation

```bash
cargo run --bin n --release -- 3 --charset "abc"
```

* Total combinations: 27
* Useful for educational demos

### 2️⃣ Password Generation Simulation

```bash
cargo run --bin max --release -- 8 --limit 100000 --threads 8 --output passwords.txt
```

* Benchmarks: Fixed (n) ~42s, Optimized (pro) ~16s, Ultra/Max (max) ~11s

### 3️⃣ Test Data Creation (QA)

```bash
cargo run --bin pro --release -- 5 --charset "0123456789" --limit 1000000 --dry-run
```

### 4️⃣ Resuming Interrupted Jobs

```bash
cargo run --bin max --release -- 6 --limit 500000 --resume resume.txt
```

### 5️⃣ Memory-Only Mode

```bash
cargo run --bin pro --release -- 4 --memory --verbose
```

### 6️⃣ Compressed Output

```bash
cargo run --bin max --release -- 8 --limit 100000 --compress gzip --output archive.gz
```

* Saves 70-90% disk space

### 7️⃣ Dry-Run for Benchmarking

```bash
cargo run --bin max --release -- 6 --limit 1000000 --dry-run --threads 16
```

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
| 100M-10B | Hours           | Max (max)           |
| > 10B    | Days            | Max + Resume        |

---

## 🤝 Contributing

* SIMD intrinsics
* GPU acceleration
* Distributed generation
* Custom allocators
* Additional output formats

---

## 🚀 Quick Reference

```bash
# Simple generation
cargo run --bin max --release -- 6 --limit 100000

# Maximum performance
RUSTFLAGS="-C target-cpu=native" cargo build --release
./target/release/max 8 --threads $(nproc)

# Resume large job
cargo run --bin max --release -- 10 --resume state.txt --output big.txt

# Benchmark system
./benchmark.sh
```

---

**Made with ❤️ and Rust** | **Optimized for Speed** | **Production Ready**

