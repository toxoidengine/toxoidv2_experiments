use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use std::thread;
// use toxoid_api::*;

// TODO: Make this configurable via ENV variable
const HOST_ADDRESS: &str = "127.0.0.1:7878";
// TODO: Make this configurable via ENV variable
const GUEST_WASM_PATH: &str = "guest.wasm";

#[cfg(not(target_arch = "wasm32"))]
fn game_loop(delta_time: f32) {
    // println!("delta_time: {}", delta_time);
    toxoid_host::toxoid_progress(delta_time);
}

// TODO: Make this Emscripten specific flag, not just WASM32 when we merge toxoid_bootstrap and toxoid_host
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

        for stream in listener
            .incoming()
            .filter_map(Result::ok)
        {
            let mut conn = BufReader::new(stream);
            let mut buffer = String::new();
            conn
                .read_line(&mut buffer)
                .unwrap();
            if buffer.contains("reload") {
                println!("Reloading WASM component...");
                toxoid_runtime::load_wasm_component(GUEST_WASM_PATH)
                    .unwrap_or_else(|e| println!("Failed to reload WASM component: {}", e));
            }
        }
    });

    // Initial load of the main WASM component / game engine script
    if std::path::Path::new(GUEST_WASM_PATH).exists() {
        println!("Loading WASM component...");
        toxoid_runtime::load_wasm_component(GUEST_WASM_PATH)
            .unwrap_or_else(|e| println!("Failed to load WASM component: {}", e));
    } else {
        println!("WASM component not found at {}, modify the guest script source file or use `toxoid_cli build` to generate it", GUEST_WASM_PATH);
    }
}

extern "C" fn sokol_init() {
    // Initialization code for Sokol
    // println!("Sokol initialized");
    toxoid_sokol::init(sokol_init, sokol_frame, sokol_event);
    // use toxoid_api::*;
    // use toxoid_api::components::*;
    // use toxoid_sokol::SokolRenderer2D;
    // use toxoid_render::Renderer2D;
    // let mut entity = Entity::new(Some(EntityDesc {
    //     name: Some("Test entity".to_string())
    // }));
    // entity.add::<Rect>();
    // entity.add::<Position>();
    // entity.add::<Size>();
    // entity.add::<Color>();
    // let mut size = entity.get::<Size>();
    // size.set_width(100);
    // size.set_height(100);
    // let mut color = entity.get::<Color>();
    // color.set_r(1.0);
    // color.set_g(1.0);
    // color.set_b(1.0);
    // color.set_a(1.0);
    
    // System::dsl("Rect, Position, Size, Color", |iter| {
    //     iter
    //         .entities()
    //         .iter()
    //         .for_each(|entity| {
    //             let mut rect = entity.get::<Rect>();
    //             let mut pos = entity.get::<Position>();
    //             let mut size = entity.get::<Size>();
    //             let mut color = entity.get::<Color>();
    //             println!("Hello render rect system!");
    //             // SokolRenderer2D::begin();
    //             // SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
    //             // let renderer_2d = &mut *toxoid_sokol::RENDERER_2D.lock().unwrap();
    //             // renderer_2d.end();
    //         });
    // })
    // .build();
}

extern "C" fn sokol_frame() {
    // Frame update code for Sokol
    // println!("Sokol frame updated");
}

extern "C" fn sokol_event(event: *const toxoid_sokol::sokol::app::Event) {
    // Event handling code for Sokol
    // println!("Sokol event received");
}

// use toxoid_api::*;
// component! {
//     Velocity {
//         x: u32,
//         y: u32
//     }
// }

pub fn init() {
    // bootstrap();
    toxoid_sokol::init(sokol_init, sokol_frame, sokol_event);
    
    // println!("{}", Velocity::get_name());
    // let entity = Entity::new(Some(EntityDesc {
    //     name: Some("Test entity".to_string())
    // }));
    // // Create entity with Velocity component
    // let mut entity = Entity::named("Test");
    // entity.add::<Velocity>();
    // let mut velocity = entity.get::<Velocity>();
    // velocity.set_x(777);
    // velocity.set_y(999);
    // // Print x and y of velocity component
    // println!("Velocity 1 - X: {}, Y: {}", velocity.get_x(), velocity.get_y());
    // let mut entity = Entity::new(None);
    // entity.add::<Velocity>();
    // let mut velocity = entity.get::<Velocity>();
    // velocity.set_x(555);
    // velocity.set_y(888);
    // // Print x and y of velocity component
    // println!("Velocity 2 - X: {}, Y: {}", velocity.get_x(), velocity.get_y());
    // System::dsl("Velocity($this)", |iter| {
    //     iter
    //         .entities()
    //         .iter()
    //         .for_each(|entity| {
    //             let mut velocity = entity.get::<Velocity>();
    //             let x = velocity.get_x();
    //             let y = velocity.get_y();
    //             velocity.set_x(x + 1);
    //             velocity.set_y(y + 1);
    //             println!("Entity: {}", entity.get_id());
    //             println!("Velocity -  X: {}, Y: {}", x, y);
    //         });
    // })
    // .build();
    
    loop {}
}