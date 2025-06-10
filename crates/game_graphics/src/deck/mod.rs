use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::{InputEvent, World};

use crate::{
    draw::units::{draw_deck_unit, draw_entity_description},
    globals::{BASE_TEXT_SIZE, BUTTON_CLICK_SHIFT, BUTTON_SIZE, GAP, SPRITE_SIZE, TILE_SIZE},
    input::InputState,
    ui::{Button, Span},
    utils::get_viewport_bounds,
};

#[derive(Default)]
pub struct DeckGraphics {
    pub input_queue: ObservableQueue<InputEvent>,
    selected: Option<Entity>,
}

pub fn deck_draw(
    state: &mut DeckGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
) {
    crate::utils::draw_background(context);

    let bounds = get_viewport_bounds(context);
    let center = 0.5 * (bounds.0 + bounds.1);
    let w = SPRITE_SIZE + GAP;

    let _ = context.graphics.draw_text(
        "default",
        "Too many cards in the deck. Please discard.",
        bounds.0 + Vector2f::splat(GAP),
        0,
        BASE_TEXT_SIZE,
        SpriteParams::default(),
    );

    let mut origin = center + Vector2f::new(-2.5 * w, TILE_SIZE + GAP);

    for (i, entity) in world.0.resources.player_data.draw.iter().enumerate() {
        let selected = state.selected == Some(*entity);

        let mut button = Button::new(origin, Vector2f::new(SPRITE_SIZE, BUTTON_SIZE), 0);
        if selected {
            button = button.with_sprite("ui", 2);
        }
        button.draw(context, input_state);

        let unit_offset = if button.pressed(input_state) {
            BUTTON_CLICK_SHIFT
        } else {
            0.
        };
        draw_deck_unit(
            *entity,
            origin + Vector2f::new(0., 0.5 * BUTTON_SIZE - unit_offset),
            0,
            world,
            context,
        );

        if button.mouse_over(input_state) {
            draw_entity_description(*entity, world, context);
        }

        if button.clicked(input_state) {
            if state.selected == Some(*entity) {
                state.selected = None
            } else {
                state.selected = Some(*entity)
            }
        }

        origin.x += w;
        if i % 5 == 4 {
            origin.y -= 1.75 * SPRITE_SIZE;
            origin.x -= 5. * w;
        }
    }

    if let Some(entity) = state.selected {
        let confirm = Button::new(
            Vector2f::new(bounds.1.x - 3. * BUTTON_SIZE - GAP, bounds.0.y + GAP),
            Vector2f::new(3. * BUTTON_SIZE, BUTTON_SIZE),
            0,
        )
        .with_span(Span::new().with_text_borrowed("Remove"));
        confirm.draw(context, input_state);

        if confirm.clicked(input_state) {
            state.input_queue.push(InputEvent::DiscardUnit(entity));
        }
    }
}
