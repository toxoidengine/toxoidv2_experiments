#![allow(non_camel_case_types)]
#![allow(warnings)]

mod bindings;
use bindings::exports::toxoid::api::ecs::{GuestComponent, Guest, ComponentDesc};
use toxoid_flecs::bindings::{ecs_entity_desc_t, ecs_entity_init, ecs_init};
use std::mem::MaybeUninit;
use core::ffi::c_void;

type ecs_entity_t = u64;

struct ToxoidApi;
// struct World;
struct Component { 
    id: ecs_entity_t,
    ptr: *const c_void
}

// impl GuestWorld for World {
//     fn new() -> World {
//         World::new()
//     }
// }

impl GuestComponent for Component {
    fn new(desc: ComponentDesc) -> Component {
        unsafe {
            let mut ent_desc: ecs_entity_desc_t = MaybeUninit::zeroed().assume_init();
            ent_desc.name = desc.name.as_ptr() as *const i8;
            let world = ecs_init();
            let component_entity: ecs_entity_t = ecs_entity_init(world, &ent_desc);
            Component { 
                id: component_entity ,
                ptr: std::ptr::null_mut()
            }
        }
    }

    fn get_id(&self) -> ecs_entity_t {
        self.id
    }
}

impl Guest for ToxoidApi {
    type Component = Component;

    // fn component_get(name: String) -> ecs_entity_t {
    //     toxoid_flecs::component_get(name.as_ptr())
    // }
}

bindings::export!(ToxoidApi with_types_in bindings);