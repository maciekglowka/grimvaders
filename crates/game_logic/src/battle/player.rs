use rogalik::math::vectors::Vector2i;
use std::collections::HashMap;
use wunderkammer::prelude::*;

use crate::{commands, globals::HAND_SIZE, GameEnv, World};

pub(super) fn player_battle_init(world: &mut World) {
    crate::player::reset_deck(world);

    for _ in 0..HAND_SIZE {
        if let Some(entity) = world.0.resources.player_data.draw.pop_front() {
            world.0.resources.player_data.hand.push(entity);
        }
    }
}

pub(super) fn player_battle_exit(world: &mut World) {}

pub(super) fn player_next_turn(env: &mut GameEnv) {}
