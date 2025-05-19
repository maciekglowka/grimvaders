use crate::globals::{SPRITE_OFFSET, TILE_SIZE, TILE_Z};
use rogalik::prelude::*;

use game_logic::{
    components::Position,
    globals::{BOARD_H, BOARD_W},
};

pub(crate) fn get_viewport_bounds(context: &Context) -> (Vector2f, Vector2f) {
    let camera = context.graphics.get_current_camera();
    camera.get_bounds()
}

pub fn tile_to_world(p: Position) -> Vector2f {
    Vector2f::new(
        0.5 * TILE_SIZE * p.x as f32 - 0.5 * TILE_SIZE * p.y as f32,
        0.25 * TILE_SIZE * p.y as f32 + 0.25 * TILE_SIZE * p.x as f32,
    )
}

pub(super) fn get_z_offset(p: Position) -> i32 {
    BOARD_H as i32 - p.y + BOARD_W as i32 - p.x
}

pub(super) fn tile_to_sprite(p: Position) -> Vector2f {
    tile_to_world(p) + SPRITE_OFFSET
}

pub(super) fn world_to_tile(v: Vector2f) -> Position {
    // Position::new(
    //     (v.x / TILE_SIZE).floor() as i32,
    //     (v.y / TILE_SIZE).floor() as i32,
    // )
    Position::new(
        ((v.x + 2.0 * v.y) / TILE_SIZE - 1.0).floor() as i32,
        ((2.0 * v.y - v.x) / TILE_SIZE).floor() as i32,
    )
}

pub fn is_mouse_over(origin: Vector2f, size: Vector2f, state: &crate::input::InputState) -> bool {
    let v = state.mouse_world_position;
    v.x >= origin.x && v.y >= origin.y && v.x <= origin.x + size.x && v.y <= origin.y + size.y
}
