use rogalik::prelude::*;

use crate::GameState;

#[derive(Default)]
pub(crate) struct Shop {
    logic_state: game_logic::shop::ShopState,
    graphics_state: game_graphics::shop::ShopGraphics,
}
impl Scene for Shop {
    type Game = GameState;

    fn enter(
        &mut self,
        game: &mut Self::Game,
        _context: &mut Context,
        _scenes: &mut SceneController<Self::Game>,
    ) {
        game.env.input = Some(self.graphics_state.input_queue.subscribe());
        game_logic::shop::shop_init(&mut self.logic_state, &mut game.env);
    }
    fn exit(
        &mut self,
        game: &mut Self::Game,
        _context: &mut Context,
        _scenes: &mut SceneController<Self::Game>,
    ) {
        game_logic::shop::shop_exit(&mut self.logic_state, &mut game.env);
    }

    fn update(
        &mut self,
        game: &mut Self::Game,
        context: &mut Context,
        scenes: &mut SceneController<Self::Game>,
    ) {
        let input = crate::input::get_input_state(game.main_camera, context);
        game_graphics::shop::shop_draw(
            &self.logic_state,
            &mut self.graphics_state,
            &game.env.world,
            context,
            &input,
        );
        game_logic::shop::shop_update(&mut self.logic_state, &mut game.env);

        if self.logic_state.done {
            scenes.switch(Box::new(super::deck::Deck::default()));
        }
    }
}
