# ComboGen Cheat Sheet 🚀

## Quick Start

```bash
# Generate 1000 combinations of length 4 (Ultra-Fast by default)
cargo run --release -- 4 --limit 1000

# Use specific version
cargo run --release -- --version ultra 4 --limit 1000     # Ultra-Fast (3-5x faster)
cargo run --release -- --version optimized 4 --limit 1000 # Optimized (2-3x faster)
cargo run --release -- --version fixed 4 --limit 1000     # Fixed (stable baseline)
```

## Command Line Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `<length>` | Combination length (required) | - | `4` |
| `--threads N` | Number of CPU threads | Auto-detected | `--threads 8` |
| `--limit N` | Stop after N combinations | All | `--limit 1000000` |
| `--output path` | Output file path | `combos.txt` | `--output passwords.txt` |
| `--charset custom` | Custom character set | ASCII 33-126 | `--charset "abc123"` |
| `--batch N` | Buffer size in bytes | 2MB (Ultra), 1MB (Opt), 64KB (Fixed) | `--batch 1048576` |
| `--resume path` | Resume from file | None | `--resume state.txt` |
| `--compress gzip` | Enable gzip compression | Off | `--compress gzip` |
| `--memory` | Store in memory only | Off | `--memory` |
| `--verbose` | Show detailed progress | Off | `--verbose` |
| `--dry-run` | Test without writing | Off | `--dry-run` |
| `--version V` | Select version: ultra/optimized/fixed | ultra | `--version optimized` |

## Performance Tuning

### For Maximum Speed (Ultra-Fast)
```bash
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release
./target/release/combo_gen 8 --threads $(nproc) --batch 4194304
```

### For Large Generations
```bash
cargo run --release -- 10 --resume state.txt --compress gzip --threads 16
```

### For Testing
```bash
cargo run --release -- 4 --limit 1000 --dry-run --verbose
```

## Character Sets

| Type | Characters | Example Command |
|------|------------|----------------|
| **ASCII** | 33-126 (94 chars) | Default |
| **Numeric** | 0-9 (10 chars) | `--charset "0123456789"` |
| **Alpha** | a-zA-Z (52 chars) | `--charset "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"` |
| **Alphanumeric** | a-zA-Z0-9 (62 chars) | `--charset "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"` |
| **Hex** | 0-9a-f (16 chars) | `--charset "0123456789abcdef"` |
| **Custom** | Any string | `--charset "!@#$%^&*()"` |

## Examples

### Example 1: PIN Generation
```bash
# Generate all 4-digit PINs (0000-9999)
cargo run --release -- 4 --charset "0123456789" --output pins.txt
# Result: 10,000 combinations
```

### Example 2: Password Wordlist
```bash
# Generate 1M lowercase passwords of length 6
cargo run --release -- 6 --charset "abcdefghijklmnopqrstuvwxyz" --limit 1000000 --output wordlist.txt
```

### Example 3: Large Generation with Resume
```bash
# Generate massive wordlist with resume capability
cargo run --release -- 8 --threads 16 --resume progress.txt --compress gzip --output combos.txt.gz
```

### Example 4: Memory Testing
```bash
# Test performance without disk I/O
cargo run --release -- 5 --limit 100000 --memory --verbose
```

### Example 5: Custom Charset
```bash
# Generate combinations from specific characters
cargo run --release -- 4 --charset "!@#$%^&*()" --output symbols.txt
```

## Version Comparison

| Version | Speed | Buffer | Best For |
|---------|-------|--------|----------|
| **Ultra-Fast** | 3-5x | 2MB | Large jobs, maximum performance |
| **Optimized** | 2-3x | 1MB | Balanced performance/features |
| **Fixed** | 1x | 64KB | Stability, debugging, learning |

## Performance Benchmarks

### Test System: AMD Ryzen 9 (16 cores)

| Length | Charset | Combinations | Ultra-Fast | Optimized | Fixed |
|--------|---------|--------------|------------|-----------|-------|
| 4 | ASCII | 88M | 12s | 18s | 45s |
| 5 | ASCII | 6.9B | 14min | 22min | 55min |
| 6 | Numeric | 1M | 2s | 3s | 8s |

## Troubleshooting

### Issue: "length must be integer"
**Cause:** Version flag being passed to combo generator
**Solution:** Use `--` separator: `cargo run --release -- --version optimized 4`

### Issue: Out of memory
**Solutions:**
```bash
# Reduce batch size
--batch 524288

# Use fewer threads
--threads 4

# Enable compression
--compress gzip
```

### Issue: Slow performance
**Solutions:**
```bash
# Use Ultra-Fast version
--version ultra

# Increase threads
--threads $(nproc)

# Larger batch size
--batch 4194304

# Native CPU optimization
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

### Issue: Large output files
**Solutions:**
```bash
# Enable compression
--compress gzip

# Use external storage
--output /mnt/external/combos.txt.gz
```

## File Structure

```
combo_gen_fast/
├── src/
│   ├── main.rs              # Version selector
│   ├── combo_gen_ultra.rs   # Ultra-Fast version
│   ├── combo_gen_optimized.rs # Optimized version
│   └── combo_gen_fixed.rs   # Fixed version
├── Cargo.toml               # Dependencies
├── .gitignore              # Git ignore rules
├── CheatSheet.md           # This file
└── README.md               # Full documentation
```

## Build & Run

```bash
# Development build
cargo build

# Release build (recommended)
cargo build --release

# Maximum performance build
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release

# Run with arguments
cargo run --release -- 4 --limit 1000

# Run binary directly
./target/release/combo_gen 4 --limit 1000
```

## Memory Usage

| Mode | Memory Usage | Use Case |
|------|--------------|----------|
| **File Output** | Low (buffer size) | Large generations |
| **Memory Only** | High (all combos) | Small sets (< 1M) |
| **Dry Run** | Minimal | Performance testing |

## Resume Support

```bash
# Start large generation
cargo run --release -- 10 --resume state.txt --output big.txt

# Resume after interruption (Ctrl+C)
cargo run --release -- 10 --resume state.txt --output big.txt

# Check progress
cat state.txt  # Shows last processed index
```

## Compression

```bash
# Enable gzip compression
cargo run --release -- 8 --compress gzip --output combos.txt.gz

# Benefits:
# - 70-90% smaller files
# - 20-40% slower generation
# - Automatic .gz extension
```

## Tips & Tricks

1. **Always use `--release`** for performance
2. **Use `--dry-run`** to test parameters
3. **Enable `--resume`** for long jobs
4. **Use compression** for large outputs
5. **Monitor with `--verbose`** for debugging
6. **Choose Ultra-Fast** for maximum speed
7. **Use `--memory`** only for small sets
8. **Check system resources** during large runs

## Common Workflows

### Quick Test
```bash
cargo run --release -- 4 --limit 100 --dry-run --verbose
```

### Production Run
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
./target/release/combo_gen 8 --threads $(nproc) --compress gzip --resume state.txt
```

### Benchmarking
```bash
time cargo run --release -- 6 --limit 1000000 --dry-run
```

---

**ComboGen** - High-performance combination generator in Rust 🚀
