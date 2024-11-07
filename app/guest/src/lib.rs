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
        let x = position.get_x();
        let y = position.get_y();
        println!("X: {}, Y: {}", x, y);

        let mut entity = Entity::new(None);
        entity.add::<Position>();
        let mut position = entity.get::<Position>();
        position.set_x(555);
        position.set_y(888);
        let x = position.get_x();
        let y = position.get_y();
        println!("X: {}, Y: {}", x, y);

        // let mut query = Query::dsl("Position($this)");
        // query.build();
        // query.iter();
        // query.next();
        // let count = query.count();
        // println!("Count: {}", count);
        // query
        //     .entities()
        //     .iter()
        //     .for_each(|entity| {
        //         println!("Entity: {}", entity.get_id());
        //         let id = Position::get_id();
        //         println!("ID: {}", id);
        //         let x = entity.get::<Position>().get_x();
        //         let y = entity.get::<Position>().get_y();
        //         println!("Iter X: {}, Iter Y: {}", x, y);
        //     });

        let mut system = System::dsl("Position($this)", |iter| {
            iter
            .entities()
            .iter()
            .for_each(|entity| {
                let x = entity.get::<Position>().get_x();
                let y = entity.get::<Position>().get_y();
                println!("Entity: {}", entity.get_id());
                println!("Position -  X: {}, Y: {}", x, y);
            });
        });
        system.build();
    }
}

#[cfg(target_arch = "wasm32")]
bindings::export!(ToxoidWasmComponent with_types_in bindings);