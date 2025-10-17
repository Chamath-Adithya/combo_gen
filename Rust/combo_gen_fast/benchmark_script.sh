#!/bin/bash
# ComboGen Performance Benchmark Script

echo "╔════════════════════════════════════════════════════════╗"
echo "║        ComboGen Performance Benchmark Suite            ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if binary exists
if [ ! -f "target/release/combo_gen" ]; then
    echo -e "${RED}Error: Binary not found. Building...${NC}"
    cargo build --release
    if [ $? -ne 0 ]; then
        echo -e "${RED}Build failed!${NC}"
        exit 1
    fi
fi

BINARY="./target/release/combo_gen"
RESULTS_FILE="benchmark_results_$(date +%Y%m%d_%H%M%S).txt"

echo -e "${BLUE}Results will be saved to: ${RESULTS_FILE}${NC}"
echo ""

# Function to run benchmark
run_benchmark() {
    local name=$1
    local length=$2
    local limit=$3
    local extra_args=$4
    
    echo -e "${YELLOW}Running: ${name}${NC}"
    echo "Command: $BINARY $length --limit $limit --dry-run $extra_args"
    
    # Run benchmark and capture time
    START=$(date +%s.%N)
    $BINARY $length --limit $limit --dry-run $extra_args 2>&1 | grep -E "(Throughput|Elapsed|Generated)"
    END=$(date +%s.%N)
    
    DURATION=$(echo "$END - $START" | bc)
    echo -e "${GREEN}Completed in: ${DURATION}s${NC}"
    echo ""
    
    # Save to results file
    echo "=== $name ===" >> $RESULTS_FILE
    echo "Command: $BINARY $length --limit $limit --dry-run $extra_args" >> $RESULTS_FILE
    echo "Duration: ${DURATION}s" >> $RESULTS_FILE
    echo "" >> $RESULTS_FILE
}

# Benchmark Suite
echo "═══════════════════════════════════════════════════════"
echo "Test 1: Small Combinations (Length 4, 100K combos)"
echo "═══════════════════════════════════════════════════════"
run_benchmark "Test 1 - 1 Thread" 4 100000 "--threads 1"
run_benchmark "Test 1 - 4 Threads" 4 100000 "--threads 4"
run_benchmark "Test 1 - All Threads" 4 100000 "--threads $(nproc)"

echo "═══════════════════════════════════════════════════════"
echo "Test 2: Medium Combinations (Length 5, 1M combos)"
echo "═══════════════════════════════════════════════════════"
run_benchmark "Test 2 - Default Batch" 5 1000000 ""
run_benchmark "Test 2 - 1MB Batch" 5 1000000 "--batch 1048576"
run_benchmark "Test 2 - 4MB Batch" 5 1000000 "--batch 4194304"

echo "═══════════════════════════════════════════════════════"
echo "Test 3: Large Combinations (Length 6, 10M combos)"
echo "═══════════════════════════════════════════════════════"
run_benchmark "Test 3 - Optimized" 6 10000000 ""

echo "═══════════════════════════════════════════════════════"
echo "Test 4: Custom Charset (Length 8, 100K combos)"
echo "═══════════════════════════════════════════════════════"
run_benchmark "Test 4 - Numeric" 8 100000 "--charset '0123456789'"
run_benchmark "Test 4 - Alphanumeric" 8 100000 "--charset 'abcdefghijklmnopqrstuvwxyz0123456789'"

echo "═══════════════════════════════════════════════════════"
echo "Test 5: Thread Scaling (Length 5, 5M combos)"
echo "═══════════════════════════════════════════════════════"
for threads in 1 2 4 8 16; do
    if [ $threads -le $(nproc) ]; then
        run_benchmark "Test 5 - ${threads} Threads" 5 5000000 "--threads $threads"
    fi
done

echo ""
echo "╔════════════════════════════════════════════════════════╗"
echo "║              Benchmark Complete!                       ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""
echo -e "${GREEN}Full results saved to: ${RESULTS_FILE}${NC}"
echo ""

# Generate summary
echo "═══════════════════════════════════════════════════════"
echo "Summary (from last run):"
echo "═══════════════════════════════════════════════════════"
cat $RESULTS_FILE | grep -E "(===|Duration)" | head -20

echo ""
echo -e "${BLUE}Tip: Compare with original version to see improvements!${NC}"
echo ""

# System info
echo "System Information:" >> $RESULTS_FILE
echo "==================" >> $RESULTS_FILE
echo "CPU: $(lscpu | grep 'Model name' | cut -d: -f2 | xargs)" >> $RESULTS_FILE
echo "Cores: $(nproc)" >> $RESULTS_FILE
echo "RAM: $(free -h | grep Mem | awk '{print $2}')" >> $RESULTS_FILE
echo "Rust: $(rustc --version)" >> $RESULTS_FILE
echo "Build flags: ${RUSTFLAGS:-none}" >> $RESULTS_FILE
