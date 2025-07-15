use rogalik::prelude::*;

use game_graphics::input::ButtonState;

use crate::GameState;

const TITLE_SIZE: f32 = 256.;

pub(crate) struct MainMenu;
impl Scene for MainMenu {
    type Game = GameState;

    fn update(
        &mut self,
        game: &mut Self::Game,
        context: &mut Context,
        scenes: &mut SceneController<Self::Game>,
    ) {
        game_graphics::utils::draw_background(context);

        let bounds = game_graphics::utils::get_viewport_bounds(context);
        let center = 0.5 * (bounds.0 + bounds.1);

        let _ = context.graphics.draw_sprite(
            "main_title",
            center - Vector2f::splat(0.5 * TITLE_SIZE),
            0,
            Vector2f::splat(TITLE_SIZE),
            SpriteParams::default(),
        );

        let input = crate::input::get_input_state(game.main_camera, context);
        if input.click == ButtonState::Released {
            scenes.push(Box::new(super::game::GameScene));
        }
    }
}
