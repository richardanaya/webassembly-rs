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

## Testing

### Running Tests

```bash
# Run basic unit tests
cargo test

# Run all tests including spec tests (requires setup)
make test-all
```

### Official Spec Test Suite

This library can be tested against the official WebAssembly spec test suite:

```bash
# 1. Add the spec repo as a git submodule
git submodule add https://github.com/WebAssembly/spec.git tests/spec-tests
git commit -m "Add WebAssembly spec tests"

# 2. Convert .wast files to .wasm binaries (requires wabt)
make test-spec-setup

# Or manually:
./build-spec-tests.sh

# 3. Run spec tests
cargo test test_spec -- --ignored
```

### Updating Spec Tests

```bash
# Update to latest spec tests
git submodule update --remote tests/spec-tests
git add tests/spec-tests
git commit -m "Update spec tests"
```

### CI Integration

For CI environments, tests automatically skip spec tests if not available:

```bash
make test-ci
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
