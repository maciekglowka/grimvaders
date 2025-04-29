use rogalik::engine::{Context, Scene, SceneChange};

use crate::GameState;

pub(crate) struct GameScene;
impl GameScene {
    fn init_game(&mut self, game: &mut GameState) {
        game.env = game_logic::GameEnv::default();
        game.env.world = init_world();
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
    fn enter(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {
        self.init_game(game);
    }
    fn restore(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {
        self.init_game(game);
    }
}

fn init_world() -> game_logic::World {
    let mut world = game_logic::World::default();
    crate::assets::load_data(&mut world.0.resources.data);
    world
}
