use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FiledOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut draw_path = DrawBatch::new();
    draw_path.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);
    let mut fov = <&FiledOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            draw_path.set(*pos - offset, render.color, render.glyph);
        });
    draw_path.submit(5000).expect("Batch error");
}
