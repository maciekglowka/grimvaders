use crate::GameEnv;

pub fn init_game(env: &mut GameEnv) {
    env.world.0.resources.vm = Some(
        crate::scripting::init_rune(&env.world).expect("Script engine initialization failed!"),
    );
    crate::commands::register_handlers(&mut env.scheduler);
    crate::player::player_game_init(&mut env.world);
}
