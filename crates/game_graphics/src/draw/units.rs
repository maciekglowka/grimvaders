use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::World;

use crate::{
    globals::{
        BASE_TEXT_SIZE, DIGITS_TEXT_SIZE, FOOD_COLOR, GAP, ICON_SIZE, OVERLAY_Z, RED_COLOR,
        SPRITE_SIZE, TEXT_LINE_GAP, UI_Z,
    },
    ui::{Span, TextBox},
    utils::get_viewport_bounds,
};

use super::sprites::get_sprite_data;

pub(crate) fn draw_unit_overlay(
    entity: Entity,
    origin: Vector2f,
    z: i32,
    world: &World,
    context: &mut Context,
    with_cost: bool,
) {
    let mut spans = Vec::new();

    if let Some(health) = world.0.components.health.get(entity) {
        spans.push(
            Span::new()
                .with_sprite("icons_small", 0)
                .with_spacer(1.)
                .with_text_owned(format!("{}", health.current()))
                .with_text_color(RED_COLOR)
                .with_font("digits")
                .with_text_size(DIGITS_TEXT_SIZE)
                .with_sprite_size(ICON_SIZE),
        );
    }

    if with_cost {
        if let Some(cost) = world.0.components.cost.get(entity) {
            spans.push(Span::new().with_spacer(2.));
            spans.push(
                Span::new()
                    .with_sprite("icons_small", 1)
                    .with_spacer(1.)
                    .with_text_owned(format!("{}", cost))
                    .with_text_color(FOOD_COLOR)
                    .with_font("digits")
                    .with_text_size(DIGITS_TEXT_SIZE)
                    .with_sprite_size(ICON_SIZE),
            );
        }
    }

    let w: f32 = spans.iter().map(|s| s.width(context)).sum();

    let oh = 4.;
    let ov = 2.;
    let mut base = origin
        + Vector2f::new(
            (0.5 * (SPRITE_SIZE - w)).round(),
            SPRITE_SIZE - DIGITS_TEXT_SIZE + 2. * ov,
        );

    let _ = context.graphics.draw_atlas_sprite(
        "ui",
        3,
        base - Vector2f::new(oh, ov),
        z,
        Vector2f::new(w + 2. * oh, DIGITS_TEXT_SIZE + 2. * ov),
        SpriteParams {
            slice: Some((4, Vector2f::splat(SPRITE_SIZE))),
            ..Default::default()
        },
    );

    for span in spans {
        span.draw(base, z + 1, context);
        base.x += span.width(context);
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
    draw_unit_overlay(entity, origin, z + 1, world, context, true);

    // if let Some(cost) = world.0.components.cost.get(entity) {
    //     let span = Span::new()
    //         .with_sprite("icons_small", 1)
    //         .with_spacer(1.)
    //         .with_text_owned(format!("{}", cost))
    //         .with_text_color(FOOD_COLOR)
    //         .with_font("digits")
    //         .with_text_size(DIGITS_TEXT_SIZE)
    //         .with_sprite_size(ICON_SIZE);

    //     let w = span.width(context);
    //     span.draw(
    //         origin + Vector2f::new(0.5 * (SPRITE_SIZE - w), SPRITE_SIZE +
    // 1.),         z + 1,
    //         context,
    //     );
    // }
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
