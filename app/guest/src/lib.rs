#![allow(warnings)]

pub mod bindings;
pub mod api;

use api::*;
use bindings::toxoid_component::component::ecs::{ComponentDesc, MemberType};

use toxoid_api_macro::component;

component! {
    Test {
        best: u8
    }
}

pub struct ToxoidWasmComponent;

impl bindings::Guest for ToxoidWasmComponent {
    fn init() {
        let mut entity = Entity::new(None);
        println!("Test 1: {}", Test::get_id());
        println!("Test 2: {}", Test::get_id());

        entity.add::<Test>();
        println!("Hello world!");
        let mut test = entity.get::<Test>();
        test.set_best(21);
        let best = test.get_best();
        println!("Best: {}", best);
    }
}

#[cfg(target_arch = "wasm32")]
bindings::export!(ToxoidWasmComponent with_types_in bindings);