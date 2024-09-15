pub mod bindings {
    include!("./bindings.rs");
}
pub use bindings::*;

pub fn init() {
    unsafe {
        let world = bindings::ecs_init();
        println!("World: {:?}", world);
    }
}