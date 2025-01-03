use toxoid_sokol::SokolRenderer2D;
use toxoid_render::Renderer2D;
use toxoid_api::*;
use crate::entities;

// Direction enum
enum DirectionEnum {
    Up,
    Down,
    Left,
    Right
}

pub fn init() {    
    unsafe {
        toxoid_host::QUERY_TRAMPOLINE = Some(toxoid_runtime::query_trampoline);
    }

    // Movement System
    System::dsl("Head, Player, Position", Some(10), |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let mut pos = entity.get::<Position>();
            let direction = World::get_singleton::<Direction>();
            match direction.get_direction() {
                d if d == DirectionEnum::Up as u8 => if pos.get_y() >= 50 { pos.set_y(pos.get_y() - 50) },
                d if d == DirectionEnum::Down as u8 => if pos.get_y() <= 500 { pos.set_y(pos.get_y() + 50) },
                d if d == DirectionEnum::Left as u8 => if pos.get_x() >= 50 { pos.set_x(pos.get_x() - 50) },
                d if d == DirectionEnum::Right as u8 => if pos.get_x() <= 700 { pos.set_x(pos.get_x() + 50) },
                _ => {}
            }
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

    // Rendering System
    System::dsl("Head, Player, Position", None, |iter| {
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