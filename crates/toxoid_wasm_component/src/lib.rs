#[allow(warnings)]
pub mod bindings;
use crate::bindings::Guest;
use once_cell::sync::Lazy;

use std::sync::Mutex;

fn default_callback() -> u64 {
    0 // Default behavior
}

pub static mut CALLBACK: Lazy<Mutex<fn() -> u64>> = Lazy::new(|| Mutex::new(default_callback));

pub struct ToxoidWasmComponent;

impl Guest for ToxoidWasmComponent {
    fn init() -> u64 {
        unsafe {
            let callback = CALLBACK.lock().unwrap();
            (*callback)()
        }
    }
}

bindings::export!(ToxoidWasmComponent with_types_in bindings);