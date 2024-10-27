// #[cfg(not(target_arch = "wasm32"))]
use toxoid_engine::bindings::exports::toxoid::engine::ecs::{ComponentDesc, EntityDesc, GuestComponent, GuestComponentType, GuestEntity, GuestQuery, QueryDesc};
use toxoid_engine::{Component, ComponentType, Entity, Query};

fn main() {
    // #[cfg(not(target_arch = "wasm32"))]
    // let component = toxoid_engine::Component::new(toxoid_engine::bindings::exports::toxoid::api::ecs::ComponentDesc {
    //     name: "test".to_string(),
    //     member_names: vec!["test".to_string()],
    //     member_types: vec![0],
    // });
    // println!("{:?}", component.get_id());

    let component = ComponentType::new(ComponentDesc {
        name: "Position".to_string(),
        member_names: vec!["x".to_string(), "y".to_string()],
        // member_types: vec![MemberType::U32T as u8, MemberType::U32T as u8],
        member_types: vec![0, 0],
    });
    println!("{:?}", component.get_id());
    let entity = Entity::new(EntityDesc {
        name: Some("Test entity".to_string())
    });
    entity.add_component(component.get_id());
    let component = entity.get_component(component.get_id());
    let component = Component::new(component);
    component.set_member_u64(0, 777);
    let value = component.get_member_u64(0) as u64;
    println!("{:?}", value);
    let query = Query::new(QueryDesc { expr: "Position($this)".to_string() });
    query.build();
    toxoid_engine::toxoid_progress(1.0);
    query.iter();
    query.next();
    let count = query.count();
    println!("{:?}", count);
}
