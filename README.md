# webassembly-rs

A Rust library provide opcode values and conversions of foundation types (i32,f64,etc.) used at the bytecode level in WebAssembly. This is mostly meant to be used by compilers, interpreters, parsers, encoders. For a more powerful library, checkout [watson](http://github.com/richardanaya/watson).

- [x] deponds only on `#![no_std]` and `alloc` 
- [x] includes all opcodes

```toml
[dependencies]
webassembly = "0.8"
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
