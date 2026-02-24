# webassembly-rs

A Rust library provide opcode values and conversions of foundation types (i32,f64,etc.) used at the bytecode level in WebAssembly. This is mostly meant to be used by compilers, interpreters, parsers, encoders. For a more powerful library, checkout [watson](http://github.com/richardanaya/watson).

- [x] depends only on `#![no_std]` and `alloc` 
- [x] includes all opcodes
- [x] full parser and encoder
- [x] tested against real WebAssembly binaries

## Usage

```toml
[dependencies]
webassembly = "0.8"
```

## Testing with Official Spec

```bash
# Quick test (15 unit tests)
cargo test

# Full test with spec suite (17 tests total)  
cargo test -- --ignored

# Or use make targets
make test-all
```

### Test Results

✓ **15 unit tests** - LEB128, section parsing, round-trips, real wasm files
✓ **2 spec integration tests** - Parses 7 official test binaries:
  - sample.wasm (from WebAssembly/spec repo)
  - minimal, type_only, import_only, memory_only, export_only, custom_only

### Setup (optional - for updating spec tests)

```bash
# Already set up! But to update to latest spec tests:
git submodule update --remote tests/spec-tests
./generate-test-wasm.py  # Regenerate test files
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `webassembly` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
