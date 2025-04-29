use rogalik::prelude::*;
use wunderkammer::prelude::*;

mod board;
mod player;
mod utils;

use crate::{
    draw::{
        bubbles::Bubble,
        sprites::{
            animate_card_sprite, attack_town, attack_unit_sprite, move_unit_sprite,
            place_unit_sprite, remove_unit_sprite, UnitSprite,
        },
    },
    input::InputState,
};
use game_logic::{commands, GameEnv, InputEvent, World};

#[derive(Default)]
pub struct BattleGraphics {
    pub input_queue: ObservableQueue<InputEvent>,
    observers: Option<Observers>,
    unit_sprites: Vec<UnitSprite>,
    bubbles: Vec<Bubble>,
}

struct Observers {
    place_unit: Observer<commands::PlaceUnit>,
    attack: Observer<commands::Attack>,
    attack_town: Observer<commands::AttackTown>,
    kill: Observer<commands::Kill>,
}

pub fn battle_init(state: &mut BattleGraphics, env: &mut GameEnv) {
    let observers = Observers {
        place_unit: env.scheduler.observe(),
        attack: env.scheduler.observe(),
        attack_town: env.scheduler.observe(),
        kill: env.scheduler.observe(),
    };
    state.observers = Some(observers);
}

pub fn battle_exit(_state: &mut BattleGraphics, _env: &mut GameEnv) {
    // if let Some(events) = state.game_events.take() {
    //     world.0.resources.game_events.unsubscribe(events);
    // }
}

pub fn battle_draw(
    state: &mut BattleGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
) -> bool {
    let _ = handle_events(state, world);
    let mut is_animating = false;

    board::draw_board(context);

    for sprite in state.unit_sprites.iter_mut() {
        sprite.draw(world, context);
        is_animating |= animate_card_sprite(sprite, context.time.get_delta());
    }

    crate::draw::bubbles::update_bubbles(&mut state.bubbles, context);
    player::handle_player_ui(world, state, context, input_state, !is_animating);
    is_animating
}

fn handle_events(state: &mut BattleGraphics, world: &World) -> Option<()> {
    while state
        .observers
        .as_ref()?
        .place_unit
        .map_next(|a| place_unit_sprite(a.0, a.1, world, &mut state.unit_sprites))
        .is_some()
    {}

    while state
        .observers
        .as_ref()?
        .attack
        .map_next(|a| attack_unit_sprite(a.0, a.1, world, &mut state.unit_sprites))
        .is_some()
    {}

    while state
        .observers
        .as_ref()?
        .attack_town
        .map_next(|a| attack_town(a.0, world, &mut state.unit_sprites))
        .is_some()
    {}

    while state
        .observers
        .as_ref()?
        .kill
        .map_next(|a| remove_unit_sprite(a.0, &mut state.unit_sprites))
        .is_some()
    {}

    Some(())
}
