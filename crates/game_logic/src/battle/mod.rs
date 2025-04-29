use crate::{
    commands,
    components::Position,
    events::InputEvent,
    globals::BOARD_H,
    utils::{get_entity_at, spawn_by_name},
    GameEnv,
};

pub(crate) mod board;
mod npcs;
pub(crate) mod player;
mod systems;
pub(crate) mod utils;

#[derive(Default)]
pub enum BattleState {
    #[default]
    Plan,
    Attack,
}

pub fn battle_init(env: &mut GameEnv) {
    env.world.0.resources.player_data.level += 1;
    player::player_battle_init(&mut env.world);

    // TEMP
    let rat = spawn_by_name("Rat", &mut env.world).unwrap();
    env.world.0.components.npc.insert(rat, ());
    env.scheduler
        .send(commands::PlaceUnit(rat, Position::new(2, BOARD_H as i32)));
    let rat = spawn_by_name("Rat", &mut env.world).unwrap();
    env.world.0.components.npc.insert(rat, ());
    env.scheduler.send(commands::PlaceUnit(
        rat,
        Position::new(2, BOARD_H as i32 + 1),
    ));
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
    match env.world.0.resources.battle_state {
        BattleState::Plan => {
            handle_input_events(env);
        }
        BattleState::Attack => {
            npcs::next_attack(env);
        }
    };
}

fn handle_command_queue(env: &mut GameEnv) -> bool {
    env.scheduler.step(&mut env.world)
}

fn handle_input_events(env: &mut GameEnv) -> Option<()> {
    while let Some(event) = env.input.as_ref().unwrap().next() {
        match event {
            // InputEvent::PlayerMove(target) => {
            //     if let Some(entity) = get_entity_at(&env.world, target) {
            //         env.scheduler.send(commands::CardInteract(entity));
            //     }
            // }
            // InputEvent::PerformAction(entity, target) => {
            //     env.scheduler.send(commands::PerformAction(entity, target));
            // }
            // InputEvent::RedrawHand => env.scheduler.send(commands::RedrawHand),
            _ => (),
        }
    }
    Some(())
}
