use rogalik::prelude::*;

use game_logic::{components::Targeting, is_on_board, InputEvent, World};

use crate::{
    draw::units::draw_deck_unit,
    globals::{BASE_TEXT_SIZE, BUTTON_SIZE, GAP, SPRITE_SIZE, UI_Z},
    input::{ButtonState, InputState},
    ui::{Button, Span},
    utils::get_viewport_bounds,
};

pub(super) fn handle_player_ui(
    world: &World,
    state: &mut super::BattleGraphics,
    context: &mut Context,
    input_state: &InputState,
    take_input: bool,
) {
    draw_status(world, context);
    handle_hand(state, world, context, input_state, take_input);
    if take_input {
        handle_input_player(state, world, context, input_state);
    }
}

fn handle_input_player(
    state: &mut super::BattleGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
) {
}

fn handle_hand(
    state: &mut super::BattleGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
    take_input: bool,
) {
    let bounds = get_viewport_bounds(context);
    let mut origin = bounds.0 + Vector2f::splat(GAP);

    origin.y += BUTTON_SIZE + GAP;

    for &entity in world.0.resources.player_data.hand.iter() {
        draw_deck_unit(entity, origin, world, context);

        // if take_input && is_card_clicked(origin, input_state) {
        //     if InputMode::Action(entity) == state.input_mode {
        //         restore_input_mode(state);
        //     } else {
        //         if let Some(action) = world.0.components.action.get(entity) {
        //             if action.targeting != Targeting::None {
        //                 set_input_mode(InputMode::Action(entity), state);
        //             } else {
        //                 state
        //                     .input_queue
        //                     .push(InputEvent::PerformAction(entity, None));
        //             }
        //         }
        //     }
        // }

        origin.y += 2. * SPRITE_SIZE;
    }
}

pub(super) fn draw_status(world: &World, context: &mut Context) {
    let bounds = get_viewport_bounds(context);
    let _ = context.graphics.draw_text(
        "default",
        &format!("H: {}", world.0.resources.player_data.health,),
        Vector2f::new(bounds.0.x + GAP, bounds.1.y - GAP - BASE_TEXT_SIZE),
        UI_Z,
        BASE_TEXT_SIZE,
        SpriteParams::default(),
    );
}
