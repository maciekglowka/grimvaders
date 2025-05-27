use rand::prelude::*;
use wunderkammer::prelude::*;

use crate::{events::InputEvent, globals::SHOP_SIZE, utils::spawn_by_name, GameEnv, World};

#[derive(Default)]
pub struct ShopState {
    pub choices: [Option<Entity>; SHOP_SIZE],
    pub done: bool,
}

pub fn shop_init(state: &mut ShopState, env: &mut GameEnv) {
    let level = env.world.resources.player_data.level;
    println!("Tier: {}", level);
    for (i, name) in get_choices(level, &env.world).iter().enumerate() {
        let Some(name) = name else {
            continue;
        };
        state.choices[i] = spawn_by_name(name, &mut env.world);
    }
}

pub fn shop_exit(state: &mut ShopState, env: &mut GameEnv) {
    for entity in state.choices {
        if let Some(entity) = entity {
            env.world.despawn(entity);
        }
    }
}

pub fn shop_update(state: &mut ShopState, env: &mut GameEnv) {
    while let Some(event) = env.input.as_ref().unwrap().next() {
        match event {
            InputEvent::Done => state.done = true,
            InputEvent::PickUnit(i) => pick_unit(i, state, env),
            _ => (),
        }
    }
}

fn pick_unit(i: usize, state: &mut ShopState, env: &mut GameEnv) {
    let Some(entity) = state.choices[i] else {
        return;
    };

    env.world.resources.player_data.draw.push_back(entity);
    state.choices[i] = None;
    state.done = true;
}

fn get_choices(tier: u32, world: &World) -> [Option<String>; SHOP_SIZE] {
    let filtered = world.resources.data.categories["player"]
        .iter()
        .filter_map(|n| world.0.resources.data.entities.get(n).map(|e| (n, e)))
        .filter(|(_, e)| e.tier.unwrap_or(0) <= tier)
        .map(|(n, e)| {
            let tier_dist = 0.8 / tier as f32 * e.tier.unwrap_or(1) as f32 + 0.2;
            println!("T: {}, Et: {:?}, Dt: {}", tier, e.tier, tier_dist);
            (tier_dist * e.chance.unwrap_or(1.), n.to_string())
        })
        .collect::<Vec<_>>();

    let mut output = [const { None }; SHOP_SIZE];
    let mut rng = thread_rng();

    for (i, name) in filtered
        .choose_multiple_weighted(&mut rng, SHOP_SIZE.min(filtered.len()), |a| a.0)
        .unwrap()
        .map(|a| Some(a.1.to_string()))
        .enumerate()
    {
        output[i] = name
    }
    output
}
