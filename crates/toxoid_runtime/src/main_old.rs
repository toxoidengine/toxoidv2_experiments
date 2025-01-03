// Bindgen the WASM runtime based component instance bindings based on WIT.
bindgen!({
    world: "toxoid-api-world",
    path: "../toxoid_api/wit",
});

bindgen!({
    world: "toxoid-component-world",
    path: "../toxoid_guest/wit",
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

/// A sample host-defined type which contains arbitrary host-defined data.
///
/// In this case this is relatively simple but there's no restrictions on what
/// this type can hold other than that it must be `'static + Send`.
pub struct ComponentProxy {
    resource: ResourceAny
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
    fn new(&mut self, desc: toxoid_component::component::ecs::ComponentDesc) -> Resource<ComponentProxy> {
        // Get global toxoid api and store
        let toxoid_api = &*TOXOID_API;
        let store = unsafe { &mut *STORE };

        // Get component API
        let toxoid_ecs_interface = toxoid_api.toxoid_api_ecs();
        let toxoid_ecs_component = toxoid_ecs_interface.component();
        // Instantiate an ECS component
        let component = toxoid_ecs_component
            .call_constructor(&mut *store, &ComponentDesc{ 
                name: desc.name,
                member_names: vec![],
                member_types: vec![]
            })
            .unwrap();

        let id = self
            .table
            .push::<ComponentProxy>(ComponentProxy { 
                resource: component 
            })
            .unwrap();
        id
    }

    fn get_id(&mut self, component: Resource<toxoid_component::component::ecs::Component>) -> u64 {
        // Get global toxoid api and store
        let toxoid_api = &*TOXOID_API;
        let store = unsafe { &mut *STORE };

        // Get component API
        let toxoid_ecs_interface = toxoid_api.toxoid_api_ecs();
        let toxoid_ecs_component = toxoid_ecs_interface.component();

        // Get component from resource table
        let component = self.table.get(&component).unwrap() as &ComponentProxy;
        // Get component id using toxoid api and passing in the resource of the ECS Component instance
        let component_id = toxoid_ecs_component.call_get_id(&mut *store, component.resource.clone()).unwrap();

        // Return the component id
        component_id
    }

    fn drop(&mut self, component: Resource<toxoid_component::component::ecs::Component>) -> Result<(), wasmtime::Error> {
        self.table.delete::<ComponentProxy>(component).unwrap();
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

// Create TOXOID API
static TOXOID_API: Lazy<ToxoidApiWorld> = Lazy::new(|| {
    let engine = &*ENGINE; // Ensure ENGINE is initialized
    let linker = &*LINKER; // Ensure LINKER is initialized
    let store = unsafe { &mut *STORE }; // Ensure STORE is initialized

    // Load the component from disk
    let bytes = std::fs::read("toxoid_api.wasm").unwrap();
    // Create WASM Component
    let component = Component::new(&engine, bytes).unwrap();
    // Instantiate the WASM component
    ToxoidApiWorld::instantiate(&mut *store, &component, &linker).unwrap()
});

fn main() -> Result<()> {
    // Get WASM engine, linker and store
    let engine = &*ENGINE; // Ensure ENGINE is initialized
    let linker = &*LINKER; // Ensure LINKER is initialized
    let store = unsafe { &mut *STORE }; // Ensure STORE is initialized

    // Load the component from disk
    let bytes = std::fs::read("toxoid_guest.wasm")?;
    // Create WASM Component
    let component = Component::new(&engine, bytes)?;
    let toxoid_component_world = ToxoidComponentWorld::instantiate(&mut *store, &component, &linker)?;
    let component_id = toxoid_component_world.call_init(&mut *store, "test")?;
    let component_id_2 = toxoid_component_world.call_init(&mut *store, "test2")?;

    println!("Component ID: {:?}", component_id);
    println!("Component ID 2: {:?}", component_id_2);

    Ok(())
}