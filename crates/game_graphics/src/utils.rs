use crate::globals::{SPRITE_OFFSET, TILE_SIZE};
use rogalik::prelude::*;

use game_logic::components::Position;

pub(crate) fn get_viewport_bounds(context: &Context) -> (Vector2f, Vector2f) {
    let camera = context.graphics.get_current_camera();
    camera.get_bounds()
}

// pub(crate) fn move_towards(origin: Vector2f, target: Vector2f, max_delta:
// f32) -> Vector2f {     let a = target - origin;
//     let l = a.len_sq();
//     if l <= max_delta * max_delta || l == 0. {
//         return target;
//     }
//     origin + a / l * max_delta
// }

pub(super) fn tile_to_world(p: Position) -> Vector2f {
    Vector2f::new(TILE_SIZE * p.x as f32, TILE_SIZE * p.y as f32)
}

pub(super) fn tile_to_sprite(p: Position) -> Vector2f {
    tile_to_world(p) + SPRITE_OFFSET
}

pub(super) fn world_to_tile(v: Vector2f) -> Position {
    Position::new(
        (v.x / TILE_SIZE).floor() as i32,
        (v.y / TILE_SIZE).floor() as i32,
    )
}
