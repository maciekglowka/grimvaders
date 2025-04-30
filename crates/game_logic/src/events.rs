use rogalik::math::vectors::Vector2i;
use wunderkammer::prelude::*;

use crate::components::Position;

#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    MoveUnit(Entity, Position),
    SpawnUnit(Entity, Position),
    RedrawHand,
    Done,
    DiscardUnit(Entity),
}
