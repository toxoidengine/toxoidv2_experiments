// Bindgen the WASM runtime based component instance bindings based on WIT.
bindgen!({
    world: "toxoid-api-world",
    path: "../toxoid_api/wit",
});

bindgen!({
    world: "toxoid-component-world",
    path: "../toxoid_wasm_component/wit",
    with: {
        // Specify that our host resource is going to point to the `ComponentProxy`
        // which is defined just below this macro.
        "toxoid-component:component/ecs/component": ComponentProxy,
    },
});

use exports::toxoid::api::ecs::ComponentDesc;
use wasmtime::component::{bindgen, Component, Linker, Resource, ResourceAny, ResourceTable};
use wasmtime::{Engine, Result, Store};
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

// ResourceAnyPtr is a wrapper around ResourceAny to allow for safe concurrent access.
struct ResourceAnyPtr(*mut ResourceAny);
unsafe impl Send for ResourceAnyPtr {}
/// A sample host-defined type which contains arbitrary host-defined data.
///
/// In this case this is relatively simple but there's no restrictions on what
/// this type can hold other than that it must be `'static + Send`.
pub struct ComponentProxy {
    id: u64,
    ptr: Arc<Mutex<ResourceAnyPtr>>
}

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

impl toxoid_component::component::ecs::HostComponent for StoreState {
    fn new(&mut self, _desc: toxoid_component::component::ecs::ComponentDesc) -> Resource<ComponentProxy> {
        let engine = &*ENGINE; // Ensure ENGINE is initialized
        let linker = &*LINKER; // Ensure LINKER is initialized
        let store = unsafe { &mut *STORE }; // Ensure STORE is initialized
        // Load the component from disk
        let bytes = std::fs::read("toxoid_api.wasm").unwrap();
        // Create WASM Component
        let component = Component::new(&engine, bytes).unwrap();
        
        // Instantiate the WASM component
        let toxoid_world = ToxoidApiWorld::instantiate(&mut *store, &component, &linker).unwrap();
        let toxoid_ecs_interface = toxoid_world.toxoid_api_ecs();
        let toxoid_ecs_component = toxoid_ecs_interface.component();
        let component = toxoid_ecs_component
            .call_constructor(&mut *store, &ComponentDesc{ 
                name: "test".to_string(),
                member_names: vec![],
                member_types: vec![]
            })
            .unwrap();

        let component_id = toxoid_ecs_component.call_get_id(&mut *store, component).unwrap();
        
        let boxed_component = Box::new(component);
        let leaked_component = Box::into_raw(boxed_component);
        let id = self
            .table
            .push::<ComponentProxy>(ComponentProxy { 
                id: component_id, 
                ptr: Arc::new(Mutex::new(ResourceAnyPtr(leaked_component))) 
            })
            .unwrap();
        id
    }

    fn get_id(&mut self, component: Resource<toxoid_component::component::ecs::Component>) -> u64 {
        let component = self.table.get(&component).unwrap();
        println!("ID: {:?}", component.id);
        println!("PTR: {:?}", component.ptr.lock().unwrap().0);
        component.id
    }

    fn drop(&mut self, _component: Resource<toxoid_component::component::ecs::Component>) -> Result<(), wasmtime::Error> {
        // Drop the WASI context
        // self.ctx.drop();
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
    let bytes = std::fs::read("toxoid_api.wasm")?;
    // Create WASM Component
    let component = Component::new(&engine, bytes)?;

    // Instantiate the WASM component
    let toxoid_world = ToxoidApiWorld::instantiate(&mut *store, &component, &linker)?;
    let toxoid_ecs_interface = toxoid_world.toxoid_api_ecs();
    let toxoid_ecs_component = toxoid_ecs_interface.component();
    let component = toxoid_ecs_component.call_constructor(&mut *store, &ComponentDesc{ 
        name: "test".to_string(), 
        member_names: vec![], 
        member_types: vec![] 
    })?;
    let id = toxoid_ecs_component.call_get_id(&mut *store, component)?;

    println!("ID: {:?}", id);

    // Load the component from disk
    let bytes = std::fs::read("toxoid_wasm_component.wasm")?;
    // Create WASM Component
    let component = Component::new(&engine, bytes)?;
    let toxoid_component_world = ToxoidComponentWorld::instantiate(&mut *store, &component, &linker)?;
    let guest_id =toxoid_component_world.call_init(&mut *store)?;

    println!("Guest ID: {:?}", guest_id);

    Ok(())
}