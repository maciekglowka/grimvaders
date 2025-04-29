use rogalik::math::vectors::Vector2i;
use rune::{Any, ToValue};
use serde::Deserialize;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use wunderkammer::prelude::*;

use game_data::EntityData;

use crate::World;

#[derive(Any, Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
pub enum Targeting {
    #[default]
    None,
    Neighbour,
    Any,
}

#[derive(Any, Clone, Debug, Default, Deserialize)]
pub struct ValueDefault(u32, u32);
impl ValueDefault {
    pub fn new(val: u32) -> Self {
        Self(val, val)
    }
    pub fn current(&self) -> u32 {
        self.0
    }
    pub fn default(&self) -> u32 {
        self.1
    }
    pub fn add(&mut self, value: u32) {
        self.0 = (self.0 + value).min(self.1);
    }
    pub fn sub(&mut self, value: u32) {
        self.0 = self.0.saturating_sub(value);
    }
    pub fn add_default(&mut self, value: u32) {
        self.1 += value;
        self.0 += value;
    }
    pub fn sub_default(&mut self, value: u32) {
        self.1 = self.1.saturating_sub(value);
        self.0 = self.0.min(self.1);
    }
    pub fn set_default(&mut self, value: u32) {
        self.1 = value;
        self.0 = self.0.min(self.1);
    }
    pub fn restore(&mut self) {
        self.0 = self.1;
    }
}

macro_rules! handle_component {
    ($world:ident, $component:ident, $entity:ident, $value:ident, $type:ty) => {
        $world.0.components.$component.insert(
            $entity,
            serde_yaml::from_value::<$type>($value.clone())
                .expect(&format!("Could not parse {:?}", $value)),
        )
    };
}

pub(crate) fn insert_components(entity: Entity, world: &mut World, data: &EntityData) {
    for (k, v) in data.components.iter() {
        match k.as_str() {
            "cost" => handle_component!(world, cost, entity, v, u32),
            "health" => handle_component!(world, health, entity, v, ValueDefault),
            "player" => handle_component!(world, player, entity, v, ()),
            a => panic!("Unknown component {}", a),
        }
    }
}

pub(crate) const ORTHO: [Position; 4] = [
    Position { x: 0, y: 1 },
    Position { x: 1, y: 0 },
    Position { x: 0, y: -1 },
    Position { x: -1, y: 0 },
];

#[derive(Any, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Position {
    #[rune(get)]
    pub x: i32,
    #[rune(get)]
    pub y: i32,
}
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn manhattan(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Add<&Position> for Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign<&Position> for Position {
    fn add_assign(&mut self, other: &Position) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub<&Position> for Position {
    type Output = Position;

    fn sub(self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign<&Position> for Position {
    fn sub_assign(&mut self, other: &Position) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
