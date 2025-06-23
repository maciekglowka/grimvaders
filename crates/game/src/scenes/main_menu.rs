use rogalik::prelude::*;

use game_graphics::input::ButtonState;

use crate::GameState;

pub(crate) struct MainMenu;
impl Scene for MainMenu {
    type Game = GameState;

    fn update(
        &mut self,
        game: &mut Self::Game,
        context: &mut Context,
    ) -> Option<SceneChange<Self::Game>> {
        let _ = context.graphics.draw_text(
            "default",
            "RGLK",
            Vector2f::ZERO,
            0,
            16.,
            SpriteParams::default(),
        );

        let input = crate::input::get_input_state(game.main_camera, context);
        if input.click == ButtonState::Released {
            let _ = context.audio.play("pick", false);
            return Some(SceneChange::Push(Box::new(super::game::GameScene)));
        }

        None
    }
}
