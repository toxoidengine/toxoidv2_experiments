install:
	cargo install cargo-component --locked
	cargo install --locked wasm-tools

build-wit:
	cd crates/toxoid_engine && cargo build && cargo component check
	cd crates/toxoid_wasm_component && cargo build && cargo component check

build:
	cd app/guest && cargo component build
	cp target/wasm32-wasip1/debug/guest.wasm crates/toxoid_wasm_runtime/guest.wasm

run:
	cd app/host && cargo run