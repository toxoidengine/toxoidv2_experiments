#[allow(warnings)]
mod bindings;

use bindings::{toxoid_component::component::ecs::{Component, ComponentType, ComponentDesc, Entity, EntityDesc}, Guest};
use bindings::toxoid_component::component::ecs::MemberType;

struct ToxoidWasmComponent;

impl Guest for ToxoidWasmComponent {
    fn init(name: String) -> u64 {
        let component = ComponentType::new(&ComponentDesc {
            name: name.clone(),
            member_names: vec!["x".to_string(), "y".to_string()],
            // member_types: vec![MemberType::U32T as u8, MemberType::U32T as u8],
            member_types: vec![0, 0],
        });
        let entity = Entity::new(&EntityDesc {
            name: Some(format!("Test entity {}", name))
        });
        entity.add_component(component.get_id());
        let component = entity.get_component(component.get_id());
        component.take_handle() as u64
        // component.get_id()
    }
}

bindings::export!(ToxoidWasmComponent with_types_in bindings);