pub mod bindings {
    include!("./bindings.rs");
}
use bindings::*;

pub fn init() {
    unsafe {
        let world = ecs_init();
        println!("World: {:?}", world);
    }
}