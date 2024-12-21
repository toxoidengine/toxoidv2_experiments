use wasm_component_layer::*;

// The bytes of the component.
const WASM: &[u8] = include_bytes!("../component.wasm");

pub fn main() {
    println!("Hello world!");
    // Create a new engine for instantiating a component.
    let engine = Engine::new(wasmi_runtime_layer::Engine::default());
    println!("Hello world 2!");
    // Create a store for managing WASM data and any custom user-defined state.
    let mut store = Store::new(&engine, ());
    println!("Hello world 3!");
    println!("WASM: {:?}", WASM);
    // Parse the component bytes and load its imports and exports.
    let component = Component::new(&engine, WASM).unwrap();
    println!("Hello world 4!");
    // Create a linker that will be used to resolve the component's imports, if any.
    let linker = Linker::default();
    println!("Hello world 5!");
    // Create an instance of the component using the linker.
    let instance = linker.instantiate(&mut store, &component).unwrap();
    println!("Hello world 6!");

    // Get the interface that the interface exports.
    let interface = instance.exports().instance(&"test:guest/foo".try_into().unwrap()).unwrap();
    println!("Hello world 7!");
    // Get the function for selecting a list element.
    let select_nth = interface.func("select-nth").unwrap().typed::<(Vec<String>, u32), String>().unwrap();
    println!("Hello world 8!");
    // Create an example list to test upon.
    let example = ["a", "b", "c"].iter().map(ToString::to_string).collect::<Vec<_>>();

    println!("Calling select-nth({example:?}, 1) == {}", select_nth.call(&mut store, (example.clone(), 1)).unwrap());
    println!("Hello world 9!");
    // Prints 'Calling select-nth(["a", "b", "c"], 1) == b'
}