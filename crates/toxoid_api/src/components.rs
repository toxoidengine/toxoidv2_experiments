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
    // TODO: Replace these with tags that have no fields
    Rect {
        foo: bool
    },

    // State Representation Tags
    // TODO: Replace these with tags that have no fields
    Renderable {
        foo: bool
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
}