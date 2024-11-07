bindgen!({
    world: "toxoid-component-world",
    path: "../toxoid_wasm_component/wit",
    with: {
        // Specify that our host resource is going to point to the `ComponentProxy`
        "toxoid-component:component/ecs/component-type": ComponentTypeProxy,
        "toxoid-component:component/ecs/component": ComponentProxy,
        "toxoid-component:component/ecs/entity": EntityProxy,
        "toxoid-component:component/ecs/query": QueryProxy,
        "toxoid-component:component/ecs/system": SystemProxy,
        "toxoid-component:component/ecs/callback": CallbackProxy,
        "toxoid-component:component/ecs/iter": IterProxy,
    },
});

use toxoid_engine::bindings::exports::toxoid::engine::ecs::{GuestCallback, GuestComponent, GuestComponentType, GuestEntity, GuestIter, GuestQuery, GuestSystem};
use wasmtime::component::{bindgen, Component, Linker, Resource, ResourceTable};
use wasmtime::{Engine, Result, Store};
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder};
use once_cell::sync::Lazy;

pub struct ComponentTypeProxy {
    ptr: *mut toxoid_engine::ComponentType
}
unsafe impl Send for ComponentTypeProxy {}
pub struct ComponentProxy {
    ptr: *mut toxoid_engine::Component
}
unsafe impl Send for ComponentProxy {}
pub struct EntityProxy {
    ptr: *mut toxoid_engine::Entity
}
unsafe impl Send for EntityProxy {}
pub struct QueryProxy {
    ptr: *mut toxoid_engine::Query
}
unsafe impl Send for QueryProxy {}
pub struct SystemProxy {
    ptr: *mut toxoid_engine::System
}
unsafe impl Send for SystemProxy {}
pub struct CallbackProxy {
    ptr: *mut toxoid_engine::Callback
}
unsafe impl Send for CallbackProxy {}
pub struct IterProxy {
    ptr: *mut toxoid_engine::Iter
}
unsafe impl Send for IterProxy {}
// StoreState is the state of the WASM store.
pub struct StoreState {
    ctx: WasiCtx,
    table: ResourceTable,
}

// A trait which provides access to internal WASI state.
// For a Store<T> this trait will be implemented for the T. This also corresponds to the T in Linker<T>.
impl WasiView for StoreState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}

impl toxoid_component::component::ecs::Host for StoreState {}

impl toxoid_component::component::ecs::HostIter for StoreState {
    fn new(&mut self, ptr: i64) -> Resource<IterProxy> {
        self.table.push::<IterProxy>(IterProxy { ptr: ptr as *mut toxoid_engine::Iter }).unwrap()
    }   

    fn next(&mut self, iter: Resource<IterProxy>) -> bool {
        let iter_proxy = self.table.get(&iter).unwrap() as &IterProxy;
        let iter = unsafe { Box::from_raw(iter_proxy.ptr) };
        iter.next()
    }

    fn count(&mut self, iter: Resource<IterProxy>) -> i32 {
        let iter_proxy = self.table.get(&iter).unwrap() as &IterProxy;
        let iter = unsafe { Box::from_raw(iter_proxy.ptr) };
        iter.count()
    }

    fn entities(&mut self, iter: Resource<IterProxy>) -> Vec<Resource<EntityProxy>>{
        let iter_proxy = self.table.get(&iter).unwrap() as &IterProxy;
        let iter = unsafe { Box::from_raw(iter_proxy.ptr) };
        let entity_ids = iter.entities();
        let ids = entity_ids.iter().map(|entity_id| {
            // Create entity
            let entity = toxoid_engine::Entity::from_id(*entity_id) as *mut toxoid_engine::Entity;

            // Push component to resource table
            let id = self
                .table
                .push::<EntityProxy>(EntityProxy {
                    ptr: entity
                })
                .expect("Failed to push component to table");
            id
        }).collect();
        Box::into_raw(iter);
        ids
    }

