use wunderkammer::prelude::*;

use crate::components::Position;

#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    MoveUnit(Entity, Position),
    SummonUnit(Entity, Position),
    RedrawHand,
    Done,
    PickUnit(usize),
    DiscardUnit(Entity),
}
