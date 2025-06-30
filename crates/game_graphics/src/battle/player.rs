use rogalik::prelude::*;

use game_logic::{get_unit_at, is_on_board, is_on_extended_board, InputEvent, World};

use crate::{
    draw::units::draw_deck_button,
    globals::{
        ACTION_BUTTON_W, BASE_TEXT_SIZE, BUTTON_SIZE, BUTTON_TEXT_COLOR, CURSOR_SPIRTE,
        DECK_BUTTON_H, DECK_BUTTON_W, FIGHT_ICON, FOOD_COLOR, FOOD_ICON, GAP, HEALTH_ICON,
        ICON_SIZE, OVERLAY_Z, PANEL_SPRTE, RED_COLOR, SPRITE_SIZE, TILE_SIZE, TOWN_ICON, UI_Z,
    },
    input::{ButtonState, InputState},
    ui::{Button, Span},
    utils::{get_viewport_bounds, tile_to_world, world_to_tile},
};

use super::InputMode;

pub(super) fn handle_player_ui(
    world: &World,
    state: &mut super::BattleGraphics,
    context: &mut Context,
    input_state: &InputState,
    take_input: bool,
) {
    draw_status(state, world, context);
    handle_hand(state, world, context, input_state, take_input);
    handle_input_player(state, world, context, input_state, take_input);
    if take_input {
        draw_cursor(state, world, context, input_state);
    }
}

fn handle_input_player(
    state: &mut super::BattleGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
    take_input: bool,
) {
    let bounds = get_viewport_bounds(context);
    let fight = Button::new(
        Vector2f::new(bounds.1.x - ACTION_BUTTON_W - GAP, bounds.0.y + GAP),
        Vector2f::new(ACTION_BUTTON_W, BUTTON_SIZE),
        UI_Z,
    )
    .with_span(
        Span::new()
            .with_sprite("icons_small", FIGHT_ICON)
            .with_spacer(2.)
            .with_text_borrowed("Fight!")
            .with_sprite_size(ICON_SIZE),
    );
    fight.draw(context, input_state);

    if !take_input {
        return;
    }

    if fight.clicked(input_state) {
        state.input_queue.push(InputEvent::Done);
    }

    if input_state.click == ButtonState::Released {
        let tile = world_to_tile(input_state.mouse_world_position);

        if is_on_board(tile) {
            if let Some(entity) = get_unit_at(world, tile) {
                state.input_mode = InputMode::BoardUnit(entity);
            } else {
                match state.input_mode {
                    InputMode::HandUnit(entity) => {
                        state
                            .input_queue
                            .push(InputEvent::SummonPlayer(entity, tile));
                        state.input_mode = InputMode::None;
                    }
                    InputMode::BoardUnit(entity) => {
                        state.input_queue.push(InputEvent::MoveUnit(entity, tile));
                        state.input_mode = InputMode::None;
                    }
                    InputMode::None => (),
                }
            }
        }
    };
}

fn draw_cursor(
    state: &super::BattleGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
) {
    if let InputMode::BoardUnit(entity) = state.input_mode {
        if let Some(&position) = world.0.components.position.get(entity) {
            let _ = context.graphics.draw_atlas_sprite(
                "tiles",
                CURSOR_SPIRTE,
                tile_to_world(position),
                OVERLAY_Z,
                Vector2f::splat(TILE_SIZE),
                SpriteParams::default(),
            );
        }
    }

    let tile = world_to_tile(input_state.mouse_world_position);

    if !is_on_extended_board(tile) {
        return;
    }

    let _ = context.graphics.draw_atlas_sprite(
        "tiles",
        CURSOR_SPIRTE,
        tile_to_world(tile),
        OVERLAY_Z,
        Vector2f::splat(TILE_SIZE),
        SpriteParams::default(),
    );
}

