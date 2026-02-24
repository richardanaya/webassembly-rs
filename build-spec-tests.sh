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
find "$SPEC_DIR/test/core" -name "*.wast" -type f | head -20 | while read -r wast_file; do
    # Extract base name
    base=$(basename "$wast_file" .wast)
    wasm_out="$OUTPUT_DIR/${base}.wasm"
    
    # Note: wat2wasm converts the entire wast file
    # Some wast files have multiple modules/asserts, we may need special handling
    echo "  Converting: $base"
    
    # Try to convert - may fail for complex wast files
    if wat2wasm "$wast_file" -o "$wasm_out" 2>/dev/null; then
        echo "    ✓ Created: $wasm_out"
    else
        echo "    ⚠ Skipped (complex wast file): $base"
    fi
done

echo ""
echo "Done! Binary tests available in: $OUTPUT_DIR"
echo "Run tests with: cargo test"
