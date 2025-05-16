use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::{shop::ShopState, InputEvent, World};

use crate::{
    draw::units::{draw_deck_unit, draw_entity_description},
    globals::{BUTTON_SIZE, GAP, SPRITE_SIZE},
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
    let bounds = get_viewport_bounds(context);
    let center = 0.5 * (bounds.0 + bounds.1);
    let button_w = BUTTON_SIZE * 3.;
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

            draw_deck_unit(*entity, origin, 0, world, context);

            if crate::utils::is_mouse_over(origin, Vector2f::splat(SPRITE_SIZE), input_state) {
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

    draw_status(world, context);
}

fn draw_status(world: &World, context: &mut Context) {
    let bounds = get_viewport_bounds(context);
    // let _ = context.graphics.draw_text(
    //     "default",
    //     &format!("G: {}", world.0.resources.player_data.gold,),
    //     Vector2f::new(bounds.0.x + GAP, bounds.1.y - GAP - BASE_TEXT_SIZE),
    //     0,
    //     BASE_TEXT_SIZE,
    //     SpriteParams::default(),
    // );
}
