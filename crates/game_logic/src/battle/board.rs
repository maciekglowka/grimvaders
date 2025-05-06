use rand::prelude::*;
use std::collections::{HashMap, VecDeque};
use wunderkammer::prelude::*;

use crate::{
    components::{Position, Tile},
    globals::{BOARD_H, BOARD_W},
    GameEnv,
};

pub(crate) fn tiles_init(env: &mut GameEnv) {
    let mut rng = thread_rng();
    let pool = [Tile::Plains, Tile::Meadow, Tile::Field, Tile::Forest];

    for x in 0..BOARD_W {
        for y in 0..BOARD_H {
            let tile = pool.choose(&mut rng).unwrap();

            let entity = env.world.0.spawn();
            insert!(
                env.world.0,
                position,
                entity,
                Position::new(x as i32, y as i32)
            );
            insert!(env.world.0, tile, entity, *tile);
        }
    }
}

pub(crate) fn clear_board(env: &mut GameEnv) {
    let to_remove = query_iter!(env.world.0, With(position))
        .map(|(e, _)| e)
        .collect::<Vec<_>>();

    for entity in to_remove {
        env.world.0.despawn(entity);
    }
}
