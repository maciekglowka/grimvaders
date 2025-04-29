use rogalik::prelude::*;
use wunderkammer::prelude::*;

use game_logic::{InputEvent, World};

use crate::{
    globals::{BASE_TEXT_SIZE, BUTTON_SIZE, GAP, TILE_SIZE},
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
    let bounds = get_viewport_bounds(context);
    let center = 0.5 * (bounds.0 + bounds.1);
    let w = TILE_SIZE + GAP;

    let _ = context.graphics.draw_text(
        "default",
        "Too many cards in the deck. Please discard {}.",
        Vector2f::new(bounds.0.x + GAP, bounds.1.y - GAP - BASE_TEXT_SIZE),
        0,
        BASE_TEXT_SIZE,
        SpriteParams::default(),
    );

    let mut origin = center + Vector2f::new(-2.5 * w, TILE_SIZE + GAP);

    for (i, entity) in world.0.resources.player_data.draw.iter().enumerate() {
        let Some(name) = world.0.components.name.get(*entity) else {
            continue;
        };

        // draw_action_card(
        //     name,
        //     origin,
        //     state.selected == Some(*entity),
        //     world,
        //     context,
        // );
        // if is_card_clicked(origin, input_state) {
        //     if state.selected == Some(*entity) {
        //         state.selected = None
        //     } else {
        //         state.selected = Some(*entity)
        //     }
        // }

        origin.x += w;
        if i % 5 == 4 {
            origin.y -= TILE_SIZE + GAP;
            origin.x -= 5. * w;
        }
    }

    if let Some(entity) = state.selected {
        let confirm = Button::new(
            bounds.0 + Vector2f::splat(GAP),
            Vector2f::new(3. * BUTTON_SIZE, BUTTON_SIZE),
            0,
        )
        .with_span(Span::new().with_text_borrowed("Remove"));
        confirm.draw(context, input_state);

        if confirm.clicked(input_state) {
            // state.input_queue.push(InputEvent::DiscardCard(entity));
        }
    }
}
