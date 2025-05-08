use anyhow::Result;
use rand::prelude::*;
use rune::{ast::Comma, Any};
use wunderkammer::prelude::*;

use crate::{
    battle::BattleMode,
    components::Position,
    scripting::run_command_script,
    utils::get_entity_at,
    world::{Ent, World},
};

// Commands

pub struct Pay(u32);
pub struct GainFood(u32);
pub struct RedrawHand;
pub struct Fight;
pub struct SummonUnit(pub Entity, pub Position);
pub struct SpawnUnit(pub Entity, pub Position);
pub struct MoveUnit(pub Entity, pub Position);
pub struct Attack(pub Entity, pub Entity);
pub struct AttackTown(pub Entity);
pub struct Damage(pub Entity, pub u32);
pub struct GainHealth(pub Entity, pub u32);
pub struct Kill(pub Entity);

// Rune

#[derive(Any, Clone, Copy, Debug)]
pub enum RuneCommand {
    #[rune(constructor)]
    None,
    #[rune(constructor)]
    GainFood(#[rune(get)] u32),
    #[rune(constructor)]
    Damage(#[rune(get)] Ent, #[rune(get)] u32),
    #[rune(constructor)]
    GainHealth(#[rune(get)] Ent, #[rune(get)] u32),
}
impl RuneCommand {
    pub fn send(&self, cx: &mut SchedulerContext) {
        match self {
            Self::None => (),
            Self::GainFood(v) => cx.send(GainFood(*v)),
            Self::Damage(e, v) => cx.send(Damage(e.into(), *v)),
            Self::GainHealth(e, v) => cx.send(GainHealth(e.into(), *v)),
        }
    }
}

// Register

pub(crate) fn register_handlers(scheduler: &mut Scheduler<World>) {
    scheduler.add_system(pay);
    scheduler.add_system(gain_food);
    scheduler.add_system(redraw_hand);
    scheduler.add_system(fight);
    scheduler.add_system_with_priority(handle_on_fight, 1);
    scheduler.add_system(summon_unit);
    scheduler.add_system(spawn_unit);
    scheduler.add_system_with_priority(handle_on_spawn, 1);
    scheduler.add_system(move_unit);
    scheduler.add_system(attack);
    scheduler.add_system(attack_town);
    scheduler.add_system(damage);
    scheduler.add_system(gain_health);
    scheduler.add_system(kill);
}

// Handlers

fn pay(cmd: &mut Pay, world: &mut World) -> Result<(), CommandError> {
    world.0.resources.player_data.food = world.0.resources.player_data.food.saturating_sub(cmd.0);
    Ok(())
}

fn gain_food(cmd: &mut GainFood, world: &mut World) -> Result<(), CommandError> {
    world.0.resources.player_data.food += cmd.0;
    Ok(())
}

fn redraw_hand(
    _: &mut RedrawHand,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    let cost = 1;
    if world.0.resources.player_data.food < cost {
        return Err(CommandError::Break);
    }

    crate::player::draw_hand(world);

    cx.send(Pay(cost));
    Ok(())
}

fn fight(_: &mut Fight, world: &mut World) -> Result<(), CommandError> {
    world.0.resources.battle_state.mode = BattleMode::Fight;
    Ok(())
}

fn handle_on_fight(
    _: &mut Fight,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    let on_fight = query_iter!(world.0, With(position, on_fight))
        .map(|(e, _, s)| (e, s.to_string()))
        .collect::<Vec<_>>();

    for (entity, script) in on_fight {
        if let Some(commands) = run_command_script(&script, entity.into(), world) {
            for c in commands {
                c.send(cx);
            }
        }
    }
    Ok(())
}

fn summon_unit(
    cmd: &mut SummonUnit,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    if get_entity_at(world, cmd.1).is_some() {
        return Err(CommandError::Break);
    }
    let data = &mut world.0.resources.player_data;
    if !data.hand.contains(&cmd.0) {
        return Err(CommandError::Break);
    }
    let &cost = world
        .0
        .components
        .cost
        .get(cmd.0)
        .ok_or(CommandError::Break)?;
    if cost > data.food {
        return Err(CommandError::Break);
    }

    data.hand.retain(|a| *a != cmd.0);

    cx.send(SpawnUnit(cmd.0, cmd.1));
    cx.send(Pay(cost));

    Ok(())
}

fn spawn_unit(cmd: &mut SpawnUnit, world: &mut World) -> Result<(), CommandError> {
    if get_entity_at(world, cmd.1).is_some() {
        return Err(CommandError::Break);
    }
    world.0.components.position.insert(cmd.0, cmd.1);
    Ok(())
}

fn handle_on_spawn(
    cmd: &mut SpawnUnit,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    let on_spawn = world
        .0
        .components
        .on_spawn
        .get(cmd.0)
        .ok_or(CommandError::Continue)?
        .clone();

    if let Some(commands) = run_command_script(&on_spawn, cmd.0.into(), world) {
        for c in commands {
            c.send(cx);
        }
    }

    Ok(())
}

fn move_unit(
    cmd: &mut MoveUnit,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    // let cost = 1;
    // if world.0.resources.player_data.food < cost {
    //     return Err(CommandError::Break);
    // }
    if get_entity_at(world, cmd.1).is_some() {
        return Err(CommandError::Break);
    }
    world.0.components.position.insert(cmd.0, cmd.1);
    // cx.send(Pay(cost));
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

fn gain_health(cmd: &mut GainHealth, world: &mut World) -> Result<(), CommandError> {
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
    if world.0.components.player.get(cmd.0).is_some() {
        world.0.components.position.remove(cmd.0);
        if let Some(health) = world.0.components.health.get_mut(cmd.0) {
            health.restore();
        }
        world.0.resources.player_data.discard.push(cmd.0);
    } else {
        world.0.despawn(cmd.0);
    }
    Ok(())
}
