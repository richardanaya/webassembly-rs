build:
	@cargo clippy
	@cargo build --target wasm32-unknown-unknown
	@cargo test
lint:
	@cargo fmt
serve:
	@python3 -m http.server 8080
