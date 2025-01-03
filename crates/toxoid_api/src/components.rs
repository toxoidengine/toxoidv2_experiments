use crate::*;

component! {
    // Singletons
    KeyboardInput {
        up: bool,
        down: bool,
        left: bool,
        right: bool, 
    },
    Direction {
        direction: u8
    },
    
    // Components
    Position {
        x: u32,
        y: u32
    },
    Size {
        width: u32,
        height: u32
    },
    Color {
        r: f32,
        g: f32,
        b: f32,
        a: f32
    },
    Stats {
        score: u32,
        high_score: u32,
        tail_length: u32
    },

    // Tags
    Player {
        foo: bool
    },
    Food {
        foo: bool
    },
    Head {
        foo: bool
    },
    Tail {
        entities: Vec::<u64>
    },
    Rect {
        foo: bool
    },
    Renderable {
        foo: bool
    }
}

pub fn init() {
    // Register singletons
    KeyboardInput::register();
    Direction::register();

    // Register components
    Position::register();
    Size::register();
    Color::register();
    Stats::register();

    // Register tags
    Player::register();
    Food::register();
    Head::register();
    Tail::register();
    Rect::register();
    Renderable::register();

    // Add singletons
    World::add_singleton::<KeyboardInput>();
    World::add_singleton::<Direction>();
}