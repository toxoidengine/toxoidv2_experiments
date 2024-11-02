install:
	cargo install cargo-component --locked
	cargo install --locked wasm-tools

submodule:
	git submodule update --init

submodule-add:
	git submodule add -b docker_iter https://github.com/cimgui/cimgui.git crates/toxoid_sokol/lib/cimgui

	git submodule add -b 4.1 https://github.com/EsotericSoftware/spine-runtimes.git crates/toxoid_sokol/lib/spine-runtimes

	git submodule add https://github.com/floooh/sokol.git crates/toxoid_sokol/lib/sokol && \
	cd crates/toxoid_sokol/lib/sokol && \ 
	git checkout 56e98211a2fbd906d37a1051475e04b22a4b62ee && \
	cd ../../../.. && \
	git add crates/toxoid_sokol/lib/sokol

	git submodule add https://github.com/edubart/sokol_gp.git crates/toxoid_sokol/lib/sokol_gp && \
	cd crates/toxoid_sokol/lib/sokol_gp && \
	git checkout a6ce39f93fb2da2c47b70cdd4d1c0a35c0e756ef && \
	cd ../../../.. && \
	git add crates/toxoid_sokol/lib/sokol_gp

build-wit:
	cd crates/toxoid_engine && cargo build && cargo component check
	cd crates/toxoid_wasm_component && cargo build && cargo component check

build:
	cd app/guest && cargo component build
	cp target/wasm32-wasip1/debug/guest.wasm crates/toxoid_wasm_runtime/guest.wasm

run:
	cd app/host && cargo run

run:
	cargo run --package toxoid_cli -- watch


