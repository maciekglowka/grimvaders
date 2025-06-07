use rogalik::prelude::*;
use wunderkammer::prelude::*;

mod board;
mod player;
mod utils;

use crate::{
    draw::{
        bubbles::Bubble,
        sprites::{
            animate_unit_sprite, attack_town, attack_unit_sprite, get_unit_sprite,
            kill_unit_sprite, move_unit_sprite, place_unit_sprite, purge_unit_sprites,
            remove_unit_sprite, UnitSprite,
        },
    },
    globals::{BASE_TEXT_SIZE, FOOD_COLOR, GAP, RED_COLOR, TILE_SIZE},
    input::InputState,
    utils::get_viewport_bounds,
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
    status_origin: Vector2f,
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
    let bounds = get_viewport_bounds(context);
    state.status_origin = bounds.0 + Vector2f::splat(GAP);

    crate::utils::draw_background(context);

    handle_events(state, world);
    let mut is_animating = false;

    board::draw_board(world, context);
    board::draw_board_description(world, input_state, context);

    for sprite in state.unit_sprites.iter_mut() {
        sprite.draw(world, context);
        is_animating |= animate_unit_sprite(sprite, context.time.get_delta());
    }
    purge_unit_sprites(&mut state.unit_sprites);

    is_animating |= crate::draw::bubbles::update_bubbles(&mut state.bubbles, context);
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
        |c: &commands::ChangeHealth, _, s| {
            if let Some(sprite) = get_unit_sprite(c.0, &s.unit_sprites) {
                s.bubbles.push(Bubble::new(
                    sprite.origin + Vector2f::new(0., TILE_SIZE),
                    RED_COLOR,
                    Some(format!("{:+}", c.1)),
                    None,
                ));
            }
        },
    )));
    observers.push(Box::new(CommandObserver::new(
        &mut env.scheduler,
        |c: &commands::ChangeFood, _, s| {
            s.bubbles.push(Bubble::new(
                s.status_origin + Vector2f::splat(2. * BASE_TEXT_SIZE),
                FOOD_COLOR,
                Some((if c.0 < 0 { "-" } else { "+" }).to_string()),
                Some(1),
            ));
        },
    )));
    observers.push(Box::new(CommandObserver::new(
        &mut env.scheduler,
        |c: &commands::AttackTown, w, s| {
            attack_town(c.0, w, &mut s.unit_sprites);
            s.bubbles.push(Bubble::new(
                s.status_origin + Vector2f::splat(2. * BASE_TEXT_SIZE),
                RED_COLOR,
                Some("-".to_string()),
                Some(0),
            ));
        },
    )));
    observers.push(Box::new(CommandObserver::new(
        &mut env.scheduler,
        |c: &commands::Kill, _, s| kill_unit_sprite(c.0, &mut s.unit_sprites),
    )));
    observers.push(Box::new(CommandObserver::new(
        &mut env.scheduler,
        |c: &commands::RemoveUnit, _, s| remove_unit_sprite(c.0, &mut s.unit_sprites),
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
