#![allow(warnings)]

pub mod bindings;
pub mod api;
pub mod components;

use api::*;
use components::*;

impl bindings::Guest for ToxoidWasmComponent {
    fn init() {
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
                    let x = position
                        .get_x();
                    let y = position
                        .get_y();
                    // Set position +1
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