use rogalik::prelude::*;
use std::collections::VecDeque;
use wunderkammer::prelude::*;

use game_data::SpriteData;
use game_logic::{components::Position, globals::BOARD_H, World};

use crate::{
    globals::{DIGITS_TEXT_SIZE, MOVE_SPEED, MOVE_THRESH, OVERLAY_Z, SPRITE_SIZE, UNIT_Z},
    utils::{move_towards, tile_to_sprite, tile_to_world},
};

#[derive(Default)]
pub struct UnitSprite {
    pub entity: Entity,
    pub origin: Vector2f,
    pub atlas: String,
    pub index: usize,
    pub animation: Option<EntityAnimation>,
}
impl UnitSprite {
    pub fn new(entity: Entity, world: &World) -> Self {
        let mut atlas = "sprites";
        let mut index = 721;
        if let Some(name) = world.0.components.name.get(entity) {
            if let Some(data) = get_sprite_data(name, world) {
                atlas = &data.atlas;
                index = data.index;
            }
        }

        Self {
            entity,
            atlas: atlas.to_string(),
            index,
            ..Default::default()
        }
    }
    pub fn with_origin(mut self, v: Vector2f) -> Self {
        self.origin = v;
        self
    }
    pub fn add_translations(&mut self, path: &[Vector2f]) {
        if let Some(EntityAnimation::Translate(animation)) = &mut self.animation {
            animation.extend(path);
        } else {
            self.animation = Some(EntityAnimation::Translate(VecDeque::from_iter(
                path.iter().copied(),
            )));
        }
    }
    pub fn draw(&self, world: &World, context: &mut Context) {
        let _ = context.graphics.draw_atlas_sprite(
            &self.atlas,
            self.index,
            self.origin,
            UNIT_Z,
            Vector2f::splat(SPRITE_SIZE),
            SpriteParams::default(),
        );

        super::units::draw_unit_overlay(self.entity, self.origin, world, context);
    }
}

pub enum EntityAnimation {
    Translate(VecDeque<Vector2f>),
}

pub(crate) fn get_unit_sprite(entity: Entity, sprites: &Vec<UnitSprite>) -> Option<&UnitSprite> {
    sprites.iter().find(|a| a.entity == entity)
}

pub(crate) fn get_unit_sprite_mut(
    entity: Entity,
    sprites: &mut Vec<UnitSprite>,
) -> Option<&mut UnitSprite> {
    sprites.iter_mut().find(|a| a.entity == entity)
}

pub(crate) fn place_unit_sprite(
    entity: Entity,
    position: Position,
    world: &World,
    sprites: &mut Vec<UnitSprite>,
) {
    let origin = tile_to_sprite(position);
    if let Some(sprite) = get_unit_sprite_mut(entity, sprites) {
        sprite.origin = origin;
        return;
    }
    let sprite = UnitSprite::new(entity, world).with_origin(origin);
    sprites.push(sprite);
}

pub(crate) fn remove_unit_sprite(entity: Entity, sprites: &mut Vec<UnitSprite>) {
    sprites.retain(|a| a.entity != entity);
}

pub(crate) fn move_unit_sprite(entity: Entity, world: &World, sprites: &mut Vec<UnitSprite>) {
    // if let Some(sprite) = get_card_sprite_mut(entity, sprites) {
    //     if let Some(position) = world.0.components.position.get(entity) {
    //         sprite.add_translations(&vec![card_to_world(*position)]);
    //     }
    // }
}

pub(crate) fn attack_unit_sprite(
    source: Entity,
    target: Entity,
    world: &World,
    sprites: &mut Vec<UnitSprite>,
) {
    if let Some(sprite) = get_unit_sprite_mut(source, sprites) {
        if let Some(position) = world.0.components.position.get(source) {
            if let Some(target_position) = world.0.components.position.get(target) {
                let origin = tile_to_sprite(*position);
                let dest = tile_to_sprite(*target_position);
                let path = vec![dest, origin];
                sprite.add_translations(&path);
            }
        }
    }
}

pub(crate) fn attack_town(source: Entity, world: &World, sprites: &mut Vec<UnitSprite>) {
    if let Some(sprite) = get_unit_sprite_mut(source, sprites) {
        if let Some(position) = world.0.components.position.get(source) {
            // let origin = tile_to_sprite(*position);
            let dest = tile_to_sprite(Position::new(position.x, -1));
            let path = vec![dest];
            sprite.add_translations(&path);
        }
    }
}

pub(crate) fn get_sprite_data<'a>(name: &str, world: &'a World) -> Option<&'a SpriteData> {
    Some(&world.0.resources.data.entities.get(name)?.sprite)
}

pub(crate) fn animate_card_sprite(sprite: &mut UnitSprite, delta: f32) -> bool {
    let Some(animation) = &mut sprite.animation else {
        return false;
    };
    match animation {
        EntityAnimation::Translate(path) => {
            if let Some(target) = path.get(0) {
                if (*target - sprite.origin).len() <= MOVE_THRESH {
                    sprite.origin = *target;
                    path.pop_front();
                } else {
                    sprite.origin = move_towards(sprite.origin, *target, delta * MOVE_SPEED)
                }
            } else {
                sprite.animation = None
            }
        }
    }
    true
}
