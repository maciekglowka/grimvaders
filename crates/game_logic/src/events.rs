use rogalik::math::vectors::Vector2i;
use wunderkammer::prelude::*;

use crate::components::Position;

#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    PlaceUnit(Entity, Position),
    BuyUnit(usize),
    Done,
    DiscardUnit(Entity),
}
