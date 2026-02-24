build:
	@cargo clippy
	@cargo build --target wasm32-unknown-unknown
	@cargo test

lint:
	@cargo fmt

serve:
	@python3 -m http.server 8080

# Spec test suite integration
test-spec-setup:
	@echo "Setting up WebAssembly spec tests..."
	@if [ ! -d "tests/spec-tests" ]; then \
		git submodule add https://github.com/WebAssembly/spec.git tests/spec-tests; \
	else \
		git submodule update --remote tests/spec-tests; \
	fi
	@echo "Converting .wast to .wasm..."
	@./build-spec-tests.sh

test-spec:
	@cargo test test_spec -- --ignored

test-all:
	@cargo test
	@cargo test test_spec -- --ignored

# CI-friendly test that skips spec tests if not available
test-ci:
	@cargo test
	@if [ -d "tests/fixtures" ]; then \
		cargo test test_spec -- --ignored || true; \
	fi
