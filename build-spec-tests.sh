#!/bin/bash
# build-spec-tests.sh - Converts WebAssembly spec .wast tests to .wasm binaries

set -e

SPEC_DIR="tests/spec-tests"
OUTPUT_DIR="tests/fixtures"

# Check if wat2wasm is available
if ! command -v wat2wasm &> /dev/null; then
    echo "Error: wat2wasm not found. Install wabt (WebAssembly Binary Toolkit):"
    echo "  Ubuntu/Debian: apt install wabt"
    echo "  Mac: brew install wabt"
    echo "  Or build from: https://github.com/WebAssembly/wabt"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Find and convert .wast files
echo "Converting spec tests to .wasm..."
echo "This may take a while for 486 test files..."
echo ""

SUCCESS=0
FAILED=0
SKIPPED=0
TOTAL=0

# Find all wast files and convert them
find "$SPEC_DIR" -name "*.wast" -type f | while read -r wast_file; do
    TOTAL=$((TOTAL + 1))
    
    # Extract base name (preserve subdirectory structure in name)
    rel_path="${wast_file#$SPEC_DIR/}"
    base=$(echo "$rel_path" | sed 's|/|_|g' | sed 's/.wast$//')
    wasm_out="$OUTPUT_DIR/${base}.wasm"
    
    # Show progress every 50 files
    if [ $((TOTAL % 50)) -eq 0 ]; then
        echo "  Progress: $TOTAL files processed..."
    fi
    
    # Try to convert - may fail for complex wast files with assertions
    if wat2wasm "$wast_file" -o "$wasm_out" 2>/dev/null; then
        SUCCESS=$((SUCCESS + 1))
    else
        # Check if it's a "valid" wast file (simple module) vs test script
        if grep -q "(assert" "$wast_file" 2>/dev/null || \
           grep -q "(invoke" "$wast_file" 2>/dev/null || \
           grep -q "(module" "$wast_file" 2>/dev/null | head -1 | grep -q "\$"; then
            # It's a test script with assertions, skip it
            SKIPPED=$((SKIPPED + 1))
        else
            FAILED=$((FAILED + 1))
        fi
    fi
done

echo ""
echo "=================================="
echo "Conversion Results:"
echo "  Total files: $TOTAL"
echo "  ✓ Successful: $SUCCESS"
echo "  ⚠ Skipped (test scripts): $SKIPPED"
echo "  ✗ Failed: $FAILED"
echo "=================================="
echo ""
echo "Binary tests available in: $OUTPUT_DIR"
echo "Run tests with: cargo test test_spec -- --ignored"
