use toxoid_wasm_component::bindings::{
    self, 
    exports::toxoid_component::component::callbacks::Guest as CallbacksGuest, 
    Guest as WorldGuest, 
    toxoid_component::component::ecs::Iter
};

pub struct ToxoidWasmComponent;

impl CallbacksGuest for ToxoidWasmComponent {
    fn run(iter: Iter, handle: i64) {
        // Implementation for the run method
        // This is where you handle the callback execution
        println!("ToxoidWasmComponent run with handle: {}", handle);
    }
}

impl WorldGuest for ToxoidWasmComponent {
    fn init() {
        // Implementation for the init method
        println!("ToxoidWasmComponent init");
    }
}

bindings::export!(ToxoidWasmComponent with_types_in bindings);