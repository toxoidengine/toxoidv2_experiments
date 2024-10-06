install:
	cargo install cargo-component --locked
	cargo install --locked wasm-tools

check:
	cd crates/toxoid_api && cargo component check