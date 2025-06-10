use wunderkammer::prelude::*;

use crate::{GameEnv, World};

pub(super) fn player_battle_init(world: &mut World) {
    crate::player::reset_deck(world);
    world.resources.player_data.food = 0;
}

pub(super) fn player_battle_exit(world: &mut World) {
    let placed = query_iter!(world.0, With(position, player))
        .map(|(e, _, _)| e)
        .collect::<Vec<_>>();

    for entity in placed {
        remove_player_from_board(entity, world);
    }
}

pub(super) fn player_next_turn(env: &mut GameEnv) {
    crate::player::draw_hand(&mut env.world);
    env.world.0.resources.player_data.food += 4;
}

pub(crate) fn reset_player(entity: Entity, world: &mut World) {
    if let Some(health) = world.components.health.get_mut(entity) {
        health.restore();
    }
    world.components.killed.remove(entity);
}

pub(crate) fn remove_player_from_board(entity: Entity, world: &mut World) {
    world.components.position.remove(entity);
    reset_player(entity, world);
    world.resources.player_data.discard.push(entity);
}
