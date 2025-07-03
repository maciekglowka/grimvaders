use rogalik::prelude::*;

use game_graphics::input::ButtonState;

use crate::GameState;

#[derive(Default)]
pub(crate) struct GameWin;

impl Scene for GameWin {
    type Game = GameState;

    fn update(
        &mut self,
        game: &mut Self::Game,
        context: &mut Context,
    ) -> Option<SceneChange<Self::Game>> {
        game_graphics::utils::draw_background(context);
        super::draw_centered_text("We are safe, for now...", context);

        let input = crate::input::get_input_state(game.main_camera, context);
        if input.click == ButtonState::Released {
            return Some(SceneChange::Pop);
        }
        None
    }
}
