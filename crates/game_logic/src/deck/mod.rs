use crate::{events::InputEvent, globals::MAX_DECK_SIZE, GameEnv};

pub fn deck_init(env: &mut GameEnv) {
    // restore draw pile, to verify the size
    crate::player::reset_deck(&mut env.world);
}

pub fn deck_update(env: &mut GameEnv) -> bool {
    if env.world.resources.player_data.draw.len() <= MAX_DECK_SIZE {
        return true;
    }
    while let Some(event) = env.input.as_ref().unwrap().next() {
        match event {
            InputEvent::DiscardUnit(entity) => {
                env.world
                    .resources
                    .player_data
                    .draw
                    .retain(|e| *e != entity);
            }
            _ => (),
        }
    }
    false
}
