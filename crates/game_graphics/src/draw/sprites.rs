use rogalik::prelude::*;
use std::collections::VecDeque;
use wunderkammer::prelude::*;

use game_data::SpriteData;
use game_logic::{components::Position, World};

use crate::{
    globals::{MOVE_SPEED, MOVE_THRESH, SPRITE_SIZE, UNIT_Z},
    utils::tile_to_sprite,
};

#[derive(Clone, Copy, Default)]
pub enum Ease {
    #[default]
    None,
    In,
    Out,
    InOut,
}

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
    pub fn add_translations(&mut self, path: &[(Vector2f, Ease)]) {
        if let Some(EntityAnimation::Translate(_, animation)) = &mut self.animation {
            animation.extend(path);
        } else {
            let mut path = VecDeque::from_iter(path.iter().copied());
            path.push_front((self.origin, Ease::default()));
            self.animation = Some(EntityAnimation::Translate(0., path));
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
    pub fn mouse_over(&self, state: &crate::input::InputState) -> bool {
        crate::utils::is_mouse_over(self.origin, Vector2f::splat(SPRITE_SIZE), state)
    }
}

pub enum EntityAnimation {
    Translate(f32, VecDeque<(Vector2f, Ease)>),
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
    let sprite = UnitSprite::new(entity, world).with_origin(origin);
    sprites.push(sprite);
}

pub(crate) fn remove_unit_sprite(entity: Entity, sprites: &mut Vec<UnitSprite>) {
    sprites.retain(|a| a.entity != entity);
}

pub(crate) fn move_unit_sprite(entity: Entity, world: &World, sprites: &mut Vec<UnitSprite>) {
    if let Some(sprite) = get_unit_sprite_mut(entity, sprites) {
        if let Some(position) = world.0.components.position.get(entity) {
            sprite.add_translations(&vec![(tile_to_sprite(*position), Ease::InOut)]);
        }
    }
}

pub(crate) fn attack_unit_sprite(
    source: Entity,
    target: Entity,
    world: &World,
    sprites: &mut Vec<UnitSprite>,
) {
    if let Some(sprite) = get_unit_sprite_mut(source, sprites) {
        if let Some(target_position) = world.0.components.position.get(target) {
            // let tile_in_front = Position::new(target_position.x, target_position.y + 1);
            let dest = tile_to_sprite(*target_position);
            // let next_origin = 0.5 * (sprite.origin + dest);
            // let path = vec![dest];
            let path = vec![(dest, Ease::In), (sprite.origin, Ease::Out)];
            sprite.add_translations(&path);
        }
    }
}

pub(crate) fn attack_town(source: Entity, world: &World, sprites: &mut Vec<UnitSprite>) {
    if let Some(sprite) = get_unit_sprite_mut(source, sprites) {
        if let Some(position) = world.0.components.position.get(source) {
            let dest = tile_to_sprite(Position::new(position.x, -1));
            let path = vec![(dest, Ease::In)];
            sprite.add_translations(&path);
        }
    }
}

pub(crate) fn get_sprite_data<'a>(name: &str, world: &'a World) -> Option<&'a SpriteData> {
    Some(&world.0.resources.data.entities.get(name)?.sprite)
}

pub(crate) fn animate_unit_sprite(sprite: &mut UnitSprite, delta: f32) -> bool {
    let Some(animation) = &mut sprite.animation else {
        return false;
    };
    match animation {
        EntityAnimation::Translate(t, path) => {
            if path.len() < 2 {
                sprite.animation = None
            } else if *t >= 1. {
                path.pop_front();
                *t = 0.;
            } else {
                // TODO do not calculate each frame
                let total = translation_time(path[0].0, path[1].0);
                *t += delta / total;
                sprite.origin = path[0].0.lerp(&path[1].0, ease(*t, path[1].1));
            }
        }
    }
    true
}

fn translation_time(a: Vector2f, b: Vector2f) -> f32 {
    (b - a).len() / MOVE_SPEED
}

fn ease(val: f32, ease: Ease) -> f32 {
    match ease {
        Ease::None => val,
        Ease::In => ease_in(val),
        Ease::Out => ease_out(val),
        Ease::InOut => ease_in_out(val),
    }
}

fn ease_in(val: f32) -> f32 {
    1. - f32::cos(0.5 * val * std::f32::consts::PI)
}

fn ease_out(val: f32) -> f32 {
    f32::sin(0.5 * val * std::f32::consts::PI)
}

fn ease_in_out(val: f32) -> f32 {
    -0.5 * (f32::cos(std::f32::consts::PI * val) - 1.)
}
