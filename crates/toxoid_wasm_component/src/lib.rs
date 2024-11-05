pub mod bindings;
use bindings::Guest;

pub struct ToxoidWasmComponent;

impl Guest for ToxoidWasmComponent {
    fn init() {
        println!("ToxoidWasmComponent init");
    }
}

#[cfg(target_arch = "wasm32")]
bindings::export!(ToxoidWasmComponent with_types_in bindings);