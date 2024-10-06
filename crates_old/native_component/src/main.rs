use toxoid_api::bindings::exports::toxoid::api::ecs::GuestComponent;
fn main() {
    let component = toxoid_api::Component::new(toxoid_api::bindings::exports::toxoid::api::ecs::ComponentDesc {
        name: "test".to_string(),
        member_names: vec!["test".to_string()],
        member_types: vec![0],
    });
    println!("{:?}", component.get_id());
}
