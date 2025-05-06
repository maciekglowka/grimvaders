use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::{
    components::{Position, Tile},
    globals::{BOARD_H, BOARD_W},
    World,
};

use crate::{
    globals::{TILE_SIZE, TILE_Z},
    utils::tile_to_world,
};

pub(super) fn draw_board(world: &World, context: &mut Context) {
    query_execute!(
        world.0,
        With(position, tile),
        |_, p: &Position, t: &Tile| {
            let _ = context.graphics.draw_atlas_sprite(
                "sprites",
                get_tile_sprite(t),
                tile_to_world(*p),
                TILE_Z,
                Vector2f::splat(TILE_SIZE),
                SpriteParams::default(),
            );
        }
    );
}

fn get_tile_sprite(tile: &Tile) -> usize {
    match &tile {
        Tile::Plains => 1,
        Tile::Meadow => 5,
        Tile::Field => 98,
        Tile::Forest => 52,
    }
}
