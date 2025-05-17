use wunderkammer::prelude::*;

use crate::{battle::player::remove_player_from_board, World};

pub(crate) fn handle_killed(world: &mut World) {
    let entities = query_iter!(world, With(killed))
        .map(|(e, _)| e)
        .collect::<Vec<_>>();
    for entity in entities {
        world.components.killed.remove(entity);
        if world.components.player.get(entity).is_some() {
            remove_player_from_board(entity, world);
        } else {
            world.despawn(entity);
        }
    }
}
