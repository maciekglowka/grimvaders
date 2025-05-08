use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::{
    components::{Position, Tile},
    get_entity_at,
    globals::{BOARD_H, BOARD_W},
    World,
};

use crate::{
    draw::units::draw_entity_description,
    globals::{TILE_SIZE, TILE_Z},
    input::InputState,
    utils::{tile_to_world, world_to_tile},
};

pub(super) fn draw_board_description(
    world: &World,
    input_state: &InputState,
    context: &mut Context,
) {
    let tile = world_to_tile(input_state.mouse_world_position);
    if let Some(entity) = get_entity_at(world, tile) {
        draw_entity_description(entity, world, context);
    }
}

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
