bindgen!({
    world: "toxoid-component-world",
    path: "../toxoid_wasm_component/wit",
    with: {
        // Specify that our host resource is going to point to the `ComponentProxy`
        "toxoid-component:component/ecs/component-type": ComponentTypeProxy,
        "toxoid-component:component/ecs/component": ComponentProxy,
        "toxoid-component:component/ecs/entity": EntityProxy,
    },
});

use toxoid_engine::bindings::exports::toxoid::engine::ecs::{GuestComponent, GuestComponentType, GuestEntity};
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

// StoreState is the state of the WASM store.
struct StoreState {
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
    fn get_component(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> Resource<ComponentProxy> {
        // Safely retrieve the entity proxy
        let entity_proxy = self.table.get(&entity).expect("Entity not found in table") as &EntityProxy;

        // Get entity
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        
        // Retrieve the component
        let component = entity.get_component(component);
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
        println!("Component ID: {:?}", id);
        id
    }

    #[cfg(feature = "wasm")]
    fn get_component(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> Resource<ComponentProxy> {
        unimplemented!()
    }

    fn add_component(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>, component: toxoid_component::component::ecs::EcsEntityT) -> () {
        let entity_proxy = self.table.get(&entity).unwrap() as &EntityProxy;
        let entity = unsafe { Box::from_raw(entity_proxy.ptr) };
        entity.add_component(component);
        Box::into_raw(entity);
    }

    fn drop(&mut self, entity: Resource<toxoid_component::component::ecs::Entity>) -> Result<(), wasmtime::Error> {
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

    fn drop(&mut self, component: Resource<toxoid_component::component::ecs::ComponentType>) -> Result<(), wasmtime::Error> {
        // let component_proxy = self.table.get(&component).unwrap() as &ComponentTypeProxy;
        // drop(unsafe { Box::from_raw(component_proxy.ptr) });
        // self.table.delete::<ComponentTypeProxy>(component).unwrap();
        Ok(())
    }

}

impl toxoid_component::component::ecs::HostComponent for StoreState {
    // fn new(&mut self, resource_id: u32) -> Resource<ComponentProxy> {
    //     todo!()
    // }
    // fn new(&mut self, resource_id: u32) -> Resource<ComponentProxy> {
    //     // Create component
    //     let component = toxoid_engine::ComponentType::new(toxoid_engine::bindings::exports::toxoid::engine::ecs::ComponentDesc {
    //         name: "".to_string(),
    //         member_names: vec![],
    //         member_types: vec![],
    //     });
    //     // Create boxed component
    //     let boxed_component = Box::new(component);
    //     let boxed_component_ptr = Box::into_raw(boxed_component);
    //     // Push component to resource table
    //     let id = self
    //         .table
    //         .push::<ComponentProxy>(ComponentProxy { 
    //             ptr: boxed_component_ptr as *mut toxoid_engine::Component
    //         })
    //         .unwrap();
    //     id
    // }

    fn drop(&mut self, component: Resource<toxoid_component::component::ecs::Component>) -> Result<(), wasmtime::Error> {
        // let component_proxy = self.table.get(&component).unwrap() as &ComponentProxy;
        // drop(unsafe { Box::from_raw(component_proxy.ptr) });
        // self.table.delete::<ComponentProxy>(component).unwrap();
        Ok(())
    }
}

// Instantiate the WASM engine
static ENGINE: Lazy<Engine> = Lazy::new(Engine::default);

// Create WASM Component Linker
static LINKER: Lazy<Linker<StoreState>> = Lazy::new(|| {
    let engine = &*ENGINE; // Ensure ENGINE is initialized
    let mut linker = Linker::<StoreState>::new(engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();
    ToxoidComponentWorld::add_to_linker(&mut linker, |store_state| store_state).unwrap();
    linker
});

// Create WASM Store
static mut STORE: Lazy<Store<StoreState>> = Lazy::new(|| {
    let engine = &*ENGINE; // Ensure ENGINE is initialized
    Store::new(
        engine,
        StoreState {
            ctx: WasiCtxBuilder::new().build(),
            table: ResourceTable::new(),
        }
    )
});

fn main() -> Result<()> {
    // Get WASM engine, linker and store
    let engine = &*ENGINE; // Ensure ENGINE is initialized
    let linker = &*LINKER; // Ensure LINKER is initialized
    let store = unsafe { &mut *STORE }; // Ensure STORE is initialized

    // Load the component from disk
    let bytes = std::fs::read("toxoid_wasm_component.wasm")?;
    // Create WASM Component
    let component = Component::new(&engine, bytes)?;
    let toxoid_component_world = ToxoidComponentWorld::instantiate(&mut *store, &component, &linker)?;
    let component_id = toxoid_component_world.call_init(&mut *store, "test")?;
    let component_id_2 = toxoid_component_world.call_init(&mut *store, "test2")?;

    println!("Component ID: {:?}", component_id);
    println!("Component ID 2: {:?}", component_id_2);

    Ok(())
}