#![allow(non_camel_case_types)]
#![allow(warnings)]

pub mod bindings;
use bindings::exports::toxoid::engine::ecs::{ComponentDesc, EntityDesc, Guest, GuestComponent, GuestEntity};
use toxoid_flecs::bindings::{ecs_add_id, ecs_entity_desc_t, ecs_entity_init, ecs_get_mut_id, ecs_init, ecs_member_t, ecs_struct_desc_t, ecs_struct_init, ecs_world_t};
use std::mem::MaybeUninit;
use core::ffi::c_void;
use once_cell::sync::Lazy;
type ecs_entity_t = u64;
use core::ffi::c_char;

pub struct ToxoidApi;
// struct World;
pub struct Component { 
    pub id: ecs_entity_t,
    pub ptr: *const c_void
}
pub struct Entity { 
    id: ecs_entity_t
}

pub struct EcsWorldPtr(*mut ecs_world_t);
unsafe impl Send for EcsWorldPtr {}
unsafe impl Sync for EcsWorldPtr {}

static WORLD: Lazy<EcsWorldPtr> = Lazy::new(|| 
    EcsWorldPtr(unsafe { ecs_init() })
);

// impl GuestWorld for World {
//     fn new() -> World {
//         World::new()
//     }
// }

unsafe fn get_member_type(member_type: u8) -> ecs_entity_t {
    match member_type {
        0 => toxoid_flecs::bindings::FLECS_IDecs_u8_tID_,
        1 => toxoid_flecs::bindings::FLECS_IDecs_u16_tID_,
        2 => toxoid_flecs::bindings::FLECS_IDecs_u32_tID_,
        3 => toxoid_flecs::bindings::FLECS_IDecs_u64_tID_,
        4 => toxoid_flecs::bindings::FLECS_IDecs_i8_tID_,
        5 => toxoid_flecs::bindings::FLECS_IDecs_i16_tID_,
        6 => toxoid_flecs::bindings::FLECS_IDecs_i32_tID_,
        7 => toxoid_flecs::bindings::FLECS_IDecs_i64_tID_,
        8 => toxoid_flecs::bindings::FLECS_IDecs_f32_tID_,
        9 => toxoid_flecs::bindings::FLECS_IDecs_f64_tID_,
        10 => toxoid_flecs::bindings::FLECS_IDecs_bool_tID_,
        11 => toxoid_flecs::bindings::FLECS_IDecs_string_tID_,
        _ => toxoid_flecs::bindings::FLECS_IDecs_uptr_tID_,
    }
}

impl GuestComponent for Component {
    fn new(desc: ComponentDesc) -> Component {
        unsafe {
            // Create component entity
            let mut ent_desc: ecs_entity_desc_t = MaybeUninit::zeroed().assume_init();
            ent_desc.name = desc.name.as_ptr() as *const i8;
            let component_entity: ecs_entity_t = ecs_entity_init(WORLD.0, &ent_desc);

            // Create runtime component description
            let mut struct_desc: ecs_struct_desc_t = MaybeUninit::zeroed().assume_init();
            struct_desc.entity = component_entity;
            let member: ecs_member_t = MaybeUninit::zeroed().assume_init();
            struct_desc.members = [member; 32usize];
            
            // Iterate through member names
            for (index, member_name) in desc.member_names.iter().enumerate() {
                // Create component member
                let mut member: ecs_member_t = MaybeUninit::zeroed().assume_init();
                member.name = member_name.as_ptr() as *const i8;
                member.type_ = get_member_type(desc.member_types[index]);
                struct_desc.members[index] = member;
            }

            // Initialize component
            let component = ecs_struct_init(WORLD.0, &struct_desc);

            // Return component 
            Component { 
                id: component_entity,
                ptr: std::ptr::null_mut()
            }
        }
    }

    fn get_id(&self) -> ecs_entity_t {
        self.id
    }

    // fn set_member_u8(&self, offset: u32, value: u8) {
    //     unsafe {
    //         let member_ptr = self.ptr.offset(offset as isize) as *mut u8;
    //         *member_ptr = value;
    //     }
    // }
}

impl GuestEntity for Entity {
    fn new(desc: EntityDesc) -> Entity {
        unsafe {
            let mut ent_desc: ecs_entity_desc_t = MaybeUninit::zeroed().assume_init();
            if let Some(name) = desc.name {
                ent_desc.name = name.as_ptr() as *const i8;
            }
            let entity = ecs_entity_init(WORLD.0, &ent_desc);
            Entity { id: entity }
        }
    }
    
    fn get_id(&self) -> ecs_entity_t {
        self.id
    }

    fn add_component(&self, component: ecs_entity_t) {
        unsafe {
            ecs_add_id(WORLD.0, self.id, component);
        }
    }

    fn get_component(&self, component: ecs_entity_t) -> bindings::exports::toxoid::engine::ecs::Component {
        unsafe {
            let ptr = ecs_get_mut_id(WORLD.0, self.id, component);
            bindings::exports::toxoid::engine::ecs::Component::new(Component { id: component, ptr })
        }
    }
}

impl Guest for ToxoidApi {
    type Component = Component;
    type Entity = Entity;
    // fn component_get(name: String) -> ecs_entity_t {
    //     toxoid_flecs::component_get(name.as_ptr())
    // }
}

#[cfg(target_arch = "wasm32")]
bindings::export!(ToxoidApi with_types_in bindings);