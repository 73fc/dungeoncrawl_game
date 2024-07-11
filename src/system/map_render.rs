use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_patch = DrawBatch::new();
    let offset = Point::new(camera.left_x, camera.top_y);
    draw_patch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                draw_patch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }
    draw_patch.submit(0).expect("Batch error");
}
