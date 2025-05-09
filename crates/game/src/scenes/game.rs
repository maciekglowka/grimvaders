use rogalik::engine::{Context, Scene, SceneChange};

use crate::{assets::load_data, GameState};

pub(crate) struct GameScene;
impl GameScene {
    fn init_game(&mut self, game: &mut GameState, context: &mut Context) {
        game.env = game_logic::GameEnv::default();
        game.env.world = game_logic::World::default();
        load_data(
            &game.data_assets,
            &mut game.env.world.0.resources.data,
            context,
            false,
        );
        game_logic::startup::init_game(&mut game.env);
    }
}
impl Scene for GameScene {
    type Game = GameState;

    fn update(
        &mut self,
        _game: &mut Self::Game,
        _context: &mut Context,
    ) -> Option<SceneChange<Self::Game>> {
        Some(SceneChange::Push(
            Box::new(super::battle::Battle::default()),
        ))
    }
    fn enter(&mut self, game: &mut Self::Game, context: &mut rogalik::engine::Context) {
        self.init_game(game, context);
    }
    fn restore(&mut self, game: &mut Self::Game, context: &mut rogalik::engine::Context) {
        self.init_game(game, context);
    }
}
