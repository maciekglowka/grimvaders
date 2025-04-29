use rogalik::prelude::*;

use crate::GameState;

#[derive(Default)]
pub(crate) struct Battle {
    graphics_state: game_graphics::battle::BattleGraphics,
}
impl Scene for Battle {
    type Game = GameState;

    fn enter(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {
        game.env.input = Some(self.graphics_state.input_queue.subscribe());
        game_graphics::battle::battle_init(&mut self.graphics_state, &mut game.env);
        game_logic::battle::battle_init(&mut game.env);
    }
    fn exit(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {
        game_graphics::battle::battle_exit(&mut self.graphics_state, &mut game.env);
        game_logic::battle::battle_exit(&mut game.env);
    }

    fn update(
        &mut self,
        game: &mut Self::Game,
        context: &mut Context,
    ) -> Option<SceneChange<Self::Game>> {
        let input = crate::input::get_input_state(game.main_camera, context);
        if !game_graphics::battle::battle_draw(
            &mut self.graphics_state,
            &game.env.world,
            context,
            &input,
        ) {
            game_logic::battle::battle_update(&mut game.env);
        }

        // match game.env.world.0.resources.player_data.status {
        //     PlayerStatus::GameOver => {
        //         Some(SceneChange::Switch(Box::new(super::game_over::GameOver)))
        //     }
        //     PlayerStatus::Descend => {
        //         Some(SceneChange::Switch(Box::new(super::shop::Shop::default())))
        //     }
        //     _ => None,
        // }
        None
    }
}
