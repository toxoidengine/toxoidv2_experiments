#[allow(warnings)]
pub mod bindings;
use bindings::{toxoid_component::component::ecs::{Component, ComponentDesc, ComponentType, Entity, EntityDesc, MemberType, Query, QueryDesc}, Guest};

pub struct ToxoidWasmComponent;

bindings::export!(ToxoidWasmComponent with_types_in bindings);