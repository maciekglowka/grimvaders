use rand::prelude::*;

use crate::{utils::take_random, World};

#[derive(Default)]
pub struct PlayerData {
    pub deck: Vec<String>,
    pub level: u32,
    pub health: u32,
    pub food: u32,
}

pub(crate) fn player_game_init(world: &mut World) {
    world.resources.player_data = PlayerData::default();
    world.resources.player_data.health = 5;
    world.resources.player_data.deck = get_initial_squad().iter().map(|a| a.to_string()).collect();
}

fn get_initial_squad() -> Vec<&'static str> {
    let mut rng = thread_rng();
    let extra_unit = ["Peasant", "Sheep", "Wanderer"].choose(&mut rng).unwrap();
    vec!["Scarecrow", "Villager", extra_unit]
}

// fn get_initial_squad() -> Vec<&'static str> {
//     let mut output = vec!["Scarecrow"];
//     let mut rng = thread_rng();
//     let special_unit_count = rng.gen_range(1..=2);
//     let mut special_units = vec!["Peasant", "Sheep", "Wanderer"];
//     for _ in 0..special_unit_count {
//         output.push(take_random(&mut special_units, &mut rng));
//     }
//     while output.len() < 5 {
//         output.push("Villager");
//     }
//     output
// }
