#!/bin/bash
# convert-all-spec-tests.sh - Converts all WebAssembly spec .wast tests to .wasm binaries

set -e

SPEC_DIR="tests/spec-tests"
OUTPUT_DIR="tests/fixtures"

# Check if wast2json is available
if ! command -v wast2json &> /dev/null; then
    echo "Error: wast2json not found. Install wabt (WebAssembly Binary Toolkit):"
    echo "  Ubuntu/Debian: apt install wabt"
    echo "  Mac: brew install wabt"
    exit 1
fi

echo "Converting all spec tests to .wasm..."
echo "This extracts modules from 486 wast files..."
echo ""

# Clean and create output directory
mkdir -p "$OUTPUT_DIR/spec"
rm -f "$OUTPUT_DIR/spec/"*.wasm

SUCCESS=0
FAILED=0
TOTAL=0

# Find all wast files and convert them
while IFS= read -r wast_file; do
    TOTAL=$((TOTAL + 1))
    
    # Get basename without extension
    base=$(basename "$wast_file" .wast)
    
    # Show progress every 50 files
    if [ $((TOTAL % 50)) -eq 0 ]; then
        echo "  Progress: $TOTAL files processed..."
    fi
    
    # Convert with wast2json - this creates multiple .wasm files per wast
    # Files are named: base.XX.wasm where XX is line number
    if wast2wasm "$wast_file" -o "$OUTPUT_DIR/spec/${base}.json" 2>/dev/null; then
        # wast2wasm sometimes works when wast2json doesn't
        SUCCESS=$((SUCCESS + 1))
    elif wast2json "$wast_file" --disable-saturating-float-to-int --disable-sign-extension --disable-simd --disable-multi-value --disable-mutable-globals --disable-reference-types --disable-bulk-memory --disable-tail-call --disable-nontrapping-float-to-int -o "$OUTPUT_DIR/spec/${base}.json" 2>/dev/null; then
        SUCCESS=$((SUCCESS + 1))
    else
        # Try basic conversion
        if wast2json "$wast_file" -o "$OUTPUT_DIR/spec/${base}.json" 2>/dev/null; then
            SUCCESS=$((SUCCESS + 1))
        else
            FAILED=$((FAILED + 1))
        fi
    fi
done < <(find "$SPEC_DIR" -name "*.wast" -type f)

echo ""
echo "=================================="
echo "Conversion Results:"
echo "  Total wast files: $TOTAL"
echo "  ✓ Successful: $SUCCESS"
echo "  ✗ Failed: $FAILED"
echo "=================================="

# Count generated wasm files
WASM_COUNT=$(find "$OUTPUT_DIR/spec" -name "*.wasm" -type f | wc -l)
echo ""
echo "Generated $WASM_COUNT .wasm files"
echo ""
echo "Run tests with: cargo test test_spec -- --ignored"
