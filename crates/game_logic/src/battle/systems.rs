use wunderkammer::prelude::*;

use crate::{battle::player::remove_player_from_board, commands::RemoveUnit, GameEnv};

pub(crate) fn handle_killed(env: &mut GameEnv) {
    let entities = query_iter!(env.world, With(killed))
        .map(|(e, _)| e)
        .collect::<Vec<_>>();
    for entity in entities {
        env.scheduler.send(RemoveUnit(entity));
    }
}
