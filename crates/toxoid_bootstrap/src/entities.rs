use toxoid_api::*;
use toxoid_api::components::*;

pub fn create_snake() {
    let mut snake_head = Entity::new(Some(EntityDesc {
        name: Some("Snake Head".to_string())
    }));
    snake_head.add::<Position>();
    snake_head.add::<Size>();
    snake_head.add::<Color>();
    snake_head.add::<Head>();
    snake_head.add::<Player>();
    snake_head.add::<Renderable>();

    World::add_singleton::<Direction>();

    let mut pos = snake_head.get::<Position>();
    pos.set_x(0);
    pos.set_y(0);

    let mut size = snake_head.get::<Size>();
    size.set_width(50);
    size.set_height(50);

    let mut color = snake_head.get::<Color>();
    color.set_r(0.0);
    color.set_g(1.0);
    color.set_b(0.0);
    color.set_a(1.0);
}

pub fn create_food() {
    let mut food = Entity::new(Some(EntityDesc {
        name: Some("Food".to_string())
    }));
    food.add::<Position>();
    food.add::<Size>();
    food.add::<Color>();
    food.add::<Food>();
    food.add::<Renderable>();

    let mut pos = food.get::<Position>();
    pos.set_x(200);
    pos.set_y(200);

    let mut size = food.get::<Size>();
    size.set_width(50);
    size.set_height(50);

    let mut color = food.get::<Color>();
    color.set_r(1.0);
    color.set_g(0.0);
    color.set_b(0.0);
    color.set_a(1.0);
}