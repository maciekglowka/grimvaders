use rogalik::prelude::*;

use game_logic::GameMode;

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
        game: &mut Self::Game,
        _context: &mut Context,
        scenes: &mut SceneController<Self::Game>,
    ) {
        match game.env.world.resources.game_mode {
            GameMode::GameOver => scenes.switch(Box::new(super::game_over::GameOver)),
            GameMode::Win => scenes.switch(Box::new(super::win::GameWin)),
            _ => (), // _ => scenes.push(Box::new(super::battle::Battle::default())),
        }
    }
    fn enter(
        &mut self,
        game: &mut Self::Game,
        context: &mut Context,
        scenes: &mut SceneController<Self::Game>,
    ) {
        game.env.world.resources.game_mode = GameMode::Init;
        let _ = context.graphics.set_postprocess_strength("noise", 0.);
        self.init_game(game, context);
        scenes.push(Box::new(super::battle::Battle::default()));
    }
    fn exit(
        &mut self,
        _game: &mut Self::Game,
        context: &mut Context,
        _scenes: &mut SceneController<Self::Game>,
    ) {
        let _ = context.graphics.set_postprocess_strength("noise", 1.);
    }
}
