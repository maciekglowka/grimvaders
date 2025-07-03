use rogalik::prelude::*;

use game_graphics::input::ButtonState;

use crate::GameState;

#[derive(Default)]
pub(crate) struct GameOver;

impl Scene for GameOver {
    type Game = GameState;

    fn update(
        &mut self,
        game: &mut Self::Game,
        context: &mut Context,
    ) -> Option<SceneChange<Self::Game>> {
        game_graphics::utils::draw_background(context);
        super::draw_centered_text("You have failed...", context);

        let input = crate::input::get_input_state(game.main_camera, context);
        if input.click == ButtonState::Released {
            return Some(SceneChange::Pop);
        }
        None
    }
}
