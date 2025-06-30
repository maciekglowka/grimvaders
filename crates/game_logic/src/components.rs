use rune::{alloc::clone::TryClone, runtime::VmResult, Any};
use serde::Deserialize;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use wunderkammer::prelude::*;

use game_data::EntityData;

use crate::World;

#[derive(Any, Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum Tile {
    #[rune(constructor)]
    Plains,
    #[rune(constructor)]
    Meadow,
    #[rune(constructor)]
    Field,
    #[rune(constructor)]
    Forest,
}
impl Tile {
    #[rune::function(keep, instance, protocol = PARTIAL_EQ)]
    pub fn partial_eq(&self, rhs: &Self) -> VmResult<bool> {
        VmResult::Ok(self == rhs)
    }
}

#[derive(Any, Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum Tag {
    #[rune(constructor)]
    Basic,
    #[rune(constructor)]
    FoodProducer,
    #[rune(constructor)]
    Healer,
    #[rune(constructor)]
    Heavy,
}
impl From<Tag> for String {
    fn from(value: Tag) -> Self {
        match value {
            Tag::Basic => "Basic",
            Tag::FoodProducer => "Food Producer",
            Tag::Healer => "Healer",
            Tag::Heavy => "Heavy",
        }
        .to_string()
    }
}
impl From<&Tag> for String {
    fn from(value: &Tag) -> Self {
        <Tag as Into<String>>::into(*value)
    }
}

#[derive(Any, Clone, Debug, Default, Deserialize)]
pub struct ValueDefault(#[rune(get)] u32, #[rune(get)] u32);
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
        self.0 += value;
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
        self.0 = self.0.saturating_sub(value);
    }
    pub fn restore(&mut self) {
        self.0 = self.1;
    }
}

pub(crate) fn insert_components(entity: Entity, world: &mut World, data: &EntityData) {
    for (k, v) in data.components.iter() {
        crate::world::Components::insert_from_yaml(entity, k, v, world);
    }
}

pub(crate) const ORTHO: [Position; 4] = [
    Position { x: 0, y: 1 },
    Position { x: 1, y: 0 },
    Position { x: 0, y: -1 },
    Position { x: -1, y: 0 },
];

#[derive(Any, Clone, Copy, Debug, Hash, PartialEq, Deserialize, TryClone)]
#[rune(constructor)]
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
    #[rune::function(keep, instance, protocol = PARTIAL_EQ)]
    pub fn partial_eq(&self, rhs: &Self) -> VmResult<bool> {
        VmResult::Ok(self == rhs)
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
