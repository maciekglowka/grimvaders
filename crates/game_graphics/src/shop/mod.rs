use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::{shop::ShopState, InputEvent, World};

use crate::{
    draw::units::{draw_deck_unit, draw_entity_description},
    globals::{BASE_TEXT_SIZE, BUTTON_SIZE, GAP, SPRITE_SIZE},
    input::InputState,
    ui::{Button, Span},
    utils::get_viewport_bounds,
};

#[derive(Default)]
pub struct ShopGraphics {
    pub input_queue: ObservableQueue<InputEvent>,
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
    let button_w = BUTTON_SIZE * 3.;
    let unit_offset = Vector2f::new(0.5 * (button_w - SPRITE_SIZE), 0.);
    let w = button_w + GAP;

    let mut origin = center - Vector2f::new(0.5 * logic_state.choices.len() as f32 * w, 0.);

    for i in 0..logic_state.choices.len() {
        if let Some(entity) = &logic_state.choices[i] {
            let button = Button::new(
                origin - Vector2f::new(0., BUTTON_SIZE + GAP),
                Vector2f::new(button_w, BUTTON_SIZE),
                0,
            )
            .with_span(Span::new().with_text_borrowed("Pick"));
            button.draw(context, input_state);

            if button.clicked(input_state) {
                state.input_queue.push(InputEvent::PickUnit(i));
            }

            draw_deck_unit(*entity, origin + unit_offset, 0, world, context);

            if let Some(name) = world.components.name.get(*entity) {
                let offset = Vector2f::new(
                    0.5 * (button_w
                        - context
                            .graphics
                            .text_dimensions("default", name, BASE_TEXT_SIZE)
                            .x),
                    SPRITE_SIZE + 3. * GAP,
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

            if crate::utils::is_mouse_over(
                origin + unit_offset,
                Vector2f::splat(SPRITE_SIZE),
                input_state,
            ) || button.mouse_over(input_state)
            {
                draw_entity_description(*entity, world, context);
            }
        }

        origin.x += w;
    }

    let done = Button::new(
        bounds.0 + Vector2f::splat(GAP),
        Vector2f::new(button_w, BUTTON_SIZE),
        0,
    )
    .with_span(Span::new().with_text_borrowed("Skip"));
    done.draw(context, input_state);
    if done.clicked(input_state) {
        state.input_queue.push(InputEvent::Done);
    }
}
