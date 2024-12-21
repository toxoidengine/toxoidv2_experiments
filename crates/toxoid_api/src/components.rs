use crate::*;

component! {
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
    Rect {
        foo: bool
    }
}
