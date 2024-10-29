#[cfg(not(target_arch = "wasm32"))]
use toxoid_engine::bindings::exports::toxoid::engine::ecs::{ComponentDesc, EntityDesc, GuestComponent, GuestComponentType, GuestEntity, GuestQuery, MemberType, QueryDesc};
#[cfg(not(target_arch = "wasm32"))]
use toxoid_engine::{Component as ToxoidComponent, ComponentType as ToxoidComponentType, Entity as ToxoidEntity, Query as ToxoidQuery};
#[cfg(target_arch = "wasm32")]
use toxoid_wasm_component::bindings::exports::toxoid::component::ecs::{Component, ComponentDesc, ComponentType, Entity, EntityDesc, MemberType, Query, QueryDesc};
#[cfg(target_arch = "wasm32")]
use toxoid_wasm_component::{Component as ToxoidComponent, ComponentType as ToxoidComponentType, Entity as ToxoidEntity, Query as ToxoidQuery};
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[cfg(target_arch = "wasm32")]
pub static mut INIT: Lazy<fn() -> u64> = Lazy::new(|| { || 0 });
pub static mut COMPONENT_CACHE: Lazy<HashMap<u64, ToxoidComponent>> = Lazy::new(|| HashMap::new());

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

    pub fn add<T: Component + ComponentType + 'static>(&mut self) {
        // let type_hash = T::get_hash();
        // let component_id_split = toxoid_component_cache_get(type_hash);
        // let component_id = combine_u32(component_id_split);
        // toxoid_entity_add_component(self.entity.id, component_id);
        // self.entity.add(component_id);
    }

    pub fn remove<T: Component + ComponentType + 'static>(&mut self) {
        // let type_hash = T::get_hash();
        // let component_id_split = toxoid_component_cache_get(type_hash);
        // let component_id = combine_u32(component_id_split);
        // toxoid_entity_remove_component(self.entity.id, component_id);
    }

    // pub fn remove_component(&self, component_id: u64) {
    //     self.entity.remove_component(component_id);
    // }
}

pub struct Query {
    query: ToxoidQuery
}

impl Query {
    pub fn new(desc: Option<QueryDesc>) -> Self {
        let desc = desc.unwrap_or(QueryDesc { expr: "".to_string() });
        let query = ToxoidQuery::new(desc);
        Self { query }
    }

    pub fn build(&mut self) {
        self.query.build();
    }

    pub fn iter(&mut self) {
        self.query.iter();
    }

    pub fn next(&mut self) {
        self.query.next();
    }

    pub fn count(&self) -> i32 {
        self.query.count()
    }

    pub fn entities(&self) -> Vec<Entity> {
        // self.query.entities().iter().map(|entity_id| Entity { entity: EntityToxoidEntity::from_id(entity_id) }).collect()
        unimplemented!()
    }
}


#[cfg(target_arch = "wasm32")]
impl Guest for ToxoidWasmComponent {
    fn init() -> u64 {
        INIT()
    }
}

pub fn init(f: fn() -> u64) {
    #[cfg(not(target_arch = "wasm32"))]
    f();
    #[cfg(target_arch = "wasm32")]
    unsafe {
        INIT = Lazy::new(|| f());
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// pub fn init() -> u64 {
//     let component = ToxoidComponentType::new(&ComponentDesc {
//         name: "Position".to_string(),
//         member_names: vec!["x".to_string(), "y".to_string()],
//         member_types: vec![MemberType::U32T as u8, MemberType::U32T as u8],
//     });
//     let entity = ToxoidEntity::new(&EntityDesc {
//         name: Some("Test Entity".to_string())
//     });
//     let component_id = component.get_id();
//     entity.add(component_id);
//     let component = entity.get(component_id);
//     component.set_member_u64(0, 777);
//     component.get_member_u64(0) as u64;
//     let query = ToxoidQuery::new(&QueryDesc { expr: "Position($this)".to_string() });
//     query.build();
//     query.iter();
//     query.next();
//     let entities = query.entities();
//     let entity = entities.get(0).unwrap();
//     let component = entity.get(component_id);
//     let value = component.get_member_u64(0) as u64;
//     value
// }

