
use toxoid_sokol::{SokolRenderer2D, sokol::app::frame_duration};
use toxoid_render::Renderer2D;

#[no_mangle]
extern "C" fn sokol_init() {
    // Initialization code for Sokol
    // println!("Sokol initialized");
    toxoid_sokol::sokol_init();
}

#[no_mangle]
extern "C" fn sokol_frame() {
    // Frame update code for Sokol
    SokolRenderer2D::begin();
    let delta_time = frame_duration();
    toxoid_host::toxoid_progress(delta_time as f32);
    SokolRenderer2D::end();
}

#[no_mangle]
extern "C" fn sokol_cleanup() {
    // Cleanup code for Sokol
    // println!("Sokol cleanup");
}

// use toxoid_api::*;
// component! {
//     Velocity {
//         x: u32,
//         y: u32
//     }
// }

pub fn init() {
    toxoid_sokol::init(sokol_init, sokol_frame, crate::input::sokol_event);
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

    // let pass_action = sg::PassAction::new();
    // let (window_width, window_height) = SokolRenderer2D::window_size();
    // // Begin recording draw commands for a frame buffer of size (width, height).
    // sgp_begin(window_width as i32, window_height as i32);
    // // Set frame buffer drawing region to (0,0,width,height).
    // sgp_viewport(0, 0, window_width as i32, window_height as i32);
    // // Set drawing coordinate space to (left=0, right=width, top=0, bottom=height).
    // sgp_project(0.0, window_width as f32, 0.0, window_height as f32);
    // // Clear the frame buffer.
    // sgp_set_color(1., 1., 1., 1.);
    // sgp_clear();
    // // Draw a filled rectangle
    // let scale_factor = window_width as f32 / crate::GAME_WIDTH as f32;
    // sgp_reset_color();
    // sgp_set_color(1., 0., 0., 1.);
    // sgp_draw_filled_rect(0 as f32 * scale_factor, 0 as f32 * scale_factor, 100 as f32 * scale_factor, 100 as f32 * scale_factor);
    // // Begin a render pass.
    // sg::begin_pass(&sg::Pass {
    //     action: pass_action,
    //     swapchain: sglue::swapchain(),
    //     ..Default::default()
    // });
    // // Dispatch all draw commands to Sokol GFX.
    // sgp_flush(); 
    // // Finish a draw command queue, clearing it.
    // sgp_end();
    // // End render pass.
    // sg::end_pass();
    // // Commit Sokol render.
    // sg::commit();
}