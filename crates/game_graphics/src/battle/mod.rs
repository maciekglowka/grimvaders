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

#[derive(Default, Eq, PartialEq)]
enum InputMode {
    #[default]
    None,
    HandUnit(Entity),
    BoardUnit(Entity),
}

#[derive(Default)]
pub struct BattleGraphics {
    input_mode: InputMode,
    pub input_queue: ObservableQueue<InputEvent>,
    observers: Vec<Box<dyn ErasedObserver>>,
    unit_sprites: Vec<UnitSprite>,
    bubbles: Vec<Bubble>,
}

struct Observers {
    spawn_unit: Observer<commands::SpawnUnit>,
    move_unit: Observer<commands::MoveUnit>,
    attack: Observer<commands::Attack>,
    attack_town: Observer<commands::AttackTown>,
    kill: Observer<commands::Kill>,
}

pub fn battle_init(state: &mut BattleGraphics, env: &mut GameEnv) {
    subscribe_events(env, state);
}

pub fn battle_exit(_state: &mut BattleGraphics, _env: &mut GameEnv) {}

pub fn battle_draw(
    state: &mut BattleGraphics,
    world: &World,
    context: &mut Context,
    input_state: &InputState,
) -> bool {
    let _ = handle_events(state, world);
    let mut is_animating = false;

    board::draw_board(world, context);
    board::draw_board_description(world, input_state, context);

    for sprite in state.unit_sprites.iter_mut() {
        sprite.draw(world, context);
        is_animating |= animate_card_sprite(sprite, context.time.get_delta());
    }

    crate::draw::bubbles::update_bubbles(&mut state.bubbles, context);
    player::handle_player_ui(world, state, context, input_state, !is_animating);
    is_animating
}

fn handle_events(state: &mut BattleGraphics, world: &World) {
    let mut observers = std::mem::take(&mut state.observers);
    for observer in observers.iter_mut() {
        observer.handle(world, state);
    }
    state.observers = observers;
}

fn subscribe_events(env: &mut GameEnv, state: &mut BattleGraphics) {
    let mut observers: Vec<Box<dyn ErasedObserver>> = Vec::new();

    observers.push(Box::new(CommandObserver::new(
        &mut env.scheduler,
        |c: &commands::SpawnUnit, w, s| place_unit_sprite(c.0, c.1, w, &mut s.unit_sprites),
    )));
    observers.push(Box::new(CommandObserver::new(
        &mut env.scheduler,
        |c: &commands::MoveUnit, w, s| move_unit_sprite(c.0, w, &mut s.unit_sprites),
    )));
    observers.push(Box::new(CommandObserver::new(
        &mut env.scheduler,
        |c: &commands::Attack, w, s| attack_unit_sprite(c.0, c.1, w, &mut s.unit_sprites),
    )));
    observers.push(Box::new(CommandObserver::new(
        &mut env.scheduler,
        |c: &commands::AttackTown, w, s| attack_town(c.0, w, &mut s.unit_sprites),
    )));
    observers.push(Box::new(CommandObserver::new(
        &mut env.scheduler,
        |c: &commands::Kill, _, s| remove_unit_sprite(c.0, &mut s.unit_sprites),
    )));

    state.observers = observers;
}

trait ErasedObserver {
    fn handle(&mut self, world: &World, state: &mut BattleGraphics);
}
struct CommandObserver<T> {
    handler: Box<dyn FnMut(&T, &World, &mut BattleGraphics)>,
    observer: Observer<T>,
}
impl<T: 'static> CommandObserver<T> {
    pub fn new(
        scheduler: &mut Scheduler<World>,
        handler: impl FnMut(&T, &World, &mut BattleGraphics) + 'static,
    ) -> Self {
        Self {
            handler: Box::new(handler),
            observer: scheduler.observe(),
        }
    }
}
impl<T> ErasedObserver for CommandObserver<T> {
    fn handle(&mut self, world: &World, state: &mut BattleGraphics) {
        while self
            .observer
            .map_next(|c| (self.handler)(c, world, state))
            .is_some()
        {}
    }
}
