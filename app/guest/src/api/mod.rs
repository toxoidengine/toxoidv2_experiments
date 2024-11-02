#![allow(warnings)]

#[cfg(not(target_arch = "wasm32"))]
use toxoid_engine::{Component as ToxoidComponent, ComponentType as ToxoidComponentType, Entity as ToxoidEntity, Query as ToxoidQuery, bindings::exports::toxoid::engine::ecs::{ComponentDesc, EntityDesc, GuestComponent, GuestComponentType, GuestEntity, GuestQuery, MemberType, QueryDesc}};
#[cfg(target_arch = "wasm32")]
use crate::bindings::{toxoid_component::component::ecs::{Component as ToxoidComponent, ComponentType as ToxoidComponentType, Entity as ToxoidEntity, Query as ToxoidQuery, ComponentDesc, EntityDesc, MemberType, QueryDesc}, Guest};



pub type ecs_entity_t = u64;

pub trait ComponentType {
    // fn register() -> ecs_entity_t;
    fn get_name() -> &'static str;
    fn get_id() -> ecs_entity_t;
    // fn get_hash() -> u64;
}

pub trait Component {
    // fn get_id(&self) -> u64;
    // #[cfg(all(target_arch="wasm32", target_os="unknown"))]
    fn set_component(&mut self, component: crate::bindings::toxoid_component::component::ecs::Component);
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
        #[cfg(not(target_arch = "wasm32"))]
        let entity = ToxoidEntity::new(desc);
        #[cfg(target_arch = "wasm32")]
        let entity = ToxoidEntity::new(&desc);
        Self { entity }
    }

    pub fn get_id(&self) -> u64 {
        self.entity.get_id()
    }

    pub fn get<T: Component + ComponentType + Default + 'static>(&self) -> T {
        let mut component= T::default();
        #[cfg(not(target_arch = "wasm32"))]
        let component_ptr = self.entity.get(T::get_id());
        #[cfg(target_arch = "wasm32")]
        let component_ptr = self.entity.get(T::get_id());
        #[cfg(not(target_arch = "wasm32"))]
        let toxoid_component = crate::bindings::toxoid_component::component::ecs::Component::new(component_ptr); 
        #[cfg(target_arch = "wasm32")]
        let toxoid_component = component_ptr;
        component.set_component(toxoid_component);
        component
    }

    pub fn add<T: Component + ComponentType + 'static>(&mut self) {
        let component_id = T::get_id();
        self.entity.add(component_id);
    }

    pub fn remove<T: Component + ComponentType + 'static>(&mut self) {
        // let type_hash = T::get_hash();
        // let component_id_split = toxoid_component_cache_get(type_hash);
        // let component_id = combine_u32(component_id_split);
        // toxoid_entity_remove_component(self.entity.id, component_id);
    }
}

pub struct Query {
    query: ToxoidQuery
}

impl Query {
    pub fn new(desc: Option<QueryDesc>) -> Self {
        let desc = desc.unwrap_or(QueryDesc { expr: "".to_string() });
        #[cfg(not(target_arch = "wasm32"))]
        let query = ToxoidQuery::new(desc);
        #[cfg(target_arch = "wasm32")]
        let query = ToxoidQuery::new(&desc);
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

/* Native
    let component = ComponentType::new(ComponentDesc {
        name: "Position".to_string(),
        member_names: vec!["x".to_string(), "y".to_string()],
        member_types: vec![MemberType::U32T as u8, MemberType::U32T as u8],
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
    query.iter();
    query.next();
    // toxoid_engine::toxoid_progress(1.0);
    let count = query.count();
    println!("{:?}", count); 
*/

/* WASM32
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
*/

