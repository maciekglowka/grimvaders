use wunderkammer::prelude::*;

use crate::{commands::RemoveUnit, components::ValueDefault, GameEnv, World};

pub(crate) fn handle_killed(env: &mut GameEnv) -> bool {
    let mut removed = false;
    let entities = query_iter!(env.world, With(killed))
        .map(|(e, _)| e)
        .collect::<Vec<_>>();
    for entity in entities {
        env.scheduler.send(RemoveUnit(entity));
        removed = true;
    }
    removed
}

pub(crate) fn reset_trigger_limits(world: &mut World) {
    query_execute_mut!(world, With(trigger_limit), |_, limit: &mut ValueDefault| {
        limit.restore();
    });
}