fn handle_hand(
    state: &mut super::BattleGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
    take_input: bool,
) {
    let bounds = get_viewport_bounds(context);
    let bottom = bounds.0.y + BUTTON_SIZE + 2. * GAP;
    let base = Vector2f::new(bounds.1.x - SPRITE_SIZE - GAP, bottom + BUTTON_SIZE + GAP);

    let step = game_logic::globals::DECK_SIZE / 2;

    for (i, &entity) in world.0.resources.player_data.deck.iter().enumerate() {
        let origin = base
            + Vector2f::new(
                -(GAP + DECK_BUTTON_W) * (i % step) as f32,
                (GAP + DECK_BUTTON_H) * (i / step) as f32,
            );
        let selected = state.input_mode == InputMode::HandUnit(entity);

        let clicked = draw_deck_button(entity, origin, UI_Z, selected, world, context, input_state);

        if take_input && clicked {
            if selected {
                state.input_mode = InputMode::None
            } else {
                state.input_mode = InputMode::HandUnit(entity);
            }
        }
    }

    // let reroll = Button::new(
    //     Vector2f::new(bounds.1.x - ACTION_BUTTON_W - GAP, bottom),
    //     Vector2f::new(ACTION_BUTTON_W, BUTTON_SIZE),
    //     UI_Z,
    // )
    // .with_span(
    //     Span::new()
    //         .with_sprite("icons_small", UNIT_ICON)
    //         .with_spacer(2.)
    //         .with_text_borrowed("Reroll")
    //         .with_sprite_size(ICON_SIZE),
    // );
    // reroll.draw(context, input_state);
    // if reroll.clicked(input_state) {
    //     state.input_queue.push(InputEvent::RedrawHand);
    // }
}

pub(super) fn draw_status(state: &super::BattleGraphics, world: &World, context: &mut Context) {
    let mut spans = Vec::new();
    spans.push(
        Span::new()
            .with_sprite("icons_small", HEALTH_ICON)
            .with_spacer(2.)
            .with_text_owned(format!("{}", world.resources.player_data.health))
            .with_spacer(4.)
            .with_sprite_size(ICON_SIZE)
            .with_text_size(BASE_TEXT_SIZE)
            .with_text_color(RED_COLOR),
    );
    spans.push(
        Span::new()
            .with_sprite("icons_small", FOOD_ICON)
            .with_spacer(2.)
            .with_text_owned(format!("{}", world.resources.player_data.food))
            .with_spacer(4.)
            .with_sprite_size(ICON_SIZE)
            .with_text_size(BASE_TEXT_SIZE)
            .with_text_color(FOOD_COLOR),
    );
    spans.push(
        Span::new()
            .with_sprite("icons_small", FIGHT_ICON)
            .with_spacer(2.)
            .with_text_owned(format!(
                "{}/{}",
                world.resources.battle_state.wave,
                game_logic::globals::WAVE_COUNT
            ))
            .with_spacer(4.)
            .with_sprite_size(ICON_SIZE)
            .with_text_size(BASE_TEXT_SIZE)
            .with_text_color(BUTTON_TEXT_COLOR),
    );
    spans.push(
        Span::new()
            .with_sprite("icons_small", TOWN_ICON)
            .with_spacer(2.)
            .with_text_owned(format!("{}", world.resources.player_data.level))
            .with_sprite_size(ICON_SIZE)
            .with_text_size(BASE_TEXT_SIZE)
            .with_text_color(BUTTON_TEXT_COLOR),
    );

    let ov = 4.;
    let oh = 8.;
    let w: f32 = spans.iter().map(|s| s.width(context)).sum();

    let _ = context.graphics.draw_atlas_sprite(
        "ui",
        PANEL_SPRTE,
        state.status_origin,
        UI_Z,
        Vector2f::new(w + 2. * oh, BASE_TEXT_SIZE + 2. * ov),
        SpriteParams {
            slice: Some((4, Vector2f::splat(SPRITE_SIZE))),
            ..Default::default()
        },
    );

    let mut origin = state.status_origin + Vector2f::new(oh, ov);
    for span in spans {
        span.draw(origin, UI_Z, context);
        origin.x += span.width(context);
    }
}
