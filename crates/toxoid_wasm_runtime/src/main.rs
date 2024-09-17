// Bindgen the WASM runtime based component instance bindings based on WIT.
bindgen!({
    world: "toxoid-api-world",
    path: "../toxoid_api/wit",
});

bindgen!({
    world: "toxoid-component-world",
    path: "../toxoid_wasm_component/wit",
});

use exports::toxoid::api::ecs::ComponentDesc;
use wasmtime::component::{bindgen, Component, Linker, Resource, ResourceTable};
use wasmtime::{Engine, Result};
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder};

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

impl toxoid_component::component::ecs::HostComponent for StoreState {
    fn new(&mut self, _desc: toxoid_component::component::ecs::ComponentDesc) -> Resource<toxoid_component::component::ecs::Component> {
        // Instantiate the engine and store
        let engine = Engine::default();
        // Create WASM Component Linker
        let mut linker = Linker::<StoreState>::new(&engine);
        // Add WASI imports to the linker
        wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();
        // Create WASI Context
        let mut builder = WasiCtxBuilder::new();
        // Create WASM Store
        let mut store = wasmtime::Store::new(
            &engine,
            StoreState {
                ctx: builder.build(),
                table: ResourceTable::new(),
            },
        );
        
        // Load the component from disk
        let bytes = std::fs::read("toxoid_api.wasm").unwrap();
        // Create WASM Component
        let component = Component::new(&engine, bytes).unwrap();

        // Instantiate the WASM component
        let toxoid_world = ToxoidApiWorld::instantiate(&mut store, &component, &linker).unwrap();
        let toxoid_ecs_interface = toxoid_world.toxoid_api_ecs();
        let toxoid_ecs_component = toxoid_ecs_interface.component();
        let _component = toxoid_ecs_component.call_constructor(&mut store, &ComponentDesc{ 
            name: "test".to_string(), 
            member_names: vec![], 
            member_types: vec![] 
        }).unwrap();

        let resource: Resource<toxoid_component::component::ecs::Component>= Resource::new_borrow(0);
        resource
    }

    fn get_id(&mut self, _component: Resource<toxoid_component::component::ecs::Component>) -> u64 {
        0
    }

    fn drop(&mut self, _component: Resource<toxoid_component::component::ecs::Component>) -> Result<(), wasmtime::Error> {
        // Drop the WASI context
        // self.ctx.drop();
        Ok(())
    }
}

fn main() -> Result<()> {
    // Instantiate the engine and store
    let engine = Engine::default();
    // Create WASM Component Linker
    let mut linker = Linker::<StoreState>::new(&engine);
    // Add WASI imports to the linker
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    // Create WASI Context
    let mut builder = WasiCtxBuilder::new();
    // Create WASM Store
    let mut store = wasmtime::Store::new(
        &engine,
        StoreState {
            ctx: builder.build(),
            table: ResourceTable::new(),
        },
    );
    
    // Load the component from disk
    let bytes = std::fs::read("toxoid_api.wasm")?;
    // Create WASM Component
    let component = Component::new(&engine, bytes)?;

    // Instantiate the WASM component
    let toxoid_world = ToxoidApiWorld::instantiate(&mut store, &component, &linker)?;
    let toxoid_ecs_interface = toxoid_world.toxoid_api_ecs();
    let toxoid_ecs_component = toxoid_ecs_interface.component();
    let component = toxoid_ecs_component.call_constructor(&mut store, &ComponentDesc{ 
        name: "test".to_string(), 
        member_names: vec![], 
        member_types: vec![] 
    })?;
    let id = toxoid_ecs_component.call_get_id(&mut store, component)?;

    println!("ID: {:?}", id);

    // Load the component from disk
    let bytes = std::fs::read("toxoid_wasm_component.wasm")?;
    // Create WASM Component
    let component = Component::new(&engine, bytes)?;
    let toxoid_component_world = ToxoidComponentWorld::instantiate(&mut store, &component, &linker)?;
    toxoid_component_world.call_init(&mut store)?;

    Ok(())
}