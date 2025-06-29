use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::{components::ValueDefault, get_data_by_name, World};

use crate::{
    globals::{
        BASE_TEXT_SIZE, BUTTON_CLICK_SHIFT, DECK_BUTTON_H, DECK_BUTTON_SPRITE,
        DECK_BUTTON_SPRITE_SELECTED, DECK_BUTTON_W, DIGITS_TEXT_SIZE, FOOD_COLOR, FOOD_ICON, GAP,
        HEALTH_ICON, ICON_SIZE, RED_COLOR, SPRITE_SIZE, TEXT_LINE_GAP, UI_Z,
    },
    input::InputState,
    ui::{Button, Span, TextBox},
    utils::get_viewport_bounds,
};

use super::sprites::get_sprite_data;

pub(crate) fn draw_deck_unit_stats(
    data: &game_data::EntityData,
    origin: Vector2f,
    z: i32,
    world: &World,
    context: &mut Context,
) {
    let mut spans = Vec::new();

    if let Some(health) = component_from_data::<u32>(data, "health") {
        spans.push(
            Span::new()
                .with_sprite("icons_small", HEALTH_ICON)
                .with_spacer(1.)
                .with_text_owned(format!("{}", health))
                .with_text_color(RED_COLOR)
                .with_font("digits")
                .with_text_size(DIGITS_TEXT_SIZE)
                .with_sprite_size(ICON_SIZE),
        );
    }

    if let Some(cost) = data.cost {
        spans.push(Span::new().with_spacer(2.));
        spans.push(
            Span::new()
                .with_sprite("icons_small", FOOD_ICON)
                .with_spacer(1.)
                .with_text_owned(format!("{}", cost))
                .with_text_color(FOOD_COLOR)
                .with_font("digits")
                .with_text_size(DIGITS_TEXT_SIZE)
                .with_sprite_size(ICON_SIZE),
        );
    }

    let w: f32 = spans.iter().map(|s| s.width(context)).sum();

    let mut base = origin
        + Vector2f::new(
            (0.5 * (SPRITE_SIZE - w)).round(),
            0., // SPRITE_SIZE - DIGITS_TEXT_SIZE,
        );

    for span in spans {
        span.draw(base, z + 1, context);
        base.x += span.width(context);
    }
}

pub(crate) fn draw_unit_overlay(
    entity: Entity,
    origin: Vector2f,
    z: i32,
    world: &World,
    context: &mut Context,
) {
    let Some(health) = world.components.health.get(entity) else {
        return;
    };

    let t = format!("{}", health);
    let w = context
        .graphics
        .text_dimensions("digits_outline", &t, DIGITS_TEXT_SIZE)
        .x;
    let base = origin
        + Vector2f::new(
            0.25 * SPRITE_SIZE - w,
            0.75 * SPRITE_SIZE - DIGITS_TEXT_SIZE,
        );

    let _ = context.graphics.draw_text(
        "digits_outline",
        &t,
        base,
        z + 1,
        DIGITS_TEXT_SIZE,
        SpriteParams {
            color: RED_COLOR,
            ..Default::default()
        },
    );
}

pub(crate) fn draw_deck_button(
    name: &str,
    origin: Vector2f,
    z: i32,
    selected: bool,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
) -> bool {
    let mut button = Button::new(origin, Vector2f::new(DECK_BUTTON_W, DECK_BUTTON_H), z)
        .with_sprite("ui", DECK_BUTTON_SPRITE);
    if selected {
        button = button.with_sprite("ui", DECK_BUTTON_SPRITE_SELECTED);
    }
    button.draw(context, input_state);

    let unit_offset = if button.pressed(input_state) {
        BUTTON_CLICK_SHIFT
    } else {
        0.
    };

    if button.mouse_over(input_state) {
        draw_description(name, world, context);
    }

    let Some(data) = get_data_by_name(name, world) else {
        return false;
    };

    // Draw unit sprite
    let _ = context.graphics.draw_atlas_sprite(
        &data.sprite.atlas,
        data.sprite.index,
        origin + Vector2f::new(0., 2. * GAP - unit_offset),
        z + 1,
        Vector2f::splat(SPRITE_SIZE),
        SpriteParams::default(),
    );

    // Draw stats
    draw_deck_unit_stats(
        data,
        origin + Vector2f::new(0., DECK_BUTTON_H - 8. - unit_offset),
        z + 1,
        world,
        context,
    );

    // Return click status
    button.clicked(input_state)
}

pub(crate) fn draw_entity_description(entity: Entity, world: &World, context: &mut Context) {
    let Some(name) = world.components.name.get(entity) else {
        return;
    };
    draw_description(name, world, context);
}

pub(crate) fn draw_description(name: &str, world: &World, context: &mut Context) {
    let Some(data) = get_data_by_name(name, world) else {
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
        let mut content = descr.to_string();

        if let Some(limit) = component_from_data::<ValueDefault>(data, "trigger_limit") {
            content += &format!("Triggers max {}x/turn.", limit.default());
        }

        let text = TextBox::owned(content);

        let h = text.draw(origin, bounds.1.x - bounds.0.x - 2. * GAP, UI_Z, context);
        origin.y -= h - BASE_TEXT_SIZE;
    };

    if let Some(tags) = component_from_data::<Vec<game_logic::components::Tag>>(data, "tags") {
        let names: Vec<String> = tags.iter().map(|a| a.into()).collect();
        let _ = context.graphics.draw_text(
            "default",
            &names.join(", "),
            origin,
            UI_Z,
            BASE_TEXT_SIZE,
            SpriteParams {
                color: RED_COLOR,
                ..Default::default()
            },
        );
    }
}

fn component_from_data<T: serde::de::DeserializeOwned>(
    data: &game_data::EntityData,
    component: &str,
) -> Option<T> {
    serde_yaml::from_value(data.components.get(component)?.clone()).ok()
}
