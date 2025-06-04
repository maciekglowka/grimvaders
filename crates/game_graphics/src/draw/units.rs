use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::World;

use crate::{
    globals::{BASE_TEXT_SIZE, DIGITS_TEXT_SIZE, GAP, OVERLAY_Z, SPRITE_SIZE, TEXT_LINE_GAP, UI_Z},
    ui::TextBox,
    utils::get_viewport_bounds,
};

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
            &format!("{}", cost),
            origin + Vector2f::new(0., -DIGITS_TEXT_SIZE),
            z + 1,
            DIGITS_TEXT_SIZE,
            SpriteParams::default(),
        );
    }
}

pub(crate) fn draw_entity_description(entity: Entity, world: &World, context: &mut Context) {
    let Some(name) = world.components.name.get(entity) else {
        return;
    };
    draw_description(entity, name, world, context);
}

pub(crate) fn draw_description(entity: Entity, name: &str, world: &World, context: &mut Context) {
    let Some(data) = world.resources.data.entities.get(name) else {
        return;
    };

    let bounds = get_viewport_bounds(context);
    let mut origin = Vector2f::new(bounds.0.x + GAP, bounds.1.y - BASE_TEXT_SIZE - GAP);

    let _ = context.graphics.draw_text(
        "default",
        name,
        origin,
        UI_Z,
        BASE_TEXT_SIZE,
        SpriteParams::default(),
    );

    let gap = TEXT_LINE_GAP * BASE_TEXT_SIZE;
    origin.y -= BASE_TEXT_SIZE + 2. * gap;

    if let Some(descr) = &data.description {
        let text = TextBox::borrowed(descr);

        let h = text.draw(origin, bounds.1.x - bounds.0.x - 2. * GAP, UI_Z, context);
        origin.y -= h - BASE_TEXT_SIZE;
    };

    if let Some(tags) = world.components.tags.get(entity) {
        let names: Vec<String> = tags.iter().map(|a| a.into()).collect();
        let _ = context.graphics.draw_text(
            "default",
            &names.join(", "),
            origin,
            UI_Z,
            BASE_TEXT_SIZE,
            SpriteParams::default(),
        );
    }
}
