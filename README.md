# combo_gen

A suite of high-performance combination generators for various platforms, designed for generating all possible combinations of a given character set and length. Ideal for testing, benchmarking, password generation simulation, and research purposes.

## Overview

This project provides multiple implementations across different languages and platforms:

- **Rust v1 (ComboGen Optimem)**: Memory-optimized, resumable generator with multi-threading and compression support.
- **Rust v2 (ComboGen)**: Advanced versions with different optimization levels (n, pro, max) for maximum performance.
- **ESP32 Firmware**: Microcontroller-based generator for embedded systems with limited RAM.

## Rust Implementations

### v1: ComboGen Optimem

A high-performance, memory-optimized, resumable combination generator written in Rust.

#### Features

* Multi-threaded generation for maximum speed.
* Optimized memory usage using per-thread buffers (optimem approach).
* Resume support: safely continue after interruption.
* Optional compressed output (gzip).
* Memory-only mode for small sets to avoid I/O overhead.
* Unicode/UTF-8 charset support.
* CLI flags for batch size, verbosity, dry-run, and more.
* Progress bar with ETA and throughput.

#### Build

```bash
cargo build --release
```

#### Usage

```bash
./combo_gen_optimem <length> [options]
```

Options:
- `--threads N`: Number of threads (default: CPU cores)
- `--limit N`: Limit combinations generated
- `--output path`: Output file path (default: combos.txt)
- `--charset <string>`: Custom character set
- `--batch N`: Buffer size per thread in bytes (default: 64 KB)
- `--resume path`: Resume from file
- `--compress gzip`: Compress output
- `--memory`: Memory-only mode
- `--verbose`: Show thread progress
- `--dry-run`: Benchmark without writing

#### Examples

```bash
# Generate all 8-character combos
./combo_gen_optimem 8

# Generate first 1000 combos of length 5
./combo_gen_optimem 5 --limit 1000

# Use 8 threads and save to file
./combo_gen_optimem 6 --threads 8 --output combos.txt

# Custom charset
./combo_gen_optimem 4 --charset "abc123!@"

# Resume generation
./combo_gen_optimem 8 --resume resume.txt --limit 1000000

# Compressed output
./combo_gen_optimem 8 --compress gzip --output combos.gz
```

### v2: ComboGen

A high-performance Rust tool for generating combinations, optimized for speed, flexibility, and scalability.

#### Features

* Multiple optimization levels: n (stable), pro (optimized), max (ultra-fast)
* Base-N conversion algorithm with odometer pattern
* Loop unrolling for lengths 1-8
* Batched atomic operations
* Large buffers for cache efficiency
* Thread-safe resume
* Gzip compression support
* Memory-only mode
* Verbose and dry-run modes

#### Installation & Build

```bash
git clone https://github.com/chamath-adithya/combo_gen.git
cd combo_gen/Rust/v2

# Standard build
cargo build --release

# Ultra-fast build
RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=fat" cargo build --release
```

Binaries: `n`, `pro`, `max`, `combo_gen`

#### Usage

```bash
cargo run --bin <version> --release -- <length> [OPTIONS]
```

Options:
- `--threads N`: Number of threads (default: CPU cores)
- `--limit N`: Stop after N combinations
- `--output path`: Output file path (default: combos.txt)
- `--charset custom`: Custom charset
- `--batch N`: Buffer size in bytes (default: 2 MB)
- `--resume path`: Resume from file
- `--compress gzip`: Enable compression
- `--memory`: Memory-only mode
- `--verbose`: Detailed progress
- `--dry-run`: Generate without writing

#### Examples

```bash
# Basic generation
cargo run --bin max --release -- 8 --limit 100000

# Custom charset
cargo run --bin pro --release -- 5 --charset "abc123" --output custom.txt

# Resume interrupted job
cargo run --bin max --release -- 8 --resume resume.txt --limit 500000

# Compressed output
cargo run --bin max --release -- 8 --compress gzip --output archive.gz
```

## ESP32 Implementation

An optimized ESP32 Arduino firmware that generates all possible combinations of a given character set for a specified code length.

#### Features

* Accepts code length from Serial Monitor (1–10 characters)
* Calculates total possible combinations instantly
* Generates combinations on-the-fly without RAM storage
* Streams combinations to Serial Monitor
* Safe for ESP32's limited RAM
* Optional limit for testing large lengths

#### Supported Character Set

94 printable ASCII characters by default (customizable).

#### Hardware Requirements

* ESP32 Development Board
* USB cable
* Arduino IDE or PlatformIO

#### Installation & Setup

1. Install Arduino IDE
2. Select ESP32 board
3. Connect ESP32 via USB
4. Upload the firmware from `ESP-32/combo_gen/combo_gen.ino`

#### Usage

1. Open Serial Monitor (115200 baud)
2. Enter code length when prompted
3. View combinations streaming

#### Performance Considerations

| Code Length | Total Combinations | Practical |
|-------------|-------------------|-----------|
| 1–5        | 94 – 7M          | Fast     |
| 6–7        | 689M – 64B       | Slow     |
| 8–10       | 6T+              | Impractical |

## Requirements

### Rust Versions
* Rust 1.70+
* Cargo

## License

MIT License
