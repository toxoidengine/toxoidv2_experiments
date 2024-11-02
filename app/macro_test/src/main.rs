use toxoid_api_macro::component;
use serde::{Serialize, Deserialize};
type ecs_entity_t = u64;
pub trait ComponentType {}

pub trait Component {}

component! {
    Test {
        best: u8
    }
}

fn main() {
    let test = Test {
        best: 1,
        component: ::std::ptr::null_mut(),
        singleton: false,
        id: 0
    };
    test.get_best(); 
}