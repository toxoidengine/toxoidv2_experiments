#[allow(warnings)]
mod bindings;

use bindings::{toxoid_component::component::ecs::{Component, ComponentDesc}, Guest};

struct ToxoidWasmComponent;

impl Guest for ToxoidWasmComponent {
    fn init(name: String) -> u64 {
        let component = Component::new(&ComponentDesc {
            name: name,
            member_names: vec![],
            member_types: vec![],
        });
        // println!("{:?}", component.get_id());
        component.get_id()
    }
}

bindings::export!(ToxoidWasmComponent with_types_in bindings);