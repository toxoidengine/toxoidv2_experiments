fn main() -> wasmtime::Result<()> {
    // Instantiate the engine and store
    let engine = wasmtime::Engine::default();
    let mut store = wasmtime::Store::new(&engine, ());

    // Load the component from disk
    let bytes = std::fs::read("toxoid_api.wasm")?;
    let component = wasmtime::component::Component::new(&engine, bytes)?;

    // Configure the linker
    let mut linker = wasmtime::component::Linker::new(&engine);
    // The component expects one import `name` that
    // takes no params and returns a string
    linker
        .root()
        .func_wrap("name", |_store, _params: ()| {
            Ok((String::from("Alice"),))
        })?;

    // Instantiate the component
    let instance = linker.instantiate(&mut store, &component)?;

    // Call the `greet` function
    let func = instance
        .get_func(&mut store, "greet")
        .expect("greet export not found");
    let mut result = [wasmtime::component::Val::String("".into())];
    func.call(&mut store, &[], &mut result)?;

    // This should print out `Greeting: [String("Hello, Alice!")]`
    println!("Greeting: {:?}", result);

    Ok(())
}