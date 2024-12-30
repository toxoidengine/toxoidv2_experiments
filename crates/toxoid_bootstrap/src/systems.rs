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
use toxoid_api::components::*;

pub fn init() {    
    unsafe {
        toxoid_host::QUERY_TRAMPOLINE = Some(toxoid_runtime::query_trampoline);
    }
    /*
    TODO:
        Figure out why this is happening -
        error: unresolved identifier 'Rect'
        expr: Rect, Position, Size, Color, Renderable
            > Rect($this),
            Position(0),
            Size(0),
            Color(0),
            Renderable(0)
     */

    let mut entity = Entity::new(Some(EntityDesc {
        name: Some("Test entity".to_string())
    }));
    entity.add::<Rect>();
    entity.add::<Position>();
    entity.add::<Size>();
    entity.add::<Color>();
    entity.add::<Renderable>();
    let mut size = entity.get::<Size>();
    size.set_width(100);
    size.set_height(100);
    let mut color = entity.get::<Color>();
    color.set_r(1.);
    color.set_g(0.);
    color.set_b(0.);
    color.set_a(1.);
    
    System::dsl("Rect, Position, Size, Color, Renderable", None, |iter| {
        iter
            .entities()
            .iter_mut()
            .for_each(|entity| {
                let mut pos = entity.get::<Position>();
                let mut size = entity.get::<Size>();
                let mut color = entity.get::<Color>();
                SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
            });
    })
        .build();

    System::dsl("Rect, Position, Size, Color, Renderable", None, |iter| {
        iter
            .entities()
            .iter_mut()
            .for_each(|entity| {
                let mut pos = entity.get::<Position>();
                let mut size = entity.get::<Size>();
                let keyboard_input = World::get_singleton::<KeyboardInput>();
                if keyboard_input.get_up() && pos.get_y() >= 5 {
                    pos.set_y(pos.get_y() - 5);
                }
                if keyboard_input.get_down() && pos.get_y() <= 500 {
                    pos.set_y(pos.get_y() + 5);
                }
                if keyboard_input.get_left() && pos.get_x() >= 5 {
                    pos.set_x(pos.get_x() - 5);
                }
                if keyboard_input.get_right() && pos.get_x() <= 700 {
                    pos.set_x(pos.get_x() + 5);
                }
            });
    })
        .build();
}