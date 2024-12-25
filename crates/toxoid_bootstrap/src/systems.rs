// pub fn render_rect_system(iter: &mut Iter) {
//     let entities = iter.entities();
//     components
//         .enumerate()
//         .for_each(|(i, (color, size, pos))| {
//             let entity = &mut entities[i];
//             let mut rt_entity = entity.children().get_mut(0).unwrap();
//             let rt = rt_entity.get::<RenderTarget>().get_render_target().ptr as *mut SokolRenderTarget;
//             let rt_box = unsafe { Box::from_raw(rt) };
//             let rt_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(rt_box as Box<dyn toxoid_render::RenderTarget>));

//             unsafe {
//                 SokolRenderer2D::begin_rt(&rt_trait_object, size.get_width() as f32, size.get_height() as f32);
//                 SokolRenderer2D::draw_filled_rect(pos, size, color);
//                 SokolRenderer2D::end_rt()
//             }     

//             // Add renderable to render target we just blitted the sprite to, which which be drawn on the main canvas buffer
//             rt_entity.add::<Renderable>();
//             // TODO: Bug: render_rect_system not blitting correctly the first timee maybe? So this has to be commented out to make it every frame
//             // Remove renderable from sprite after blitting 
//             // entity.remove::<Renderable>();
//         });
// }

use toxoid_sokol::SokolRenderer2D;
use toxoid_render::Renderer2D;
use toxoid_api::*;

pub fn init() {    
    System::dsl("Rect, Position, Size, Color, Renderable", |iter| {
        println!("Hello render rect system!");
        iter
            .entities()
            .iter()
            .for_each(|entity| {
                let mut rect = entity.get::<Rect>();
                let mut pos = entity.get::<Position>();
                let mut size = entity.get::<Size>();
                let mut color = entity.get::<Color>();
                SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
            });
    })
    .build();
}