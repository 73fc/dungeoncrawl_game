use crate::prelude::*;

#[system]
#[read_component(FiledOfView)]
#[read_component(Player)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_patch = DrawBatch::new();
    let offset = Point::new(camera.left_x, camera.top_y);
    let mut fov = <&FiledOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    draw_patch.target(0);
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let idx = map_idx(x, y);
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx]
            {
                let tint = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARK_GRAY
                };
                match map.tiles[idx] {
                    TileType::Floor => {
                        draw_patch.set(pt - offset, ColorPair::new(tint, BLACK), to_cp437('.'));
                    }
                    TileType::Wall => {
                        draw_patch.set(pt - offset, ColorPair::new(tint, BLACK), to_cp437('#'));
                    }
                };
            }
        }
    }
    draw_patch.submit(0).expect("Batch error");
}
