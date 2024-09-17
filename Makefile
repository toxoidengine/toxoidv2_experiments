install:
	cargo install cargo-component --locked
	cargo install --locked wasm-tools

check:
	cd crates/toxoid_api && cargo component check

build:
	cd crates/toxoid_api && cargo component build
	cp target/wasm32-wasip1/debug/toxoid_api.wasm crates/toxoid_wasm_runtime/toxoid_api.wasm

build-wasi:
	cd crates/toxoid_api && RUSTFLAGS='-L crates/toxoid_flecs/wasi-sdk-24.0-x86_64-windows/share/wasi-sysroot/share/wasm32-wasip1' CXXSTDLIB=c++ CC=/c/Users/troye/dev/toxoid/toxoidv2_experiments/crates/toxoid_flecs/wasi-sdk-24.0-x86_64-windows/bin/clang CXX=/c/Users/troye/dev/toxoid/toxoidv2_experiments/crates/toxoid_flecs/wasi-sdk-24.0-x86_64-windows/bin/clang++ CXXFLAGS="-fno-exceptions" cargo component build --target wasm32-wasip1
	cp target/wasm32-wasip1/debug/toxoid_api.wasm crates/toxoid_wasm_runtime/toxoid_api.wasm

build-component:
	cd crates/toxoid_wasm_component && cargo component build
	cp target/wasm32-wasip1/debug/toxoid_wasm_component.wasm crates/toxoid_wasm_runtime/toxoid_wasm_component.wasm

run:
	cd crates/toxoid_wasm_runtime && cargo run