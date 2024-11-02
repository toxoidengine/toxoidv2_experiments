#![allow(warnings)]

pub mod bindings;
pub mod api;

use api::*;
use bindings::toxoid_component::component::ecs::{ComponentDesc, MemberType};

use toxoid_api_macro::component;

component! {
    Position {
        x: u32,
        y: u32
    }
}

pub struct ToxoidWasmComponent;

impl bindings::Guest for ToxoidWasmComponent {
    fn init() {
        let mut entity = Entity::new(None);
        entity.add::<Position>();
        let mut position = entity.get::<Position>();
        position.set_x(777);
        position.set_y(999);
        let x = position.get_x();
        let y = position.get_y();
        println!("X: {}, Y: {}", x, y);
    }
}

#[cfg(target_arch = "wasm32")]
bindings::export!(ToxoidWasmComponent with_types_in bindings);