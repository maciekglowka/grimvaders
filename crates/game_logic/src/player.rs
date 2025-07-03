use rand::prelude::*;
use std::collections::VecDeque;
use wunderkammer::prelude::*;

use crate::{globals::DECK_SIZE, utils::take_random, World};

#[derive(Default)]
pub struct PlayerData {
    pub discard: Vec<Entity>,
    pub deck: Vec<Entity>,
    pub level: u32,
    pub health: u32,
    pub food: u32,
}

pub(crate) fn player_game_init(world: &mut World) {
    world.0.resources.player_data = PlayerData::default();
    world.0.resources.player_data.health = 5;

    for name in get_initial_squad() {
        let entity = crate::utils::spawn_by_name(name, world).unwrap();
        world.0.components.player.insert(entity, ());
        world.0.resources.player_data.deck.push(entity);
    }
}

pub(crate) fn reset_deck(world: &mut World) {
    let discard: Vec<_> = world.resources.player_data.discard.drain(..).collect();
    world.resources.player_data.deck.extend(discard);
}

// pub(crate) fn draw_hand(world: &mut World) {
//     world
//         .0
//         .resources
//         .player_data
//         .discard
//         .append(&mut world.0.resources.player_data.hand);

//     if world.0.resources.player_data.draw.len() < HAND_SIZE {
//         reset_deck(world);
//     }

//     for _ in 0..HAND_SIZE {
//         if let Some(entity) = world.0.resources.player_data.draw.pop_front()
// {             world.0.resources.player_data.hand.push(entity);
//         }
//     }
// }

fn get_initial_squad() -> Vec<&'static str> {
    let mut output = vec!["Scarecrow"];
    let mut rng = thread_rng();
    let mut special_units = vec!["Peasant", "Sheep"];
    output.push(take_random(&mut special_units, &mut rng));
    while output.len() < 4 {
        output.push("Villager");
    }
    output
}
