use rogalik::prelude::*;

use game_logic::{get_unit_at, is_on_board, InputEvent, World};

use crate::{
    draw::units::{draw_deck_unit, draw_entity_description},
    globals::{BASE_TEXT_SIZE, BUTTON_SIZE, GAP, OVERLAY_Z, SPRITE_SIZE, TILE_SIZE, UI_Z},
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
    if take_input {
        handle_input_player(state, world, context, input_state);
        draw_cursor(state, world, context, input_state);
    }
}

fn handle_input_player(
    state: &mut super::BattleGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
) {
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

    let bounds = get_viewport_bounds(context);
    let fight = Button::new(
        bounds.0 + Vector2f::splat(GAP),
        Vector2f::new(3. * BUTTON_SIZE, BUTTON_SIZE),
        UI_Z,
    )
    .with_span(Span::new().with_text_borrowed("Fight!"));
    fight.draw(context, input_state);
    if fight.clicked(input_state) {
        state.input_queue.push(InputEvent::Done);
    }
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
                0,
                tile_to_world(position),
                OVERLAY_Z,
                Vector2f::splat(TILE_SIZE),
                SpriteParams::default(),
            );
        }
    }

    let tile = world_to_tile(input_state.mouse_world_position);

    if !is_on_board(tile) {
        return;
    }

    let _ = context.graphics.draw_atlas_sprite(
        "tiles",
        0,
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
    let mut origin = Vector2f::new(bounds.1.x - 3. * SPRITE_SIZE, bounds.0.y + GAP);

    origin.y += BUTTON_SIZE + GAP;

    for &entity in world.0.resources.player_data.hand.iter() {
        let selected = state.input_mode == InputMode::HandUnit(entity);
        let mut button = Button::new(origin, Vector2f::splat(2. * SPRITE_SIZE), UI_Z);
        if selected {
            button = button.with_sprite("sprites", 726);
        }
        button.draw(context, input_state);
        draw_deck_unit(
            entity,
            origin + Vector2f::splat(0.5 * SPRITE_SIZE),
            UI_Z + 1,
            world,
            context,
        );

        if button.mouse_over(input_state) {
            draw_entity_description(entity, world, context);
        }

        if take_input && button.clicked(input_state) {
            if selected {
                state.input_mode = InputMode::None
            } else {
                state.input_mode = InputMode::HandUnit(entity);
            }
        }

        origin.y += 3. * SPRITE_SIZE;
    }

    let redraw = Button::new(
        Vector2f::new(bounds.1.x - 3. * BUTTON_SIZE - GAP, bounds.0.y + GAP),
        Vector2f::new(3. * BUTTON_SIZE, BUTTON_SIZE),
        UI_Z,
    )
    .with_span(Span::new().with_text_borrowed("Redraw"));
    redraw.draw(context, input_state);
    if redraw.clicked(input_state) {
        state.input_queue.push(InputEvent::RedrawHand);
    }
}

pub(super) fn draw_status(state: &super::BattleGraphics, world: &World, context: &mut Context) {
    let text = format!(
        "Health: {} | Food: {} | Wave: {}/{}",
        world.0.resources.player_data.health,
        world.0.resources.player_data.food,
        world.0.resources.battle_state.wave,
        game_logic::globals::WAVE_COUNT
    );
    let w = context
        .graphics
        .text_dimensions("default", &text, BASE_TEXT_SIZE)
        .x;
    let _ = context.graphics.draw_text(
        "default",
        &text,
        state.status_origin - Vector2f::new(0.5 * w, 0.),
        UI_Z,
        BASE_TEXT_SIZE,
        SpriteParams::default(),
    );
}
