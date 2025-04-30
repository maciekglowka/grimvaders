use rand::prelude::*;

use crate::{events::InputEvent, globals::SHOP_SIZE, utils::spawn_by_name, GameEnv, World};

#[derive(Default)]
pub struct ShopState {
    pub choices: [Option<(String, u32)>; SHOP_SIZE],
    pub done: bool,
}

pub fn shop_init(state: &mut ShopState, env: &mut GameEnv) {
    let level = env.world.0.resources.player_data.level;
    state.choices = get_choices(level, &env.world);
}

pub fn shop_update(state: &mut ShopState, env: &mut GameEnv) {
    while let Some(event) = env.input.as_ref().unwrap().next() {
        match event {
            InputEvent::Done => state.done = true,
            // InputEvent::BuyUnit(i) => buy_unit(i, state, env),
            _ => (),
        }
    }
}

fn buy_unit(i: usize, state: &mut ShopState, env: &mut GameEnv) {
    // let Some((name, price)) = &state.choices[i] else {
    //     return;
    // };

    // if env.world.0.resources.player_data.gold < *price {
    //     return;
    // }

    // let Some(entity) = spawn_by_name(name, &mut env.world) else {
    //     return;
    // };
    // env.world.0.resources.player_data.gold -= price;
    // env.world.0.resources.player_data.draw.push_back(entity);

    // state.choices[i] = None;
}

fn get_choices(level: u32, world: &World) -> [Option<(String, u32)>; SHOP_SIZE] {
    let filtered = world
        .0
        .resources
        .data
        .actions
        .iter()
        .filter_map(|n| world.0.resources.data.entities.get(n).map(|e| (n, e)))
        .filter(|(_, e)| {
            e.min_level.unwrap_or(0) <= level && e.max_level.unwrap_or(u32::MAX) >= level
        })
        .map(|(n, e)| {
            (
                e.chance,
                n.to_string(),
                get_price(
                    level,
                    e.min_level.unwrap_or(0),
                    e.max_level.unwrap_or(u32::MAX),
                    e.chance,
                ),
            )
        })
        .collect::<Vec<_>>();

    let mut output = [const { None }; SHOP_SIZE];
    let mut rng = thread_rng();

    for i in 0..SHOP_SIZE {
        if let Ok((_, name, price)) = filtered.choose_weighted(&mut rng, |a| a.0) {
            output[i] = Some((name.to_string(), *price));
        }
    }
    output
}

fn get_price(level: u32, min_level: u32, max_level: u32, chance: f32) -> u32 {
    ((5 + min_level) as f32 / chance) as u32
}
