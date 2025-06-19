use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::{
    components::{Position, Tile},
    get_unit_at,
    globals::{BOARD_H, BOARD_W, MAX_WAVE_H},
    World,
};

use crate::{
    draw::units::draw_entity_description,
    globals::{NPC_TILE_SPRITE, TILE_SIZE, TILE_Z, TOWN_SPRITE},
    input::InputState,
    utils::{get_z_offset, tile_to_world, world_to_tile},
};

pub(super) fn draw_board_description(
    world: &World,
    input_state: &InputState,
    context: &mut Context,
) {
    let tile = world_to_tile(input_state.mouse_world_position);
    if let Some(entity) = get_unit_at(world, tile) {
        draw_entity_description(entity, world, context);
    }
}

pub(super) fn draw_board(world: &World, context: &mut Context) {
    query_execute!(
        world.0,
        With(position, tile),
        |_, p: &Position, t: &Tile| {
            let _ = context.graphics.draw_atlas_sprite(
                "tiles",
                get_tile_sprite(t),
                tile_to_world(*p),
                TILE_Z + get_z_offset(*p),
                Vector2f::splat(TILE_SIZE),
                SpriteParams::default(),
            );
            let _ = context.graphics.draw_atlas_sprite(
                "tiles",
                get_tile_sprite(t) + 1,
                tile_to_world(*p),
                TILE_Z + get_z_offset(*p) + 2,
                Vector2f::splat(TILE_SIZE),
                SpriteParams::default(),
            );
        }
    );

    // Draw Town
    for x in 0..BOARD_W {
        let p = Position::new(x as i32, -1);
        let _ = context.graphics.draw_atlas_sprite(
            "tiles",
            TOWN_SPRITE + x % 2,
            tile_to_world(p),
            TILE_Z + get_z_offset(p),
            Vector2f::splat(TILE_SIZE),
            SpriteParams::default(),
        );
    }

    // Draw NPC zone
    for y in 0..MAX_WAVE_H {
        for x in 0..BOARD_W {
            let p = Position::new(x as i32, (BOARD_H + y) as i32);
            let _ = context.graphics.draw_atlas_sprite(
                "tiles",
                NPC_TILE_SPRITE,
                tile_to_world(p),
                TILE_Z + get_z_offset(p),
                Vector2f::splat(TILE_SIZE),
                SpriteParams::default(),
            );
        }
    }
}

fn get_tile_sprite(tile: &Tile) -> usize {
    match &tile {
        Tile::Plains => 2,
        Tile::Meadow => 4,
        Tile::Field => 6,
        Tile::Forest => 8,
    }
}
