use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder};

fn main() -> wasmtime::Result<()> {
    // Instantiate the engine and store
    let engine = wasmtime::Engine::default();
    // Create WASM Component Linker
    let mut linker = wasmtime::component::Linker::<MyState>::new(&engine);
    // Add WASI imports to the linker
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    // The component expects one import `name` that
    // takes no params and returns a string
    linker
        .root()
        .func_wrap("name", |_store, _params: ()| {
            Ok((String::from("Alice"),))
        })?;

    // Create WASI Context
    let mut builder = WasiCtxBuilder::new();
    // Create WASM Store
    let mut store = wasmtime::Store::new(
        &engine,
        MyState {
            ctx: builder.build(),
            table: ResourceTable::new(),
        },
    );
    

    // Load the component from disk
    let bytes = std::fs::read("toxoid_api.wasm")?;
    // Create WASM Component
    let component = wasmtime::component::Component::new(&engine, bytes)?;

    // Instantiate the component
    let instance = linker.instantiate(&mut store, &component)?;
    
    // Call the `greet` function
    let func = instance
        .get_func(&mut store, "greet")
        .expect("greet export not found");
    // Create result memory to store the result
    let mut result = [wasmtime::component::Val::String("".into())];
    // Call the `greet` function
    // Params: Store, func params, func result
    func.call(&mut store, &[], &mut result)?;

    // This should print out `Greeting: [String("Hello, Alice!")]`
    println!("Greeting: {:?}", result);

    Ok(())
}

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}