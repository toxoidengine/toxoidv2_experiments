#![allow(warnings)]
// Native
#[cfg(not(target_arch = "wasm32"))]
use toxoid_engine::{Component as ToxoidComponent, ComponentType as ToxoidComponentType, Entity as ToxoidEntity, Query as ToxoidQuery, System as ToxoidSystem, Callback as ToxoidCallback, Iter as ToxoidIter, bindings::exports::toxoid::engine::ecs::{GuestComponent, GuestComponentType, GuestEntity, GuestQuery, GuestSystem, GuestCallback, GuestIter}};
#[cfg(not(target_arch = "wasm32"))]
pub use toxoid_engine::bindings::exports::toxoid::engine::ecs::{EntityDesc, ComponentDesc, QueryDesc, SystemDesc, MemberType};
// WASM
#[cfg(target_arch = "wasm32")]
use toxoid_wasm_component::bindings::{toxoid_component::component::ecs::{Component as ToxoidComponent, ComponentType as ToxoidComponentType, Entity as ToxoidEntity, Query as ToxoidQuery, System as ToxoidSystem, Callback as ToxoidCallback, Iter as ToxoidIter}, Guest};
#[cfg(target_arch = "wasm32")]
pub use toxoid_wasm_component::bindings::toxoid_component::component::ecs::{EntityDesc, ComponentDesc, QueryDesc, SystemDesc, MemberType};
#[cfg(target_arch = "wasm32")]
pub use toxoid_wasm_component;
// Both (Native + WASM)
pub use toxoid_api_macro::component;

pub struct ToxoidWasmComponent;

#[cfg(target_arch = "wasm32")]
impl toxoid_wasm_component::bindings::exports::toxoid_component::component::callbacks::Guest for ToxoidWasmComponent {
    fn run(iter: toxoid_wasm_component::bindings::toxoid_component::component::ecs::Iter, handle: i64) {
        let iter = Iter::new(iter);
        let callback = unsafe { CALLBACKS[handle as usize].as_ref() };
        callback(&iter);
    }
}

pub fn run_callback(iter: ToxoidIter, handle: i64) {
    let iter = Iter::new(iter);
    let callback = unsafe { CALLBACKS[handle as usize].as_ref() };
    callback(&iter);
}

pub type ecs_entity_t = u64;

pub trait ComponentType {
    fn get_name() -> &'static str;
    fn get_id() -> ecs_entity_t;
    // fn register() -> ecs_entity_t;
    // fn get_hash() -> u64;
}

pub trait Component {
    // fn get_id(&self) -> u64;
    // #[cfg(all(target_arch="wasm32", target_os="unknown"))]
    #[cfg(target_arch = "wasm32")]
    fn set_ptr(&mut self, ptr: *mut toxoid_wasm_component::bindings::toxoid_component::component::ecs::Component);
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

    pub fn named(name: &str) -> Self {
        let desc = EntityDesc { name: Some(name.to_string()) };
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
        let toxoid_component_ptr = Box::into_raw(Box::new(toxoid_component));
        component.set_ptr(toxoid_component_ptr);
        component
    }

    pub fn add<T: Component + ComponentType + 'static>(&mut self) -> &Self {
        let component_id = T::get_id();
        self.entity.add(component_id);
        self
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

    pub fn dsl(dsl: &str) -> Self {
        let desc = QueryDesc { expr: dsl.to_string() };
        Self::new(Some(desc))
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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn entities(&self) -> Vec<Entity> {
        unimplemented!()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn entities(&self) -> Vec<Entity> {
        self.query.entities().iter().map(|entity| Entity { entity: ToxoidEntity::from_id(entity.get_id()) }).collect()
    }
}

pub struct System {
    system: ToxoidSystem
}

impl System {
    // #[cfg(not(target_arch = "wasm32"))]
    // pub fn new(desc: Option<SystemDesc>) -> Self {
    //     unimplemented!()
    // }

    #[cfg(target_arch = "wasm32")]
    pub fn new(desc: Option<SystemDesc>, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        // Create the Toxoid callback with the registered callback handle
        let callback = ToxoidCallback::new(callback.cb_handle());
        let query_desc = desc.as_ref().unwrap().query_desc.clone();
        let desc = desc.unwrap_or(SystemDesc { name: None, callback, query_desc });
        Self { system: ToxoidSystem::new(desc) }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn dsl(dsl: &str, callback: fn(&Iter)) -> Self {
        unimplemented!()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn dsl(dsl: &str, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        // Create the Toxoid callback with the registered callback handle
        let callback = ToxoidCallback::new(callback.cb_handle());
        println!("Callback handle: {}", callback.cb_handle());
        let desc = SystemDesc { name: None, callback, query_desc: QueryDesc { expr: dsl.to_string() } };
        Self::new(Some(desc), callback_fn)
    }

    pub fn build(&mut self) {
        self.system.build();
    }
}

pub struct Callback {
    callback: ToxoidCallback,
}

pub static mut CALLBACKS: once_cell::sync::Lazy<Vec<Box<dyn Fn(&Iter)>>> = once_cell::sync::Lazy::new(|| Vec::new());

impl Callback {
    pub fn new(callback_fn: fn(&Iter)) -> Self {
        let handle = unsafe { CALLBACKS.push(Box::new(callback_fn)); CALLBACKS.len() - 1 };
        Self { callback: ToxoidCallback::new(handle as i64) }   
    }

    pub fn run(&self, iter: &Iter) {
        let callback = unsafe { CALLBACKS[self.callback.cb_handle() as usize].as_ref() };
        callback(iter);
    }

    pub fn cb_handle(&self) -> i64 {
        self.callback.cb_handle()
    }
}

pub struct Iter {
    iter: ToxoidIter
}

impl Iter {
    pub fn new(iter: ToxoidIter) -> Self {
        Self { iter }
    }

    pub fn next(&mut self) {
        self
            .iter
            .next();
    }

    pub fn count(&self) -> i32 {
        self
            .iter
            .count()
    }

    pub fn entities(&self) -> Vec<Entity> {
        self
            .iter
            .entities()
            .iter()
            .map(|entity| Entity { 
                entity: ToxoidEntity::from_id(entity.get_id()) 
            })
            .collect()
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

