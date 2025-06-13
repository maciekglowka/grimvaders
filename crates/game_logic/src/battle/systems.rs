use wunderkammer::prelude::*;

use crate::{
    commands::{check_trigger_limit, use_trigger_limit, RemoveUnit, RuneCommand},
    components::ValueDefault,
    scripting::run_command_script,
    GameEnv, World,
};

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

pub(crate) fn handle_on_fight(env: &mut GameEnv) -> bool {
    let Some(entity) = env.world.resources.battle_state.on_fight_queue.pop_front() else {
        return false;
    };

    if check_trigger_limit(entity, &env.world).is_err() {
        return true;
    }

    let Some(script) = env.world.components.on_fight.get(entity) else {
        return true;
    };
    let script = script.to_string();

    if let Some(commands) =
        run_command_script(&script, entity.into(), &mut env.world, RuneCommand::None)
    {
        if !commands.is_empty() {
            use_trigger_limit(entity, &mut env.world);
        }
        for c in commands {
            c.scheduler_send(&mut env.scheduler);
        }
    }

    true
}
