use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use std::thread;

// TODO: Make this configurable via ENV variable
const HOST_ADDRESS: &str = "127.0.0.1:7878";
// TODO: Make this configurable via ENV variable
const GUEST_WASM_PATH: &str = "app/host/guest.wasm";

#[cfg(not(target_arch = "wasm32"))]
fn game_loop(delta_time: f32) {
    // println!("delta_time: {}", delta_time);
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

    // Start a thread to listen for TCP messages
    thread::spawn(move || {
        // Define the TCP address and port
        let listener = TcpListener::bind(HOST_ADDRESS).unwrap();
        println!("Listening on {}", HOST_ADDRESS);

        for stream in listener.incoming().filter_map(Result::ok) {
            let mut conn = BufReader::new(stream);
            let mut buffer = String::new();
            conn.read_line(&mut buffer).unwrap();
            if buffer.contains("reload") {
                println!("Reloading WASM component...");
                toxoid_wasm_runtime::load_wasm_component(GUEST_WASM_PATH).unwrap();
            }
        }
    });

    // Initial load of the main WASM component / game engine script
    if std::path::Path::new(GUEST_WASM_PATH).exists() {
        println!("Loading WASM component...");
        toxoid_wasm_runtime::load_wasm_component(GUEST_WASM_PATH).unwrap();
    } else {
        panic!("WASM component not found at {}, modify the guest script source file or use `toxoid_cli build` to generate it", GUEST_WASM_PATH);
    }
}

pub fn init() {
    bootstrap();
    loop {}
}