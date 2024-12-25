mod renderer;
mod game_loop;
mod systems;
mod entities;
mod watch;

pub fn init() {
    // game_loop::init();
    entities::init();
    systems::init();
    watch::init();
    renderer::init();
}