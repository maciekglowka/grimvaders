use rand::prelude::*;
use wunderkammer::prelude::*;

use crate::{
    components::Position,
    globals::{BOARD_H, BOARD_W},
    World,
};

pub fn is_on_board(p: Position) -> bool {
    p.x >= 0 && p.y >= 0 && p.x < BOARD_W as i32 && p.y < BOARD_H as i32
}

pub(crate) fn spawn_by_name(name: &str, world: &mut World) -> Option<Entity> {
    let entity = world.0.spawn();
    let data = world.0.resources.data.entities.get(name)?.clone();
    crate::components::insert_components(entity, world, &data);
    world.0.components.name.insert(entity, name.to_string());
    Some(entity)
}

pub fn get_unit_at(world: &World, position: Position) -> Option<Entity> {
    query_iter!(world.0, Without(tile), With(position))
        .filter(|(_, p)| **p == position)
        .map(|(e, _)| e)
        .next()
}

pub fn get_tile_at(world: &World, position: Position) -> Option<Entity> {
    query_iter!(world.0, With(tile, position))
        .filter(|(_, _, p)| **p == position)
        .map(|(e, _, _)| e)
        .next()
}

pub(crate) fn take_random<T, R: Rng + ?Sized>(values: &mut Vec<T>, rng: &mut R) -> T {
    values.remove(rng.gen_range(0..values.len()))
}
