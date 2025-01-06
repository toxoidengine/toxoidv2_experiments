#![allow(warnings)]
pub mod components;
pub use components::*;

// Native
#[cfg(not(target_arch = "wasm32"))]
pub use toxoid_host::{
    Component as ToxoidComponent,
    ComponentType as ToxoidComponentType,
    Entity as ToxoidEntity,
    Query as ToxoidQuery,
    System as ToxoidSystem,
    Callback as ToxoidCallback,
    Iter as ToxoidIter,
    bindings::exports::toxoid::engine::ecs::{
        GuestComponent,
        GuestComponentType,
        GuestEntity,
        GuestQuery,
        GuestSystem,
        GuestCallback,
        GuestIter,
    },
    ToxoidApi
};
#[cfg(not(target_arch = "wasm32"))]
pub use toxoid_host::bindings::exports::toxoid::engine::ecs::{
    EntityDesc,
    ComponentDesc,
    QueryDesc,
    SystemDesc,
    MemberType,
    Guest as WorldGuest,
};
// WASM
#[cfg(target_arch = "wasm32")]
pub use toxoid_guest::bindings::{
    toxoid_component::component::ecs::{
        Component as ToxoidComponent,
        ComponentType as ToxoidComponentType,
        Entity as ToxoidEntity,
        Query as ToxoidQuery,
        System as ToxoidSystem,
        Callback as ToxoidCallback,
        Iter as ToxoidIter,
        EntityDesc,
        ComponentDesc,
        QueryDesc,
        SystemDesc,
        MemberType,
        self as ToxoidApi
    },
    self,
    exports::toxoid_component::component::callbacks::Guest as CallbacksGuest,
    Guest as WorldGuest,
};
#[cfg(target_arch = "wasm32")]
pub use toxoid_guest;
// Both (Native + WASM)
pub use toxoid_api_macro::component;

pub type ecs_entity_t = u64;

pub struct ToxoidWasmComponent;

pub fn run_callback(iter: ToxoidIter, handle: i64) {
    let iter = Iter::new(iter);
    // Print all keys and values in the CALLBACKS vector
    for (index, callback) in unsafe { CALLBACKS.iter().enumerate() } {
        println!("Index: {}", index);
    }
    let callback = unsafe { CALLBACKS[handle as usize].as_ref() };
    callback(&iter);
}

// TODO: Create a WIT global function to get the component id directly instead of creating a component type
// with the same name and fields as an existing component type, which is a lot of overhead
#[cfg(target_arch = "wasm32")]
pub fn get_component_id(component_name: &str, member_names: Vec<String>, member_types: Vec<u8>) -> ecs_entity_t {
    let component_type = ToxoidComponentType::new(&ComponentDesc {
        name: component_name.to_string(),
        member_names: member_names,
        member_types: member_types,
    });
    component_type.get_id()
}

// TODO: Create a WIT global function to get the component id directly instead of creating a component type
// with the same name and fields as an existing component type, which is a lot of overhead
#[cfg(not(target_arch = "wasm32"))]
pub fn get_component_id(component_name: &str, member_names: Vec<String>, member_types: Vec<u8>) -> ecs_entity_t {
    // TODO: Instead of initializing a toxoid component type
    // we should have a conditional to check that it already exists.
    // This is already happening under the hood in Flecs, but we don't need to pass 
    // all these parameters in and instantiate a new component type.
    // Haven't checked the overhead but we definitely want to come back and optimize 
    // these unnecessary operations.
    let component_type = ToxoidComponentType::new(ComponentDesc {
        name: component_name.to_string(),
        member_names: member_names,
        member_types: member_types,
    });
    component_type.get_id()
}

pub trait ComponentType {
    fn get_name() -> &'static str;
    fn get_id() -> ecs_entity_t;
    fn register() -> ecs_entity_t;
}

pub trait Component {
    #[cfg(target_arch = "wasm32")]
    fn set_component(&mut self, ptr: toxoid_guest::bindings::toxoid_component::component::ecs::Component);
    #[cfg(not(target_arch = "wasm32"))]
    fn set_component(&mut self, ptr: ToxoidComponent);
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

