mod renderer;
mod game_loop;
mod systems;
mod entities;
mod watch;

pub fn init() {
    // game_loop::init();
    systems::init();
    entities::init();
    watch::init();
    renderer::init();
}