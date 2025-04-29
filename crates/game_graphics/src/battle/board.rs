use rogalik::prelude::*;

use game_logic::{
    components::Position,
    globals::{BOARD_H, BOARD_W},
};

use crate::{
    globals::{TILE_SIZE, TILE_Z},
    utils::tile_to_world,
};

pub(super) fn draw_board(context: &mut Context) {
    for y in 0..BOARD_H {
        for x in 0..BOARD_W {
            let _ = context.graphics.draw_atlas_sprite(
                "sprites",
                5,
                tile_to_world(Position::new(x as i32, y as i32)),
                TILE_Z,
                Vector2f::splat(TILE_SIZE),
                SpriteParams::default(),
            );
        }
    }
}
