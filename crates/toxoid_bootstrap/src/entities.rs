use toxoid_api::*;

component! {
    Velocity {
        x: u32,
        y: u32
    }
}

pub fn init() {
    // let mut entity = Entity::new(Some(EntityDesc {
    //     name: Some("Test entity".to_string())
    // }));
    // entity.add::<Rect>();
    // entity.add::<Position>();
    // entity.add::<Size>();
    // entity.add::<Color>();
    // entity.add::<Renderable>();
    // let mut size = entity.get::<Size>();
    // size.set_width(100);
    // size.set_height(100);
    // let mut color = entity.get::<Color>();
    // color.set_r(1.);
    // color.set_g(0.);
    // color.set_b(0.);
    // color.set_a(1.);

    println!("{}", Velocity::get_name());
    let entity = Entity::new(Some(EntityDesc {
        name: Some("Test entity".to_string())
    }));
    // Create entity with Velocity component
    let mut entity = Entity::named("Test");
    entity.add::<Velocity>();
    let mut velocity = entity.get::<Velocity>();
    velocity.set_x(777);
    velocity.set_y(999);
    // Print x and y of velocity component
    println!("Velocity 1 - X: {}, Y: {}", velocity.get_x(), velocity.get_y());
    let mut entity = Entity::new(None);
    entity.add::<Velocity>();
    let mut velocity = entity.get::<Velocity>();
    velocity.set_x(555);
    velocity.set_y(888);
    // Print x and y of velocity component
    println!("Velocity 2 - X: {}, Y: {}", velocity.get_x(), velocity.get_y());
    System::dsl("Velocity($this)", |iter| {
        iter
            .entities()
            .iter()
            .for_each(|entity| {
                let mut velocity = entity.get::<Velocity>();
                let x = velocity.get_x();
                let y = velocity.get_y();
                velocity.set_x(x + 1);
                velocity.set_y(y + 1);
                println!("Entity: {}", entity.get_id());
                println!("Velocity -  X: {}, Y: {}", x, y);
            });
    })
    .build();
}