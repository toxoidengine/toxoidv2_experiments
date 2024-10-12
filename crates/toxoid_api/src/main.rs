#[cfg(not(target_arch = "wasm32"))]
use toxoid_engine::bindings::exports::toxoid::api::ecs::GuestComponent;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    let component = toxoid_engine::Component::new(toxoid_engine::bindings::exports::toxoid::api::ecs::ComponentDesc {
        name: "test".to_string(),
        member_names: vec!["test".to_string()],
        member_types: vec![0],
    });
    println!("{:?}", component.get_id());
}
