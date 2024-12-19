#![allow(warnings)]
pub mod components;
use components::*;
use toxoid_api::*;
use toxoid_wasm_component::bindings::{
    self, 
    exports::toxoid_component::component::callbacks::Guest as CallbacksGuest, 
    Guest as WorldGuest,
    toxoid_component::component::ecs::Iter
};

pub struct ToxoidWasmComponent;

impl CallbacksGuest for ToxoidWasmComponent {
    fn run(iter: Iter, handle: i64) {
        run_callback(iter, handle);
    }
}

impl WorldGuest for ToxoidWasmComponent {
    fn init() {
        println!("Hello?");
        let mut entity = Entity::named("Test");
        entity.add::<Position>();
        let mut position = entity.get::<Position>();
        position.set_x(777);
        position.set_y(999);
        let mut entity = Entity::new(None);
        entity.add::<Position>();
        let mut position = entity.get::<Position>();
        position.set_x(555);
        position.set_y(888);
        System::dsl("Position($this)", |iter| {
            iter
                .entities()
                .iter()
                .for_each(|entity| {
                    let mut position = entity.get::<Position>();
                    let x = position.get_x();
                    let y = position.get_y();
                    position.set_x(x + 1);
                    position.set_y(y + 1);
                    println!("Entity: {}", entity.get_id());
                    println!("Position -  X: {}, Y: {}", x, y);
                });
        })
        .build();
    }
}

#[cfg(target_arch = "wasm32")]
bindings::export!(ToxoidWasmComponent with_types_in bindings);