install:
	cargo install cargo-component --locked
	cargo install --locked wasm-tools

check:
	cd crates/toxoid_wasm_component && cargo component check

build:
	cd crates/toxoid_engine && cargo build && cargo component check
	cd crates/toxoid_wasm_component && cargo build && cargo component build
	cp target/wasm32-wasip1/debug/toxoid_wasm_component.wasm crates/toxoid_wasm_runtime/toxoid_wasm_component.wasm

run:
	cd app/host && cargo run
	