use toxoid_sokol::SokolRenderer2D;
use toxoid_render::Renderer2D;
use toxoid_api::*;
use crate::entities;
use rand::{Rng, thread_rng};

// Direction enum
enum DirectionEnum {
    Up,
    Down,
    Left,
    Right
}

// TODO: Move this config ECS singleton
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn aabb(a: &Position, a2: &Size, b: &Position, b2: &Size) -> bool {
    a.get_x() + a2.get_width() > b.get_x() &&
    a.get_x() < b.get_x() + b2.get_width() &&
    a.get_y() + a2.get_height() > b.get_y() &&
    a.get_y() < b.get_y() + b2.get_height()
}

fn get_random(max: u32) -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(0..max)
}

pub fn init() {    
    unsafe {
        toxoid_host::QUERY_TRAMPOLINE = Some(toxoid_runtime::query_trampoline);
    }
    
    // Movement System
    System::dsl("Head, Position", Some(10), |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let mut tails = World::get_singleton::<Tails>();
            let mut tails_entities = tails.get_entities();

            // Get current position of head
            let mut pos = entity.get::<Position>();
            let mut size = entity.get::<Size>();
            let current_x = pos.get_x();
            let current_y = pos.get_y();

            // Get current position of food
            let mut food_singleton = World::get_singleton::<FoodEntity>();
            let food_entity_id = food_singleton.get_entity();
            let mut food_entity = World::get_entity(food_entity_id);
            let mut food_pos = food_entity.get::<Position>();
            let mut food_size = food_entity.get::<Size>();

            // Check if head and food are colliding
            if aabb(&pos, &size, &food_pos, &food_size) {
                // Set (respawning) random position for food   
                let grid_size = 50;
                let mut food_pos = food_entity.get::<Position>();
                let mut food_size = food_entity.get::<Size>();
                food_pos.set_x(get_random((SCREEN_WIDTH - 100) / grid_size) * grid_size);
                food_pos.set_y(get_random((SCREEN_HEIGHT - 100) / grid_size) * grid_size);
                // Increase the tail length
                tails.set_max_length(tails.get_max_length() + 1);
            }

            // Create new head entity on every movement tick
            let mut new_head_entity = entities::create_new_head();
            let mut pos = new_head_entity.get::<Position>();
            new_head_entity.add::<Head>();
            new_head_entity.remove::<Tail>();
            pos.set_x(current_x);
            pos.set_y(current_y);
            tails_entities.push(new_head_entity.get_id());

            println!("Tails entities: {:?}", tails_entities.clone());

            let direction = World::get_singleton::<Direction>();
            let screen_y_bounds = SCREEN_HEIGHT - 100;
            let screen_x_bounds = SCREEN_WIDTH - 100;
            match direction.get_direction() {
                d if d == DirectionEnum::Up as u8 => if current_y >= 50 { pos.set_y(current_y - 50) },
                d if d == DirectionEnum::Down as u8 => if current_y <= screen_y_bounds { pos.set_y(current_y + 50) },
                d if d == DirectionEnum::Left as u8 => if current_x >= 50 { pos.set_x(current_x - 50) },
                d if d == DirectionEnum::Right as u8 => if current_x <= screen_x_bounds { pos.set_x(current_x + 50) },
                _ => {}
            }
            entity.remove::<Head>(); 
            entity.add::<Tail>();

            // Remove the last tail entity
            println!("Tails entities: {:?}", tails.get_max_length());
            if tails_entities.len() > tails.get_max_length() as usize {
                println!("Removing last tail entity");
                let last_tail_entity_id = tails_entities.remove(0);
                let mut last_tail_entity = World::get_entity(last_tail_entity_id);
                last_tail_entity.remove::<Tail>();
                tails.set_entities(tails_entities.clone());
                World::remove_entity(last_tail_entity_id);
            }
        });
    })
        .build();

        System::dsl("Tail, Position", Some(10), |iter| {
            iter.entities().iter_mut().for_each(|entity| {
                let mut pos = entity.get::<Position>();
                // println!("Position X: {:?}", pos.get_x());
                // println!("Position Y: {:?}", pos.get_y());
            });
        })
            .build();
    
    // Input System
    System::dsl("KeyboardInput", None, |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let mut direction = World::get_singleton::<Direction>();
            let mut keyboard_input = entity.get::<KeyboardInput>();
            if keyboard_input.get_up() {
                direction.set_direction(DirectionEnum::Up as u8);
                keyboard_input.set_up(false);
            }
            if keyboard_input.get_down() {
                direction.set_direction(DirectionEnum::Down as u8);
                keyboard_input.set_down(false);
            }
            if keyboard_input.get_left() {
                direction.set_direction(DirectionEnum::Left as u8);
                keyboard_input.set_left(false);
            }
            if keyboard_input.get_right() {
                direction.set_direction(DirectionEnum::Right as u8);
                keyboard_input.set_right(false);
            }
        });
    })
        .build();

    // Rendering Food
    System::dsl("Food, Position, Size, Color", None, |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let mut pos = entity.get::<Position>();
            let pos = entity.get::<Position>();
            let size = entity.get::<Size>();
            let color = entity.get::<Color>();
            SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
        });
    })
        .build();

    // Rendering Tail
    System::dsl("Tail, Position, Size, Color", None, |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let mut pos = entity.get::<Position>();
            let pos = entity.get::<Position>();
            let size = entity.get::<Size>();
            let color = entity.get::<Color>();
            SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
        });
    })
        .build();

    // Rendering Head
    System::dsl("Head, Position, Size, Color", None, |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let mut pos = entity.get::<Position>();
            let pos = entity.get::<Position>();
            let size = entity.get::<Size>();
            let color = entity.get::<Color>();
            SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
        });
    })
        .build();

    // // Rendering System
    // System::dsl("Rect, Position, Size, Color, Renderable", None, |iter| {
    //     iter.entities().iter_mut().for_each(|entity| {
    //         let pos = entity.get::<Position>();
    //         let size = entity.get::<Size>();
    //         let color = entity.get::<Color>();
    //         SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
    //     });
    // })
    //     .build();
}