    pub fn get<T: Component + ComponentType + Default + 'static>(&mut self) -> T {
        let mut component = T::default();
        let component_ptr = self.entity.get(T::get_id());
        #[cfg(not(target_arch = "wasm32"))]
        let toxoid_component = ToxoidComponent::new(component_ptr);
        #[cfg(target_arch = "wasm32")]
        let toxoid_component = component_ptr;
        component.set_component(toxoid_component);
        component
    }

    pub fn add<T: Component + ComponentType + 'static>(&mut self) -> &Self {
        let component_id = T::get_id();
        #[cfg(not(target_arch = "wasm32"))]
        self.entity.add(component_id);
        #[cfg(target_arch = "wasm32")]
        self.entity.add(component_id);
        self
    }

    pub fn remove<T: Component + ComponentType + 'static>(&mut self) {  
        self.entity.remove(T::get_id());
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
        self
            .query
            .entities()
            .iter()
            .map(|entity| Entity { 
                entity: ToxoidEntity::from_id(entity.get_id()) 
            })
            .collect()
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
        let desc = desc.unwrap_or(SystemDesc { 
            name: None, 
            callback, 
            query_desc, 
            is_guest: true, 
            tick_rate: None
        });
        Self { system: ToxoidSystem::new(desc) }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(desc: Option<SystemDesc>, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        // Create the Toxoid callback with the registered callback handle
        let callback = ToxoidCallback::new(callback.cb_handle());
        let query_desc = desc.as_ref().unwrap().query_desc.clone();
        let desc = desc.unwrap_or(SystemDesc { 
            name: None, 
            callback: callback.cb_handle(), 
            query_desc, 
            is_guest: false, 
            tick_rate: None 
        });
        Self { system: ToxoidSystem::new(desc) }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn dsl(dsl: &str, Stick_rate: Option<i32>, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        // Create the Toxoid callback with the registered callback handle
        let callback = ToxoidCallback::new(callback.cb_handle());
        let desc = SystemDesc { 
            name: None, 
            callback, 
            tick_rate: None, 
            is_guest: true, 
            query_desc: QueryDesc { expr: dsl.to_string() } 
        };
        Self::new(Some(desc), callback_fn)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn dsl(dsl: &str, tick_rate: Option<i32>, callback_fn: fn(&Iter)) -> Self {
        // Register the callback in the guest environment
        let callback = Callback::new(callback_fn);
        let desc = SystemDesc { 
            name: None, 
            callback: callback.cb_handle(), 
            query_desc: QueryDesc { expr: dsl.to_string() }, 
            is_guest: false,
            tick_rate
        };
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
    pub iter: ToxoidIter
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
        self.iter
            .entities()
            .iter()
            .map(|entity| {
                #[cfg(not(target_arch = "wasm32"))]
                // In native mode, we get a u64 ID directly
                return Entity {
                    entity: ToxoidEntity { id: *entity }
                };
                #[cfg(target_arch = "wasm32")]
                // In WASM mode, we're working with the guest component object
                return Entity {
                    entity: ToxoidEntity::from_id(entity.get_id())
                };
            })
            .collect()
    }
}

pub struct World;

impl World {
    pub fn add_singleton<T: Component + ComponentType + 'static>() {
        ToxoidApi::add_singleton(T::get_id())
    }

    pub fn get_singleton<T: Component + ComponentType + Default + 'static>() -> T {
        let mut component = T::default();
        let component_id = T::get_id();
        let component_ptr = ToxoidApi::get_singleton(component_id);
        #[cfg(not(target_arch = "wasm32"))]
        let toxoid_component = ToxoidComponent::new(component_ptr);
        #[cfg(target_arch = "wasm32")]
        let toxoid_component = component_ptr;
        component.set_component(toxoid_component);
        component
    }

    pub fn remove_singleton<T: Component + ComponentType + 'static>() {
        let component_id = T::get_id();
        ToxoidApi::remove_singleton(component_id);
    }

    pub fn get_entity(entity_id: u64) -> Entity {
        #[cfg(not(target_arch = "wasm32"))]
        return Entity { entity: ToxoidEntity { id: entity_id } };
        #[cfg(target_arch = "wasm32")]
        return Entity { entity: ToxoidEntity::from_id(entity_id) };
    }

    pub fn remove_entity(entity_id: u64) {
        ToxoidApi::remove_entity(entity_id);
    }
}
