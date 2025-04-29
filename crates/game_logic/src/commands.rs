use rand::prelude::*;
use rune::{ast::Comma, Any};
use wunderkammer::prelude::*;

use crate::{
    components::{Position, Targeting},
    globals::{BOARD_H, BOARD_W, HAND_SIZE},
    utils::{get_entity_at, spawn_by_name},
    world::{Ent, World},
};

// Commands

pub struct PlaceUnit(pub Entity, pub Position);
pub struct Attack(pub Entity, pub Entity);
pub struct AttackTown(pub Entity);
pub struct Damage(pub Entity, pub u32);
pub struct Heal(pub Entity, pub u32);
pub struct Kill(pub Entity);

// Rune

#[derive(Any, Clone, Copy, Debug)]
pub enum RuneCommand {
    #[rune(constructor)]
    Damage(#[rune(get)] Ent, #[rune(get)] u32),
    #[rune(constructor)]
    Heal(#[rune(get)] Ent, #[rune(get)] u32),
}
impl RuneCommand {
    pub fn send(&self, cx: &mut SchedulerContext) {
        match self {
            Self::Damage(e, v) => cx.send(Damage(e.into(), *v)),
            Self::Heal(e, v) => cx.send(Heal(e.into(), *v)),
        }
    }
}

// Register

pub(crate) fn register_handlers(scheduler: &mut Scheduler<World>) {
    scheduler.add_system(place_unit);
    scheduler.add_system(attack);
    scheduler.add_system(attack_town);
    scheduler.add_system(damage);
    scheduler.add_system(heal);
    scheduler.add_system(kill);
}

// Handlers

fn place_unit(cmd: &mut PlaceUnit, world: &mut World) -> Result<(), CommandError> {
    if get_entity_at(world, cmd.1).is_some() {
        return Err(CommandError::Break);
    }
    world.0.components.position.insert(cmd.0, cmd.1);
    Ok(())
}

fn attack(
    cmd: &mut Attack,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    let health_0 = world
        .0
        .components
        .health
        .get(cmd.0)
        .ok_or(CommandError::Break)?;
    let health_1 = world
        .0
        .components
        .health
        .get(cmd.1)
        .ok_or(CommandError::Break)?;

    cx.send(Damage(cmd.1, health_0.current()));
    cx.send(Damage(cmd.0, health_1.current()));
    Ok(())
}

fn attack_town(
    cmd: &mut AttackTown,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    let health = world
        .0
        .components
        .health
        .get(cmd.0)
        .ok_or(CommandError::Break)?;
    world.0.resources.player_data.health = world
        .0
        .resources
        .player_data
        .health
        .saturating_sub(health.current());

    // npc is removed
    cx.send(Kill(cmd.0));
    Ok(())
}

fn damage(
    cmd: &mut Damage,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    let health = world
        .0
        .components
        .health
        .get_mut(cmd.0)
        .ok_or(CommandError::Break)?;

    health.sub(cmd.1);
    if health.current() == 0 {
        cx.send(Kill(cmd.0));
    }
    Ok(())
}

fn heal(cmd: &mut Heal, world: &mut World) -> Result<(), CommandError> {
    let health = world
        .0
        .components
        .health
        .get_mut(cmd.0)
        .ok_or(CommandError::Break)?;
    health.add(cmd.1);
    Ok(())
}

fn kill(cmd: &mut Kill, world: &mut World) -> Result<(), CommandError> {
    // TODO if player -> return to hand
    world.0.despawn(cmd.0);
    Ok(())
}
