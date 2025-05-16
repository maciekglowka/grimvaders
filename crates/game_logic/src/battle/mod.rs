use crate::{commands, events::InputEvent, globals::WAVE_COUNT, GameEnv};

pub(crate) mod board;
mod npcs;
pub(crate) mod player;
mod systems;
pub(crate) mod utils;

#[derive(Default)]
pub enum BattleMode {
    #[default]
    Plan,
    Fight,
    Done,
}

#[derive(Default)]
pub struct BattleState {
    pub mode: BattleMode,
    pub wave: u32,
}

pub fn battle_init(env: &mut GameEnv) {
    env.world.resources.battle_state.mode = BattleMode::default();
    env.world.resources.battle_state.wave = 0;
    env.world.resources.player_data.level += 1;

    board::tiles_init(env);
    player::player_battle_init(&mut env.world);
    next_turn(env);
}

pub fn battle_exit(env: &mut GameEnv) {
    // purge pending commands
    while handle_command_queue(env) {}
    player::player_battle_exit(&mut env.world);
    board::clear_board(env);
}

pub fn battle_update(env: &mut GameEnv) {
    if handle_command_queue(env) {
        return;
    };
    match env.world.resources.battle_state.mode {
        BattleMode::Plan => {
            handle_input_events(env);
        }
        BattleMode::Fight => {
            if !npcs::next_attack(env) {
                next_turn(env);
            }
        }
        BattleMode::Done => (),
    };
}

fn next_turn(env: &mut GameEnv) {
    if env.world.resources.battle_state.wave >= WAVE_COUNT {
        env.world.resources.battle_state.mode = BattleMode::Done;
        return;
    }
    env.world.resources.battle_state.wave += 1;
    env.world.resources.battle_state.mode = BattleMode::Plan;
    player::player_next_turn(env);
    npcs::next_wave(env);
}

fn handle_command_queue(env: &mut GameEnv) -> bool {
    env.scheduler.step(&mut env.world)
}

fn handle_input_events(env: &mut GameEnv) -> Option<()> {
    while let Some(event) = env.input.as_ref().unwrap().next() {
        match event {
            InputEvent::SummonUnit(entity, target) => {
                env.scheduler.send(commands::SummonUnit(entity, target));
            }
            InputEvent::MoveUnit(entity, target) => {
                env.scheduler.send(commands::MoveUnit(entity, target));
            }
            InputEvent::Done => {
                env.scheduler.send(commands::Fight);
            }
            InputEvent::RedrawHand => env.scheduler.send(commands::RedrawHand),
            _ => (),
        }
    }
    Some(())
}
