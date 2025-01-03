use toxoid_api::*;
use toxoid_api::components::*;

pub fn create_snake() {
    let mut snake = Entity::new(Some(EntityDesc {
        name: Some("Snake Head".to_string())
    }));
    snake.add::<Position>();
    snake.add::<Size>();
    snake.add::<Color>();
    snake.add::<Head>();
    snake.add::<Tail>();
    snake.add::<Player>();
    snake.add::<Renderable>();

    World::add_singleton::<Direction>();

    let mut pos = snake.get::<Position>();
    pos.set_x(0);
    pos.set_y(0);

    let mut size = snake.get::<Size>();
    size.set_width(50);
    size.set_height(50);

    let mut color = snake.get::<Color>();
    color.set_r(0.0);
    color.set_g(1.0);
    color.set_b(0.0);
    color.set_a(1.0);

    let mut tail = snake.get::<Tail>();
    tail.set_entities(vec![]);
    // let mut entities_vec = tail.get_entities();
    // entities_vec.push(0);
    // entities_vec.push(1);
    // entities_vec.push(2);
    // entities_vec.push(3);
    // entities_vec.push(420);
    // tail.set_entities(entities_vec);
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

pub fn init() {
    create_snake();
    create_food();
}