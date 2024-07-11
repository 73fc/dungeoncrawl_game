use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_path = DrawBatch::new();
    draw_path.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_path.set(*pos - offset, render.color, render.glyph);
        });
    draw_path.submit(5000).expect("Batch error");
}
