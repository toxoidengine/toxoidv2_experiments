use wasmtime::component::{bindgen, ResourceTable};
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder};

bindgen!({
    world: "toxoid-world",
    path: "../toxoid_api/wit",
});

struct StoreState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for StoreState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}

fn main() -> wasmtime::Result<()> {
    // Instantiate the engine and store
    let engine = wasmtime::Engine::default();
    // Create WASM Component Linker
    let mut linker = wasmtime::component::Linker::<StoreState>::new(&engine);
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
    let component = wasmtime::component::Component::new(&engine, bytes)?;

    // Instantiate the WASM component
    let toxoid_world = ToxoidWorld::instantiate(&mut store, &component, &linker)?;
    let ecs_interface = toxoid_world.toxoid_api_ecs();
    let component = ecs_interface.component();
    let component_instance = component.call_constructor(&mut store, &[])?;
    let _ = component.call_write(&mut store, component_instance, &[]);
    let bytes = component.call_read(&mut store, 10);
    
    println!("Read bytes: {:?}", bytes);

    Ok(())
}