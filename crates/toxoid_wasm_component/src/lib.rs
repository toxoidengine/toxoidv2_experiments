#[allow(warnings)]
pub mod bindings;
use bindings::{toxoid_component::component::ecs::{Component, ComponentDesc, ComponentType, Entity, EntityDesc, MemberType, Query, QueryDesc}, Guest};

struct ToxoidWasmComponent;

impl Guest for ToxoidWasmComponent {
    fn init(name: String) -> u64 {
        let component = ComponentType::new(&ComponentDesc {
            name: "Position".to_string(),
            member_names: vec!["x".to_string(), "y".to_string()],
            member_types: vec![MemberType::U32T as u8, MemberType::U32T as u8],
        });
        let entity = Entity::new(&EntityDesc {
            name: Some(format!("Test entity {}", name))
        });
        let component_id = component.get_id();
        entity.add(component_id);
        let component = entity.get(component_id);
        component.set_member_u64(0, 777);
        component.get_member_u64(0) as u64;
        let query = Query::new(&QueryDesc { expr: "Position($this)".to_string() });
        query.build();
        query.iter();
        query.next();
        let entities = query.entities();
        let entity = entities.get(0).unwrap();
        let component = entity.get(component_id);
        let value = component.get_member_u64(0) as u64;
        value
    }
}

bindings::export!(ToxoidWasmComponent with_types_in bindings);