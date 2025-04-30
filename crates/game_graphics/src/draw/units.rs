use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::World;

use crate::globals::{DIGITS_TEXT_SIZE, GAP, OVERLAY_Z, SPRITE_SIZE, UI_Z};

use super::sprites::get_sprite_data;

pub(crate) fn draw_unit_overlay(
    entity: Entity,
    origin: Vector2f,
    world: &World,
    context: &mut Context,
) {
    if let Some(health) = world.0.components.health.get(entity) {
        let _ = context.graphics.draw_text(
            "digits",
            &format!("{}", health.current()),
            origin + Vector2f::new(0., SPRITE_SIZE - DIGITS_TEXT_SIZE),
            OVERLAY_Z,
            DIGITS_TEXT_SIZE,
            SpriteParams::default(),
        );
    }
}

pub(crate) fn draw_deck_unit(
    entity: Entity,
    origin: Vector2f,
    z: i32,
    world: &World,
    context: &mut Context,
) {
    let Some(name) = world.0.components.name.get(entity) else {
        return;
    };
    let Some(sprite) = get_sprite_data(name, world) else {
        return;
    };
    let _ = context.graphics.draw_atlas_sprite(
        &sprite.atlas,
        sprite.index,
        origin,
        z,
        Vector2f::splat(SPRITE_SIZE),
        SpriteParams::default(),
    );
    draw_unit_overlay(entity, origin, world, context);

    if let Some(cost) = world.0.components.cost.get(entity) {
        let _ = context.graphics.draw_text(
            "digits",
            &format!("G: {}", cost),
            origin + Vector2f::new(0., -DIGITS_TEXT_SIZE),
            OVERLAY_Z,
            DIGITS_TEXT_SIZE,
            SpriteParams::default(),
        );
    }
}
