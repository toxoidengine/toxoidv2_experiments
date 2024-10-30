use interprocess::local_socket::{ListenerOptions, Stream, prelude::*};
use std::io::{BufReader, prelude::*};
use std::thread;

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

    // Start a thread to listen for IPC messages
    thread::spawn(move || {
        let listener = ListenerOptions::new().name("example.sock").create_sync().unwrap();
        for conn in listener.incoming().filter_map(Result::ok) {
            let mut conn = BufReader::new(conn);
            let mut buffer = String::new();
            conn.read_line(&mut buffer).unwrap();
            if buffer.trim() == "Reload WASM script" {
                println!("Reloading WASM component...");
                toxoid_wasm_runtime::load_wasm_component("guest.wasm").unwrap();
            }
        }
    });

    // Initial load of the WASM component
    toxoid_wasm_runtime::load_wasm_component("guest.wasm").unwrap();
}

pub fn init() {
    bootstrap();
    loop {}
}