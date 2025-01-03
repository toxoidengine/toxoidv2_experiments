use toxoid_api::*;
use toxoid_api::components::*;
use toxoid_host::toxoid_progress;

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

// TODO: Move this config ECS singleton
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

use rand::{Rng, thread_rng};

fn get_random(max: u32) -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(0..max)
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
    tails.set_max_length(1);
    tails.set_entities(vec![]);

    create_snake();
    create_food();
}