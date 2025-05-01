use rand::prelude::*;
use wunderkammer::prelude::*;

use crate::{
    commands,
    components::Position,
    globals::{BOARD_H, BOARD_W},
    utils::spawn_by_name,
    GameEnv, World,
};

pub(crate) fn slide_waves(env: &mut GameEnv) {
    let mut cmds = query_iter!(env.world.0, With(npc, position))
        .map(|(e, _, p)| commands::MoveUnit(e, Position::new(p.x, p.y - 1)))
        .collect::<Vec<_>>();
    // sort to start moving from the bottom
    // in order to avoid blocking
    cmds.sort_by_key(|a| a.1.y);
    env.scheduler.send_many(cmds);
}

pub(crate) fn next_wave(y: i32, env: &mut GameEnv) {
    env.world.0.resources.battle_state.wave += 1;
    let count = (1 * env.world.0.resources.battle_state.wave).min(BOARD_W as u32);

    let mut rng = thread_rng();
    let mut pool = (0..BOARD_W as i32).collect::<Vec<_>>();

    for _ in 0..count {
        let entity = spawn_by_name("Rat", &mut env.world).unwrap();
        env.world.0.components.npc.insert(entity, ());
        let x = pool.remove(rng.gen_range(0..pool.len()));
        env.scheduler
            .send(commands::SpawnUnit(entity, Position::new(x as i32, y)));
    }
}

pub(crate) fn next_attack(env: &mut GameEnv) -> bool {
    let Some((entity, position)) = next_npc(&env.world) else {
        return false;
    };

    if let Some(target) = next_target(position.x, &env.world) {
        env.scheduler.send(commands::Attack(entity, target));
    } else {
        env.scheduler.send(commands::AttackTown(entity));
    }

    true
}

fn next_npc(world: &World) -> Option<(Entity, Position)> {
    let mut npcs = query_iter!(world.0, With(npc, position))
        .map(|(e, _, p)| (e, *p))
        .filter(|(_, p)| p.y == BOARD_H as i32)
        .collect::<Vec<_>>();
    npcs.sort_by(|a, b| a.1.x.cmp(&b.1.x).then(a.1.y.cmp(&b.1.y)));
    npcs.first().copied()
}

fn next_target(col: i32, world: &World) -> Option<Entity> {
    let mut players = query_iter!(world.0, With(player, position))
        .filter(|(_, _, p)| p.x == col)
        .map(|(e, _, p)| (e, *p))
        .collect::<Vec<_>>();
    players.sort_by(|a, b| b.1.y.cmp(&a.1.y));
    players.first().map(|(e, _)| *e)
}
