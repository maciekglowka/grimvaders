use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::{shop::ShopState, InputEvent, World};

use crate::{
    draw::units::draw_deck_button,
    globals::{ACTION_BUTTON_W, BASE_TEXT_SIZE, BUTTON_SIZE, DECK_BUTTON_W, GAP},
    input::InputState,
    ui::{Button, Span},
    utils::get_viewport_bounds,
};

#[derive(Default)]
pub struct ShopGraphics {
    pub input_queue: ObservableQueue<InputEvent>,
    selected: Option<usize>,
}

pub fn shop_draw(
    logic_state: &ShopState,
    state: &mut ShopGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
) {
    crate::utils::draw_background(context);

    let bounds = get_viewport_bounds(context);
    let center = 0.5 * (bounds.0 + bounds.1);
    let w = 2. * DECK_BUTTON_W + GAP;

    let mut origin = center - Vector2f::new(0.5 * logic_state.choices.len() as f32 * w, 0.);
    let card_offset = Vector2f::new(0.5 * (w - DECK_BUTTON_W), 0.);

    for i in 0..logic_state.choices.len() {
        if let Some(entity) = &logic_state.choices[i] {
            let selected = state.selected == Some(i);
            let clicked = draw_deck_button(
                *entity,
                origin + card_offset,
                0,
                selected,
                world,
                context,
                input_state,
            );

            if clicked {
                if state.selected == Some(i) {
                    state.selected = None
                } else {
                    state.selected = Some(i)
                }
            }

            if let Some(name) = world.components.name.get(*entity) {
                let offset = Vector2f::new(
                    0.5 * (w - context
                        .graphics
                        .text_dimensions("default", name, BASE_TEXT_SIZE)
                        .x),
                    -(BASE_TEXT_SIZE + GAP),
                );

                let _ = context.graphics.draw_text(
                    "default",
                    name,
                    origin + offset,
                    0,
                    BASE_TEXT_SIZE,
                    SpriteParams::default(),
                );
            }
        }

        origin.x += w;
    }

    let done = Button::new(
        bounds.0 + Vector2f::splat(GAP),
        Vector2f::new(ACTION_BUTTON_W, BUTTON_SIZE),
        0,
    )
    .with_span(Span::new().with_text_borrowed("Skip"));
    done.draw(context, input_state);
    if done.clicked(input_state) {
        state.input_queue.push(InputEvent::Done);
    }

    if let Some(i) = state.selected {
        let confirm = Button::new(
            Vector2f::new(bounds.1.x - ACTION_BUTTON_W - GAP, bounds.0.y + GAP),
            Vector2f::new(ACTION_BUTTON_W, BUTTON_SIZE),
            0,
        )
        .with_span(Span::new().with_text_borrowed("Pick"));
        confirm.draw(context, input_state);

        if confirm.clicked(input_state) {
            state.input_queue.push(InputEvent::PickUnit(i));
        }
    }
}
