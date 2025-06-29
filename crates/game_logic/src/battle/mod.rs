use std::collections::VecDeque;
use wunderkammer::prelude::*;

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
    on_fight_queue: VecDeque<Entity>,
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

    if systems::handle_killed(env) {
        return;
    }

    match env.world.resources.battle_state.mode {
        BattleMode::Plan => {
            handle_input_events(env);
        }
        BattleMode::Fight => {
            if systems::handle_on_fight(env) {
                return;
            }
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
    systems::reset_trigger_limits(&mut env.world);
    npcs::next_wave(env);
}

fn fight_start(env: &mut GameEnv) {
    // Change battle mode
    env.world.resources.battle_state.mode = BattleMode::Fight;

    // Collect on fight queue
    let mut on_fight = query_iter!(env.world, With(position, on_fight))
        .map(|(e, p, s)| (e, *p, s.to_string()))
        .collect::<Vec<_>>();

    // Apply consistent front to back order.
    on_fight.sort_by(|a, b| b.1.y.cmp(&a.1.y).then_with(|| a.1.x.cmp(&b.1.x)));
    env.world.resources.battle_state.on_fight_queue = on_fight.iter().map(|(e, _, _)| *e).collect();
}

fn handle_command_queue(env: &mut GameEnv) -> bool {
    env.scheduler.step(&mut env.world)
}

fn handle_input_events(env: &mut GameEnv) -> Option<()> {
    while let Some(event) = env.input.as_ref().unwrap().next() {
        match event {
            InputEvent::SummonPlayer(idx, target) => {
                let name = env.world.resources.player_data.deck[idx].to_string();
                env.scheduler.send(commands::SummonPlayer(name, target));
            }
            InputEvent::MoveUnit(entity, target) => {
                env.scheduler.send(commands::MoveUnit(entity, target));
            }
            InputEvent::Done => {
                fight_start(env);
            }
            _ => (),
        }
    }
    Some(())
}
