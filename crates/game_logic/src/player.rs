use rand::prelude::*;
use std::collections::VecDeque;
use wunderkammer::prelude::*;

use crate::{globals::HAND_SIZE, World};

#[derive(Default)]
pub struct PlayerData {
    pub draw: VecDeque<Entity>,
    pub discard: Vec<Entity>,
    pub hand: Vec<Entity>,
    pub level: u32,
    pub health: u32,
    pub food: u32,
}

pub(crate) fn player_game_init(world: &mut World) {
    world.0.resources.player_data = PlayerData::default();
    world.0.resources.player_data.health = 5;

    for name in ["Bard", "Sheep", "Sheep", "Villager", "Villager"] {
        let entity = crate::utils::spawn_by_name(name, world).unwrap();
        world.0.components.player.insert(entity, ());
        world.0.resources.player_data.draw.push_back(entity);
    }
}

pub(crate) fn reset_deck(world: &mut World) {
    let mut rng = thread_rng();

    let mut deck: Vec<_> = world.0.resources.player_data.draw.drain(..).collect();
    deck.append(&mut world.resources.player_data.hand);
    deck.append(&mut world.resources.player_data.discard);
    deck.shuffle(&mut rng);
    world.0.resources.player_data.draw = deck.into();
}

pub(crate) fn draw_hand(world: &mut World) {
    world
        .0
        .resources
        .player_data
        .discard
        .append(&mut world.0.resources.player_data.hand);

    if world.0.resources.player_data.draw.len() < HAND_SIZE {
        reset_deck(world);
    }

    for _ in 0..HAND_SIZE {
        if let Some(entity) = world.0.resources.player_data.draw.pop_front() {
            world.0.resources.player_data.hand.push(entity);
        }
    }
}
