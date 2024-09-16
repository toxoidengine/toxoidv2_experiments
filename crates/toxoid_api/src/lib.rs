#[allow(warnings)]
mod bindings;
use bindings::exports::toxoid::api::ecs::{GuestComponent, Guest};
use toxoid_flecs::ecs_entity_t;

struct ToxoidApi;
// struct World;
struct Component;

// impl GuestWorld for World {
//     fn new() -> World {
//         World::new()
//     }
// }

impl GuestComponent for Component {
    fn new(_init: Vec<u8>) -> Component {
        Component
    }
}

impl Guest for ToxoidApi {
    type Component = Component;

    fn component_get(name: String) -> ecs_entity_t {
        toxoid_flecs::component_get(name.as_ptr())
    }
}

bindings::export!(ToxoidApi with_types_in bindings);