mod renderer;
mod input;
mod systems;
mod entities;
mod watch;
// mod game_loop;

pub fn init() {
    toxoid_api::components::init();
    // entities::init();
    systems::init();
    watch::init();
    renderer::init();
    // game_loop::init();
}
