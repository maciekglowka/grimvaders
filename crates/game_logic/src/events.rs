use wunderkammer::prelude::*;

use crate::components::Position;

#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    MoveUnit(Entity, Position),
    SummonPlayer(usize, Position),
    Done,
    PickUnit(usize),
    DiscardUnit(usize),
}
