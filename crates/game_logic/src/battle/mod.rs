use crate::{
    commands,
    components::Position,
    events::InputEvent,
    globals::{BOARD_H, WAVES_VISIBLE},
    utils::{get_entity_at, spawn_by_name},
    GameEnv,
};

pub(crate) mod board;
mod npcs;
pub(crate) mod player;
mod systems;
pub(crate) mod utils;

#[derive(Default)]
pub enum BattleMode {
    #[default]
    Plan,
    Attack,
}

#[derive(Default)]
pub struct BattleState {
    pub mode: BattleMode,
    pub wave: u32,
}

pub fn battle_init(env: &mut GameEnv) {
    env.world.0.resources.player_data.level += 1;
    player::player_battle_init(&mut env.world);

    for i in 0..WAVES_VISIBLE - 1 {
        npcs::next_wave((BOARD_H + i) as i32, env);
    }
    next_turn(env);
}

pub fn battle_exit(env: &mut GameEnv) {
    // purge pending commands
    while handle_command_queue(env) {}
    player::player_battle_exit(&mut env.world);
}

pub fn battle_update(env: &mut GameEnv) {
    if handle_command_queue(env) {
        return;
    };
    match env.world.0.resources.battle_state.mode {
        BattleMode::Plan => {
            handle_input_events(env);
        }
        BattleMode::Attack => {
            if !npcs::next_attack(env) {
                next_turn(env);
            }
        }
    };
}

fn next_turn(env: &mut GameEnv) {
    env.world.0.resources.battle_state.mode = BattleMode::Plan;
    player::player_next_turn(env);
    npcs::slide_waves(env);
    npcs::next_wave((BOARD_H + WAVES_VISIBLE - 1) as i32, env);
}

fn handle_command_queue(env: &mut GameEnv) -> bool {
    env.scheduler.step(&mut env.world)
}

fn handle_input_events(env: &mut GameEnv) -> Option<()> {
    while let Some(event) = env.input.as_ref().unwrap().next() {
        match event {
            InputEvent::SummonUnit(entity, target) => {
                env.scheduler.send(commands::OrderSpawn(entity, target));
            }
            InputEvent::MoveUnit(entity, target) => {
                env.scheduler.send(commands::OrderMove(entity, target));
            }
            InputEvent::Done => env.world.0.resources.battle_state.mode = BattleMode::Attack,
            InputEvent::RedrawHand => env.scheduler.send(commands::RedrawHand),
            _ => (),
        }
    }
    Some(())
}
