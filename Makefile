install:
	cargo install cargo-component --locked
	cargo install --locked wasm-tools

check:
	cd crates/toxoid_api && cargo component check

build:
	cd crates/toxoid_api && cargo component build
	cp target/wasm32-wasip1/debug/toxoid_api.wasm crates/toxoid_wasm/toxoid_api.wasm

run:
	cd crates/toxoid_wasm && cargo run