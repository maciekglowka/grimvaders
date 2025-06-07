use crate::globals::{BACKGROUND_Z, SPRITE_OFFSET, TILE_SIZE, TILE_Z};
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
    // TODO verify multipliers
    BOARD_H as i32 - 8 * p.y + BOARD_W as i32 - 3 * p.x
}

pub(super) fn tile_to_sprite(p: Position) -> Vector2f {
    tile_to_world(p) + SPRITE_OFFSET
}

pub(super) fn world_to_tile(v: Vector2f) -> Position {
    Position::new(
        ((v.x + 2.0 * v.y) / TILE_SIZE - 1.0).floor() as i32,
        ((2.0 * v.y - v.x) / TILE_SIZE).floor() as i32,
    )
}

pub fn is_mouse_over(origin: Vector2f, size: Vector2f, state: &crate::input::InputState) -> bool {
    let v = state.mouse_world_position;
    v.x >= origin.x && v.y >= origin.y && v.x <= origin.x + size.x && v.y <= origin.y + size.y
}

pub fn draw_background(context: &mut Context) {
    let bounds = get_viewport_bounds(context);
    let u = (bounds.1.x - bounds.0.x) / TILE_SIZE;
    let v = (bounds.1.y - bounds.0.y) / TILE_SIZE;

    let _ = context.graphics.draw_mesh(
        "background",
        &[
            bounds.0,
            Vector2f::new(bounds.1.x, bounds.0.y),
            bounds.1,
            Vector2f::new(bounds.0.x, bounds.1.y),
        ],
        &[
            Vector2f::ZERO,
            Vector2f::new(u, 0.),
            Vector2f::new(u, v),
            Vector2f::new(0., v),
        ],
        &[0, 1, 2, 0, 2, 3],
        BACKGROUND_Z,
    );
}
