use std::thread;
#[cfg(not(target_arch = "wasm32"))]
fn game_loop(delta_time: f32) {
    println!("delta_time: {}", delta_time);
    toxoid_host::toxoid_progress(delta_time);
}

// TODO: Make this Emscripten specific flag, not just WASM32 when we merge toxoid_bootstrap and toxoid_host
#[cfg(target_arch = "wasm32")]
fn game_loop() {
    // #[cfg(target_arch = "wasm32")]
    // {
    //     #[cfg(target_os = "emscripten")]
    //     emscripten::start_loop(game_loop);
    // }
}

// TODO: Re-add this when we implement Flecs phases.
// For now just run the game loop with the render loop in Sokol's loop.
pub fn init() {
    // Spawn a thread to run the game loop
    thread::spawn(move || {
        // 16 ms time step
        let mut last_time = std::time::Instant::now();
        let mut delta_time;
        loop {
            let current_time = std::time::Instant::now();
            delta_time = current_time.duration_since(last_time).as_secs_f32();
            last_time = current_time;
            game_loop(delta_time);
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    });
}