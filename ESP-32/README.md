# combo_gen
Password tracking tool

# ESP32 / Laptop Combination Generator

This is a high-performance, multi-threaded combination generator written in Rust. It can dynamically generate combinations of a given length from a specified character set without storing them all in memory. This is ideal for testing, benchmarking, or limited brute-force scenarios.

## Features

* Supports all printable ASCII characters by default (`!` to `~`), or a custom charset.
* Generates combinations dynamically (no RAM-heavy storage).
* Multi-threaded generation for maximum CPU utilization.
* Supports output to file or `/dev/null` for benchmarking.
* Optional limit on the number of combinations to generate.
* Cross-platform (Linux tested, should run on Windows/Mac with Rust).

## Requirements

* Rust (latest stable)
* Cargo build tool
* Linux / Windows / Mac environment

Optional dependencies (included in Cargo.toml):

* `num_cpus` for automatic CPU thread detection.
* `parking_lot` for fast mutex synchronization.

## Installation

1. Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
cargo --version
```

2. Clone or create the project:

```bash
cargo new combo_gen --bin
cd combo_gen
```

3. Add dependencies to `Cargo.toml`:

```toml
[dependencies]
num_cpus = "1.17"
parking_lot = "0.12"
```

4. Replace `src/main.rs` with the provided Rust code.

## Build

For optimized performance:

```bash
cargo build --release
```

The executable will be located at `target/release/combo_gen`.

## Usage

```bash
./combo_gen <length> [--threads N] [--limit N] [--output path] [--charset custom]
```

### Examples

* Generate first 1,000 combinations of length 8:

```bash
./target/release/combo_gen 8 --limit 1000
```

* Generate full space (output discarded) to benchmark:

```bash
./target/release/combo_gen 8 --output /dev/null
```

* Use 16 threads and custom output:

```bash
./target/release/combo_gen 8 --threads 16 --limit 10000 --output sample.txt
```

* Use a custom charset:

```bash
./target/release/combo_gen 5 --charset "ABC123!@#"
```

## Notes

* The total number of combinations grows exponentially. For length 8 with the default 94-character set, there are ~6.1 quadrillion combinations. Generating all is impractical on a laptop.
* Use the `--limit` option to generate a subset.
* Writing to terminal is slow; use `/dev/null` or files for high-speed benchmarks.
* Multi-threading uses all available CPU cores by default. Adjust with `--threads` if needed.

## License

MIT License
