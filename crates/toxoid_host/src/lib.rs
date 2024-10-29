
#[cfg(not(target_arch = "wasm32"))]
fn game_loop(delta_time: f32) {
    println!("delta_time: {}", delta_time);
}

#[cfg(target_arch = "wasm32")]
fn bootstrap() {
    // #[cfg(target_arch = "wasm32")]
    // {
    //     #[cfg(target_os = "emscripten")]
    //     emscripten::start_loop(game_loop);
    // }
}

#[cfg(not(target_arch = "wasm32"))]
fn bootstrap() {
    // Spawn a thread to run the game loop
    std::thread::spawn(move || {
        // 16 ms time step
        // Get the last time
        let mut last_time = std::time::Instant::now();
        // Initialize the delta time
        let mut delta_time;
        // Loop indefinitely
        loop {
            // Calculate the delta time
            let current_time = std::time::Instant::now();
            delta_time = current_time.duration_since(last_time).as_secs_f32();
            last_time = current_time;
            
            // Run the game loop
            game_loop(delta_time);
            
            // Sleep for the remaining time to maintain the 16 ms time step
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    });

    // Load WASM component / guest WASM / gameplay script
    toxoid_wasm_runtime::load_wasm_component("guest.wasm").unwrap();
}

pub fn init() {
    bootstrap();
    loop {}
}