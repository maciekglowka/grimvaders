use rand::prelude::*;
use wunderkammer::prelude::*;

use crate::{
    commands,
    components::Position,
    globals::{BOARD_H, BOARD_W},
    utils::spawn_by_name,
    GameEnv, World,
};

pub(crate) fn next_wave(env: &mut GameEnv) {
    let count = 2 * env.world.0.resources.battle_state.wave;
    let mut layout = [const { Vec::new() }; BOARD_W];
    let mut rng = thread_rng();

    for _ in 0..count {
        let entity = spawn_by_name("Rat", &mut env.world).unwrap();
        env.world.0.components.npc.insert(entity, ());
        let col = rng.gen_range(0..BOARD_W);
        layout[col].push(entity);
    }

    for x in 0..BOARD_W {
        for (y, &entity) in layout[x].iter().enumerate() {
            env.scheduler.send(commands::PlaceUnit(
                entity,
                Position::new(x as i32, (BOARD_H + y) as i32),
            ));
        }
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
