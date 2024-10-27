use toxoid_wasm_runtime::load_wasm_component;

fn main() {
    load_wasm_component("toxoid_wasm_component.wasm").unwrap();
}
