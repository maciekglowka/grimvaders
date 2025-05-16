use rand::prelude::*;
use rune::alloc::HashSet;
use wunderkammer::prelude::*;

use crate::{
    commands,
    components::Position,
    globals::{BOARD_H, BOARD_W, MAX_WAVE_H},
    utils::spawn_by_name,
    GameEnv, World,
};

pub(crate) fn next_wave(env: &mut GameEnv) {
    let tier = env.world.resources.player_data.level;
    let target_score = tier * env.world.resources.battle_state.wave;
    let mut score = 0;

    let mut rng = thread_rng();

    let pool = get_pool(tier, &env.world);
    let mut layout = [const { Vec::new() }; BOARD_W];

    loop {
        let filtered = pool
            .iter()
            .filter(|(_, s)| *s <= target_score - score)
            .collect::<Vec<_>>();
        if filtered.is_empty() {
            break;
        };

        let Ok((name, entity_score)) = filtered.choose_weighted(&mut rng, |(_, s)| s) else {
            break;
        };

        let entity = spawn_by_name(name, &mut env.world).unwrap();
        score += entity_score;
        env.world.0.components.npc.insert(entity, ());

        let layout_weights = layout
            .iter()
            .enumerate()
            .map(|(i, v)| (i, MAX_WAVE_H.saturating_sub(v.len())))
            .collect::<Vec<_>>();
        let Ok((col, _)) = layout_weights.choose_weighted(&mut rng, |(_, w)| *w) else {
            break;
        };
        layout[*col].push(entity);
    }

    for x in 0..BOARD_W {
        for (y, &entity) in layout[x].iter().enumerate() {
            env.scheduler.send(commands::SpawnUnit(
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

fn get_pool(tier: u32, world: &World) -> Vec<(String, u32)> {
    world.0.resources.data.categories["npcs"]
        .iter()
        .filter(|&n| {
            world
                .0
                .resources
                .data
                .entities
                .get(n)
                .unwrap()
                .tier
                .unwrap_or(1)
                <= tier
        })
        .map(|n| {
            (
                n.to_string(),
                world
                    .0
                    .resources
                    .data
                    .entities
                    .get(n)
                    .unwrap()
                    .score
                    .unwrap_or(1),
            )
        })
        .collect()
}
