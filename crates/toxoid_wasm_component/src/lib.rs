#[allow(warnings)]
mod bindings;

use bindings::{toxoid_component::component::ecs::{Component, ComponentDesc}, Guest};
use bindings::toxoid_component::component::ecs::MemberType;

struct ToxoidWasmComponent;

impl Guest for ToxoidWasmComponent {
    fn init(name: String) -> u64 {
        let component = Component::new(&ComponentDesc {
            name: name,
            member_names: vec!["x".to_string(), "y".to_string()],
            // member_types: vec![MemberType::U32T as u8, MemberType::U32T as u8],
            member_types: vec![0, 0],
        });
        component.get_id()
    }
}

bindings::export!(ToxoidWasmComponent with_types_in bindings);