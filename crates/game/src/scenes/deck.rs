use rogalik::prelude::*;

use crate::GameState;

#[derive(Default)]
pub(crate) struct Deck {
    graphics_state: game_graphics::deck::DeckGraphics,
}
impl Scene for Deck {
    type Game = GameState;

    fn enter(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {
        game.env.input = Some(self.graphics_state.input_queue.subscribe());
        game_logic::deck::deck_init(&mut game.env);
    }
    fn exit(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {}

    fn update(
        &mut self,
        game: &mut Self::Game,
        context: &mut Context,
    ) -> Option<SceneChange<Self::Game>> {
        let input = crate::input::get_input_state(game.main_camera, context);
        game_graphics::deck::deck_draw(&mut self.graphics_state, &game.env.world, context, &input);
        let done = game_logic::deck::deck_update(&mut game.env);

        if done {
            return Some(SceneChange::Switch(Box::new(
                super::battle::Battle::default(),
            )));
        }
        None
    }
}
