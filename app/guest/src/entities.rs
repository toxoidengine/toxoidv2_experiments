use toxoid_api::*;
use crate::components::*;
use rand::{Rng, thread_rng};

// TODO: Move this config ECS singleton
pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

pub fn get_random(max: u32) -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(0..max)
}

pub fn aabb(a: &Position, a2: &Size, b: &Position, b2: &Size) -> bool {
    a.get_x() + a2.get_width() > b.get_x() &&
    a.get_x() < b.get_x() + b2.get_width() &&
    a.get_y() + a2.get_height() > b.get_y() &&
    a.get_y() < b.get_y() + b2.get_height()
}

pub fn create_snake() {
    let mut head_entity = Entity::new(None);
    head_entity.add::<Head>();
    head_entity.add::<Position>();
    head_entity.add::<Size>();
    head_entity.add::<Color>();

    let mut pos = head_entity.get::<Position>();
    pos.set_x((SCREEN_WIDTH - 100) / 2);
    pos.set_y((SCREEN_HEIGHT - 100) / 2);

    let mut size = head_entity.get::<Size>();
    size.set_width(50);
    size.set_height(50);

    let mut color = head_entity.get::<Color>();
    color.set_r(0.0);
    color.set_g(1.0);
    color.set_b(0.0);
    color.set_a(1.0);

    let mut tails = World::get_singleton::<Tails>();
    tails.set_entities(vec![head_entity.get_id()]);
}

pub fn create_new_head() -> Entity {
    let mut new_head_entity = Entity::new(None);
    new_head_entity.add::<Head>();
    new_head_entity.add::<Position>();
    new_head_entity.add::<Size>();
    new_head_entity.add::<Color>();
    
    let mut size = new_head_entity.get::<Size>();
    size.set_width(50);
    size.set_height(50);

    let mut pos = new_head_entity.get::<Position>();
    pos.set_x(0);
    pos.set_y(0);

    let mut color = new_head_entity.get::<Color>();
    color.set_r(0.0);
    color.set_g(1.0);
    color.set_b(0.0);
    color.set_a(1.0);

    new_head_entity
}

pub fn create_food() {
    let mut food = Entity::new(Some(EntityDesc {
        name: Some("Food".to_string())
    }));
    food.add::<Food>();
    food.add::<Position>();
    food.add::<Size>();
    food.add::<Color>();

    let mut pos = food.get::<Position>();
    let grid_size = 50;
    pos.set_x(get_random((SCREEN_WIDTH - 100) / grid_size) * grid_size);
    pos.set_y(get_random((SCREEN_HEIGHT - 100) / grid_size) * grid_size);
    // pos.set_x(100);
    // pos.set_y(100);

    let mut size = food.get::<Size>();
    size.set_width(50);
    size.set_height(50);

    let mut color = food.get::<Color>();
    color.set_r(1.0);
    color.set_g(0.0);
    color.set_b(0.0);
    color.set_a(1.0);

    let mut food_entity = World::get_singleton::<FoodEntity>();
    food_entity.set_entity(food.get_id());
}

pub fn init() {
    // Initialize Tails singleton
    let mut tails = World::get_singleton::<Tails>();
    tails.set_max_length(777);
    tails.set_entities(vec![]);
    println!("Tails singleton again: {:?}", tails.get_max_length());
    let mut tails = World::get_singleton::<Tails>();
    println!("Tails singleton max length again: {:?}", tails.get_max_length());
    // create_snake();
    // create_food();
}