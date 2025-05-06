use rogalik::math::vectors::Vector2i;
use std::collections::HashMap;
use wunderkammer::prelude::*;

use crate::{commands, globals::HAND_SIZE, GameEnv, World};

pub(super) fn player_battle_init(world: &mut World) {
    crate::player::reset_deck(world);
}

pub(super) fn player_battle_exit(world: &mut World) {
    let placed = query_iter!(world.0, With(position, player))
        .map(|(e, _, _)| e)
        .collect::<Vec<_>>();

    for entity in placed {
        world.0.components.position.remove(entity);
        world.0.resources.player_data.discard.push(entity);
    }
}

pub(super) fn player_next_turn(env: &mut GameEnv) {
    crate::player::draw_hand(&mut env.world);
    env.world.0.resources.player_data.food += 5;
}
