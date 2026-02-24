//! Spec test loader for integration testing
//!
//! This module requires std and is only available in test mode.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use std::fs;
use std::path::Path;

/// Loads test wasm files from the spec tests directory
///
/// To use this, you need to:
/// 1. Add the spec repo as a git submodule:
///    git submodule add https://github.com/WebAssembly/spec.git tests/spec-tests
///    
/// 2. Or manually clone it:
///    git clone https://github.com/WebAssembly/spec.git tests/spec-tests
///
/// 3. Build .wasm files from .wast:
///    ./build-spec-tests.sh  (requires wat2wasm from wabt)
///
/// 4. Or download pre-built test binaries:
///    curl -L https://github.com/WebAssembly/testsuite/releases/latest/download/testsuite.tar.gz | tar xz -C tests/
pub fn load_spec_tests() -> Vec<(String, Vec<u8>)> {
    let mut tests = Vec::new();

    // Try to load from fixtures directory (manually converted .wasm files)
    if Path::new("tests/fixtures").exists() {
        if let Ok(entries) = fs::read_dir("tests/fixtures") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
                    if let Ok(bytes) = fs::read(&path) {
                        let name = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("unknown")
                            .to_string();
                        tests.push((name, bytes));
                    }
                }
            }
        }
    }

    // Also check for any .wasm files directly in the spec repo
    if Path::new("tests/spec-tests").exists() {
        // Look for pre-built wasm binaries if they exist
        if let Ok(entries) = fs::read_dir("tests/spec-tests/test/core") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
                    if let Ok(bytes) = fs::read(&path) {
                        let name = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("unknown")
                            .to_string();
                        tests.push(("spec_".to_string() + &name, bytes));
                    }
                }
            }
        }
    }

    tests
}

/// Check if spec tests are available
pub fn spec_tests_available() -> bool {
    load_spec_tests().len() > 0
}

/// Load a specific named test
pub fn load_spec_test(name: &str) -> Option<Vec<u8>> {
    load_spec_tests()
        .into_iter()
        .find(|(n, _)| n == name)
        .map(|(_, bytes)| bytes)
}
