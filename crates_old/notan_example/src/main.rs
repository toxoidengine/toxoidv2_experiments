use notan::prelude::*;
use notan::draw::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init()
        .draw(draw)
        .add_config(DrawConfig)
        .build()
}

fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));
    gfx.render(&draw);
}