    fn drop(&mut self, _iter: Resource<IterProxy>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostCallback for StoreState {
    fn new(&mut self, handle: i64) -> Resource<CallbackProxy> {
        let callback = <toxoid_engine::Callback as toxoid_engine::bindings::exports::toxoid::engine::ecs::GuestCallback>::new(handle);
        let boxed_callback = Box::new(callback);
        let boxed_callback_ptr = Box::into_raw(boxed_callback);
        self.table.push::<CallbackProxy>(CallbackProxy { ptr: boxed_callback_ptr }).unwrap()
    }

    fn run(&mut self, _callback: wasmtime::component::Resource<CallbackProxy>, iter: wasmtime::component::Resource<IterProxy>) -> () {
        let callback_proxy = self.table.get(&_callback).unwrap() as &CallbackProxy;
        let callback = unsafe { Box::from_raw(callback_proxy.ptr) };
        let callbacks = unsafe { TOXOID_COMPONENT_WORLD.as_mut().unwrap() };
        let store = unsafe { &mut *STORE };
        callbacks.interface1.call_run(store, iter, callback.handle).unwrap();
    }

    fn cb_handle(&mut self, _callback: Resource<toxoid_component::component::ecs::Callback>) -> i64 {
        let callback_proxy = self.table.get(&_callback).unwrap() as &CallbackProxy;
        let callback = unsafe { Box::from_raw(callback_proxy.ptr) };
        callback.handle
    }

    fn drop(&mut self, _callback: Resource<toxoid_component::component::ecs::Callback>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostSystem for StoreState {
    fn new(&mut self, desc: toxoid_component::component::ecs::SystemDesc) -> Resource<SystemProxy> {
        let callback_proxy = self.table.get(&desc.callback).unwrap() as &CallbackProxy;
        let callback = unsafe { Box::from_raw(callback_proxy.ptr) };
        println!("Callback rep: {}", desc.callback.rep());
        println!("Callback handle: {}", callback.handle);
        let query_desc = toxoid_engine::bindings::exports::toxoid::engine::ecs::QueryDesc {
            expr: desc.query_desc.expr,
        };
        let query = toxoid_engine::Query::new(query_desc.clone());
        let query_ptr = Box::into_raw(Box::new(query));
        let system = <toxoid_engine::System as toxoid_engine::bindings::exports::toxoid::engine::ecs::GuestSystem>::new(toxoid_engine::bindings::exports::toxoid::engine::ecs::SystemDesc {
            name: desc.name,
            query_desc,
            query: query_ptr as i64,
            callback: callback.cb_handle(),
        });
        let id = self
            .table
            .push::<SystemProxy>(SystemProxy {
                ptr: Box::into_raw(Box::new(system))
            })
            .unwrap();
        Box::into_raw(callback);
        id
    }
    
    fn callback(&mut self, system: Resource<toxoid_component::component::ecs::System>) -> Resource<CallbackProxy> {
        let system_proxy = self.table.get(&system).unwrap() as &SystemProxy;
        let system = unsafe { Box::from_raw(system_proxy.ptr) };
        let callback_handle = system.callback();
        let callback = <toxoid_engine::Callback as toxoid_engine::bindings::exports::toxoid::engine::ecs::GuestCallback>::new(callback_handle);
        self.table.push::<CallbackProxy>(CallbackProxy { ptr: Box::into_raw(Box::new(callback)) }).unwrap()
    }

    fn build(&mut self, system: Resource<toxoid_component::component::ecs::System>) -> () {
        let system_proxy = self.table.get(&system).unwrap() as &SystemProxy;
        let mut system = unsafe { Box::from_raw(system_proxy.ptr) };
        system.as_mut().build();
        Box::into_raw(system);
    }

    fn drop(&mut self, _system: Resource<toxoid_component::component::ecs::System>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostEntity for StoreState {
    fn new(&mut self, desc: toxoid_component::component::ecs::EntityDesc) -> Resource<EntityProxy> {
        let entity = toxoid_engine::Entity::new(toxoid_engine::bindings::exports::toxoid::engine::ecs::EntityDesc {
            name: desc.name,
        });
        // Create boxed component
        let boxed_entity = Box::new(entity);
        let box_ptr = Box::into_raw(boxed_entity);
        // Push entity to resource table
        let id = self
            .table
            .push::<EntityProxy>(EntityProxy {
                ptr: box_ptr
            })
            .unwrap();
        id
    }

    fn get_id(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>) -> u64 {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        let id = entity.get_id();
        Box::into_raw(entity);
        id
    }

    #[cfg(not(feature = "wasm"))]
    fn get(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> Resource<ComponentProxy> {
        // Safely retrieve the entity proxy
        let entity_proxy = self.table.get(&entity).expect("Entity not found in table") as &EntityProxy;

        // Get entity
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        
        // Retrieve the component
        let component = entity.get(component);
        Box::into_raw(entity);

        // Create component
        let component = toxoid_engine::Component::new(component);

        // Create boxed component
        let boxed_component = Box::new(component);
        let boxed_component_ptr = Box::into_raw(boxed_component);

        // Push component to resource table
        let id = self
            .table
            .push::<ComponentProxy>(ComponentProxy { 
                ptr: boxed_component_ptr
            })
            .expect("Failed to push component to table");
        id
    }

    #[cfg(feature = "wasm")]
    fn get(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> Resource<ComponentProxy> {
        unimplemented!()
    }

    #[cfg(not(feature = "wasm"))]
    fn from_id(&mut self, id: u64) -> Resource<EntityProxy> {
        let entity = toxoid_engine::Entity::from_id(id) as *mut toxoid_engine::Entity; 
        self.table.push::<EntityProxy>(EntityProxy { ptr: entity }).unwrap()
    }

    #[cfg(feature = "wasm")]
    fn from_id(&mut self, id: u64) -> Resource<EntityProxy> {
        unimplemented!()
    }

    fn add(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.add(component);
        Box::into_raw(entity);
    }

    fn drop(&mut self, _entity: Resource<toxoid_component::component::ecs::Entity>) -> Result<(), wasmtime::Error> {
        // let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        // drop(unsafe { Box::from_raw(entity_proxy.ptr) });
        // self.table.delete::<EntityProxy>(entity).unwrap();
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostComponentType for StoreState {
    fn new(&mut self, desc: toxoid_component::component::ecs::ComponentDesc) -> Resource<ComponentTypeProxy> {
        // Create component
        let component = toxoid_engine::ComponentType::new(toxoid_engine::bindings::exports::toxoid::engine::ecs::ComponentDesc {
            name: desc.name,
            member_names: desc.member_names,
            member_types: desc.member_types,
        });
        // Create boxed component
        let boxed_component = Box::new(component);
        let boxed_component_ptr = Box::into_raw(boxed_component);
        // Push component to resource table
        let id = self
            .table
            .push::<ComponentTypeProxy>(ComponentTypeProxy { 
                ptr: boxed_component_ptr as *mut toxoid_engine::ComponentType
            })
            .unwrap();
        id
    }

    fn get_id(&mut self, component: Resource<toxoid_component::component::ecs::ComponentType>) -> u64 {    
        // Get component from resource table
        let component_proxy = self.table.get(&component).unwrap() as &ComponentTypeProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let id = component.get_id();
        Box::into_raw(component);
        id
    }

    fn drop(&mut self, _component: Resource<toxoid_component::component::ecs::ComponentType>) -> Result<(), wasmtime::Error> {
        // let component_proxy = self.table.get(&component).unwrap() as &ComponentTypeProxy;
        // drop(unsafe { Box::from_raw(component_proxy.ptr) });
        // self.table.delete::<ComponentTypeProxy>(component).unwrap();
        Ok(())
    }

}

impl toxoid_component::component::ecs::HostComponent for StoreState {
    fn new(&mut self, component_id: i64) -> Resource<ComponentProxy> {
        let component = toxoid_engine::Component::new(component_id);
        let boxed_component = Box::new(component);
        let boxed_component_ptr = Box::into_raw(boxed_component);
        self.table.push::<ComponentProxy>(ComponentProxy { ptr: boxed_component_ptr }).unwrap()
    }

    fn set_member_u8(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: u8) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u8(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u8(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> u8 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u8(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u16(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: u16) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u16(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u16(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> u16 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u16(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: u32) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u32(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> u32 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u32(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: u64) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u64(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> u64 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u64(offset);
        Box::into_raw(component);
        value
    }

    fn get_member_i8(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> i8 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i8(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i8(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: i8) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i8(offset, value);
        Box::into_raw(component);
    }

    fn get_member_i16(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> i16 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i16(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i16(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: i16) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i16(offset, value);
        Box::into_raw(component);
    }

    fn get_member_i32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> i32 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i32(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: i32) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i32(offset, value);
        Box::into_raw(component);
    }

    fn get_member_i64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> i64 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_i64(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_i64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: i64) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_i64(offset, value);
        Box::into_raw(component);
    }

    fn get_member_f32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> f32 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_f32(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_f32(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: f32) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_f32(offset, value);
        Box::into_raw(component);
    }

    fn get_member_f64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> f64 {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_f64(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_f64(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: f64) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_f64(offset, value);
        Box::into_raw(component);
    }

    fn get_member_bool(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> bool {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_bool(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_bool(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: bool) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_bool(offset, value);
        Box::into_raw(component);
    }

    fn get_member_string(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> String {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_string(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_string(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: String) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_string(offset, value);
        Box::into_raw(component);
    }

    fn get_member_u32array(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<u32> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_u32array(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_u32array(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<u32>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_u32array(offset, value);
        Box::into_raw(component);
    }

    fn get_member_f32array(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32) -> Vec<f32> {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        let value = component.get_member_f32array(offset);
        Box::into_raw(component);
        value
    }

    fn set_member_f32array(&mut self, component: Resource<toxoid_component::component::ecs::Component>, offset: u32, value: Vec<f32>) -> () {
        let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        let component = unsafe { Box::from_raw(component_proxy.ptr) };
        component.set_member_f32array(offset, value);
        Box::into_raw(component);
    }

    fn drop(&mut self, _component: Resource<toxoid_component::component::ecs::Component>) -> Result<(), wasmtime::Error> {
        // let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        // drop(unsafe { Box::from_raw(component_proxy.ptr) });
        // self.table.delete::<ComponentProxy>(component).unwrap();
        Ok(())
    }
}

impl toxoid_component::component::ecs::HostQuery for StoreState {
    fn new(&mut self, query_desc: toxoid_component::component::ecs::QueryDesc) -> Resource<toxoid_component::component::ecs::Query> {
        let query = toxoid_engine::Query::new(toxoid_engine::bindings::exports::toxoid::engine::ecs::QueryDesc { expr: query_desc.expr });
        let ptr = Box::into_raw(Box::new(query));
        let query_resource = self.table.push(QueryProxy { ptr }).unwrap();
        query_resource
    }

    fn expr(&mut self, query: Resource<toxoid_component::component::ecs::Query>, expr: String) -> () {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        query.expr(expr);
        Box::into_raw(query);
    }

    fn build(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> () {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        query.build();
        Box::into_raw(query);
    }

    fn iter(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> () {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        query.iter();
        Box::into_raw(query);
    }

    fn next(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> bool {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        let result = query.next();
        Box::into_raw(query);
        result
    }

    fn count(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> i32 {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        query.count()
    }

    fn entities(&mut self, query: Resource<toxoid_component::component::ecs::Query>) -> Vec<Resource<EntityProxy>> {
        let query_proxy = self.table.get(&query).unwrap() as &QueryProxy;
        let query = unsafe { Box::from_raw(query_proxy.ptr) };
        let entity_ids = query.entities();
        let ids = entity_ids.iter().map(|entity_id| {
            // Create entity
            let entity = toxoid_engine::Entity::from_id(*entity_id) as *mut toxoid_engine::Entity;

            // Push component to resource table
            let id = self
                .table
                .push::<EntityProxy>(EntityProxy {
                    ptr: entity
                })
                .expect("Failed to push component to table");
            id
        }).collect();
        Box::into_raw(query);
        ids
    }

    fn drop(&mut self, _query: Resource<toxoid_component::component::ecs::Query>) -> Result<(), wasmtime::Error> {
        Ok(())
    }
}

// Instantiate the WASM engine
pub static ENGINE: Lazy<Engine> = Lazy::new(Engine::default);

// Create WASM Component Linker
static LINKER: Lazy<Linker<StoreState>> = Lazy::new(|| {
    let engine = &*ENGINE; // Ensure ENGINE is initialized
    let mut linker = Linker::<StoreState>::new(engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();
    ToxoidComponentWorld::add_to_linker(&mut linker, |store_state| store_state).unwrap();
    linker
});

fn new_store() -> Store<StoreState> {
    let engine = &*ENGINE; // Ensure ENGINE is initialized
    Store::new(
        engine,
        StoreState {
            ctx: WasiCtxBuilder::new()
                    // .inherit_stdin()
                    // .inherit_stdout()
                    // .inherit_stderr()
                    .inherit_stdio()
                    .inherit_env()
                    .inherit_args()
                    // .inherit_network()
                    .build(),
            table: ResourceTable::new(),
        }
    )
}

// Create WASM Store
pub static mut STORE: Lazy<Store<StoreState>> = Lazy::new(|| new_store());
// Crate lazy initialized option
pub static mut TOXOID_COMPONENT_WORLD: Option<ToxoidComponentWorld> = None;

pub fn load_wasm_component(filename: &str) -> Result<()> {
    // Get WASM engine, linker and store
    let engine = &*ENGINE; // Ensure ENGINE is initialized
    let linker = &*LINKER; // Ensure LINKER is initialized
    // Reinitialize the store to unload the old WASM module
    // TODO: Create hashmap of stores to load multiple modules
    // where each WASM module has it's own memory and resources
    unsafe {
        *STORE = new_store();
    }
    let store = unsafe { &mut *STORE }; // Ensure STORE is initialized
    // Reset the world
    toxoid_engine::toxoid_reset();

    // Load the component from disk
    let bytes = std::fs::read(filename)?;
    
    // Create WASM Component
    let component = Component::new(&engine, bytes)?;
    unsafe { 
        // Set the world and trampoline
        TOXOID_COMPONENT_WORLD = Some(ToxoidComponentWorld::instantiate(&mut *store, &component, &linker)?); 
        toxoid_engine::QUERY_TRAMPOLINE = Some(query_trampoline);
          // Finally call init on the WASM component
        TOXOID_COMPONENT_WORLD.as_mut().unwrap().call_init(&mut *store)?;
    };

    Ok(())
}

#[no_mangle]
// Trampoline closure from Rust using C callback and binding_ctx field to call a Rust closure
pub unsafe extern "C" fn query_trampoline(iter: *mut toxoid_engine::ecs_iter_t) {
    // println!("Query trampoline called");
    // let world = toxoid_engine::WORLD.0;
    // let callback = (*iter).ctx as *mut core::ffi::c_void;
    // println!("Callback: {:?}", callback);
    // let callback_ctx = (*iter).callback_ctx as *mut core::ffi::c_void;
    // let callback_ctx = (*iter).callback_ctx as *mut core::ffi::c_void;
    // println!("Callback ctx: {:?}", callback_ctx);
    // if callback_ctx.is_null() {
    //     return;
    // }
    let handle = (*iter).callback_ctx as i64;
    let store = unsafe { &mut *STORE };
    let iter = Box::into_raw(Box::new(toxoid_engine::Iter::new(iter as i64)));
    let iter_resource_id = store.data_mut().table.push::<IterProxy>(IterProxy { ptr: iter }).unwrap();
    unsafe {
        TOXOID_COMPONENT_WORLD
            .as_ref()
            .unwrap()
            .toxoid_component_component_callbacks()
            .call_run(store, iter_resource_id, handle)
            .unwrap_or_else(|e| {
                println!("Error calling run: {:?}", e);
            });
    }
}
