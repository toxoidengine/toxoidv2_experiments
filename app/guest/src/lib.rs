pub mod bindings;
pub mod api;

use api::Entity;

pub struct ToxoidWasmComponent;

impl bindings::Guest for ToxoidWasmComponent {
    fn init() {
        let entity = Entity::new(None);
        let id = entity.get_id();
        println!("Entity ID: {}", id);
    }
}

#[cfg(target_arch = "wasm32")]
bindings::export!(ToxoidWasmComponent with_types_in bindings);