pub mod bindings;
pub mod api;

pub struct ToxoidWasmComponent;

impl bindings::Guest for ToxoidWasmComponent {
    fn init() -> u64 {
        let entity = api::Entity::new(None);
        let id = entity.get_id();
        id
    }
}

#[cfg(target_arch = "wasm32")]
bindings::export!(ToxoidWasmComponent with_types_in bindings);