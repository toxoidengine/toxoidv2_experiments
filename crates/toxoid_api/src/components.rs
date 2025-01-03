use crate::*;

component! {
    // Singletons
    KeyboardInput {
        up: bool,
        down: bool,
        left: bool,
        right: bool, 
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

    // Tags
    // TODO: Implement tags so we don't have to use 
    // these placeholder fields
    Rect {
        foo: bool
    },
    Renderable {
        foo: bool
    },

    // Snake (TODO: Remove from engine source code)
    Direction {
        direction: u8
    },
    Stats {
        score: u32,
        high_score: u32,
        tail_length: u32
    },
    Tails {
        max_length: u32,
        entities: Vec::<u64>
    },
    Head {
        foo: bool
    },
    Tail {
        foo: bool
    },
    Player {
        foo: bool
    },
    Food {
        foo: bool
    },
    FoodEntity {
        entity: u64
    }
}

pub fn init() {
    // Register singletons
    KeyboardInput::register();

    // Register components
    Position::register();
    Size::register();
    Color::register();

    // Register tags
    Rect::register();
    Renderable::register();

    // Add singletons
    World::add_singleton::<KeyboardInput>();
    
    // Snake components (TODO: Remove from source code)
    Direction::register();
    Stats::register();
    Tails::register();
    Tail::register();
    Player::register();
    Food::register();
    Head::register();
    FoodEntity::register();
    World::add_singleton::<FoodEntity>();
    World::add_singleton::<Direction>();
    World::add_singleton::<Tails>();
}