// #[cfg(not(target_arch = "wasm32"))]
use toxoid_engine::bindings::exports::toxoid::engine::ecs::{ComponentDesc, EntityDesc, GuestComponent, GuestComponentType, GuestEntity, GuestQuery, MemberType, QueryDesc};
use toxoid_engine::{Component as ToxoidComponent, ComponentType as ToxoidComponentType, Entity as ToxoidEntity, Query as ToxoidQuery};
use std::collections::HashMap;

pub static mut HASH_MAP: HashMap<u64, ToxoidComponent> = HashMap::new();

pub trait ComponentType {
    // fn register() -> ecs_entity_t;
    // fn get_name() -> &'static str;
    // fn get_hash() -> u64;
}

pub trait Component {
    // fn get_id(&self) -> u64;
    // #[cfg(all(target_arch="wasm32", target_os="unknown"))]
    // fn set_ptr(&mut self, ptr: i64);
    // #[cfg(not(all(target_arch="wasm32", target_os="unknown")))]
    // fn set_ptr(&mut self, ptr: *mut c_void);
    // #[cfg(all(target_arch="wasm32", target_os="unknown"))]
    // fn get_ptr(&self) -> i64;
    // #[cfg(not(all(target_arch="wasm32", target_os="unknown")))]
    // fn get_ptr(&self) -> *mut c_void;
    // fn set_singleton(&mut self, singleton: bool);
    // fn get_singleton(&self) -> bool;
}

pub struct Entity {
    entity: ToxoidEntity
}

impl Entity {
    pub fn new(desc: Option<EntityDesc>) -> Self {
        let desc = desc.unwrap_or(EntityDesc { name: None });
        let entity = ToxoidEntity::new(desc);
        Self { entity }
    }

    pub fn get_id(&self) -> u64 {
        self.entity.get_id()
    }

    pub fn get(&self, component_id: u64) -> ToxoidComponent {
        let component = self.entity.get(component_id);
        ToxoidComponent::new(component)
    }

    pub fn add<(&self, component_id: u64) {
        self.entity.add(component_id);
    }

    // pub fn remove_component(&self, component_id: u64) {
    //     self.entity.remove_component(component_id);
    // }
}

fn main() {
    // let component = ComponentType::new(ComponentDesc {
    //     name: "Position".to_string(),
    //     member_names: vec!["x".to_string(), "y".to_string()],
    //     member_types: vec![MemberType::U32T as u8, MemberType::U32T as u8],
    // });
    // println!("{:?}", component.get_id());
    // let entity = Entity::new(EntityDesc {
    //     name: Some("Test entity".to_string())
    // });
    // entity.add(component.get_id());
    // let component = entity.get(component.get_id());
    // let component = Component::new(component);
    // component.set_member_u64(0, 777);
    // let value = component.get_member_u64(0) as u64;
    // println!("{:?}", value);
    // let query = Query::new(QueryDesc { expr: "Position($this)".to_string() });
    // query.build();
    // query.iter();
    // query.next();
    // let count = query.count();
    // println!("{:?}", count);
}
