#[allow(warnings)]
mod bindings;
use bindings::exports::toxoid::api::ecs::{GuestComponent, Guest};

struct ToxoidApi;
struct Component;

impl GuestComponent for Component {
    fn new(_init: Vec<u8>) -> Component {
        Component
    }

    fn write(&self, bytes: Vec<u8>) {
        println!("Writing bytes: {:?}", bytes);
    }

    fn read(n: u32) -> Vec<u8> {
        vec![0; n as usize]
    }

    fn get_id() -> u64 {
        toxoid_flecs::init()
    }
}

impl Guest for ToxoidApi {
    type Component = Component;
}

bindings::export!(ToxoidApi with_types_in bindings);