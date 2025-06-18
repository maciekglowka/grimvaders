use anyhow::Result;
use rune::Any;
use wunderkammer::prelude::*;

use crate::{
    battle::player::{remove_player_from_board, reset_player},
    components::{Position, Tag},
    scripting::run_command_script,
    utils::get_unit_at,
    world::{Ent, World},
};

// Commands

pub struct ChangeFood(pub i32, pub Option<Entity>);
pub struct RedrawHand;
pub struct SummonPlayer(pub Entity, pub Position);
pub struct SpawnUnit(pub Entity, pub Position);
pub struct MoveUnit(pub Entity, pub Position);
pub struct Attack(pub Entity, pub Entity);
pub struct AttackTown(pub Entity);
pub struct ChangeHealth(pub Entity, pub i32);
pub struct Kill(pub Entity);
pub struct RemoveUnit(pub Entity);
pub struct RespawnPlayer(pub Entity, pub Position);

// Rune

#[derive(Any, Clone, Copy, Debug)]
pub enum RuneCommand {
    #[rune(constructor)]
    None,
    #[rune(constructor)]
    SpawnUnit(#[rune(get)] Ent, #[rune(get)] Position),
    #[rune(constructor)]
    ChangeFood(#[rune(get)] i32, #[rune(get)] Option<Ent>),
    #[rune(constructor)]
    ChangeHealth(#[rune(get)] Ent, #[rune(get)] i32),
    #[rune(constructor)]
    Attack(#[rune(get)] Ent, #[rune(get)] Ent),
    #[rune(constructor)]
    Kill(#[rune(get)] Ent),
    #[rune(constructor)]
    RemoveUnit(#[rune(get)] Ent),
    #[rune(constructor)]
    RespawnPlayer(#[rune(get)] Ent, #[rune(get)] Position),
}
macro_rules! rune_send {
    { $( ($match_pat:pat => $cmd:expr) ),* } => {
        pub fn send(&self, cx: &mut SchedulerContext) {
            match self {
                Self::None => (),
                $(
                    $match_pat => cx.send($cmd),
                )*
            }
        }
        pub fn scheduler_send(&self, scheduler: &mut Scheduler<World>) {
            match self {
                Self::None => (),
                $(
                    $match_pat => scheduler.send($cmd),
                )*
            }
        }
    }
}
impl RuneCommand {
    rune_send! {
            (Self::SpawnUnit(e, p) => SpawnUnit(e.into(), *p)),
            (Self::ChangeFood(v, e) => ChangeFood(*v, e.map(|a| a.into()))),
            (Self::ChangeHealth(e, v) => ChangeHealth(e.into(), *v)),
            (Self::Attack(e, t) => Attack(e.into(), t.into())),
            (Self::Kill(e) => Kill(e.into())),
            (Self::RemoveUnit(e) => RemoveUnit(e.into())),
            (Self::RespawnPlayer(e, p) => RespawnPlayer(e.into(), *p))
    }
}

// Register

pub(crate) fn register_handlers(scheduler: &mut Scheduler<World>) {
    scheduler.add_system(change_food);
    scheduler.add_system_with_priority(handle_on_ally_gain_food, 1);
    scheduler.add_system(redraw_hand);
    scheduler.add_system(summon_player);
    scheduler.add_system(spawn_unit);
    scheduler.add_system_with_priority(handle_on_spawn, 1);
    scheduler.add_system(move_unit);
    scheduler.add_system(attack);
    scheduler.add_system_with_priority(handle_on_attack, 1);
    scheduler.add_system(attack_town);
    scheduler.add_system(change_health);
    scheduler.add_system_with_priority(handle_on_damage, 1);
    scheduler.add_system_with_priority(handle_on_ally_heal, 2);
    scheduler.add_system(kill);
    scheduler.add_system_with_priority(handle_on_kill, 1);
    scheduler.add_system_with_priority(handle_on_ally_kill, 2);
    scheduler.add_system(remove_unit);
    scheduler.add_system(respawn_player);
}

// Macros

macro_rules! handle_on_ally {
    ($world:ident, $cx:ident, $handler:ident, $target_entity:expr, $command:expr) => {{
        if $world.components.player.get($target_entity).is_some() {
            let hosts = query_iter!($world, With(player, position, $handler))
                // Do not trigger on self
                .filter(|(e, _, _, _)| *e != $target_entity)
                .map(|(e, _, _, s)| (e, s.to_string()))
                .collect::<Vec<_>>();

            for (entity, script) in hosts {
                if check_trigger_limit(entity, $world).is_err() {
                    continue;
                }
                if let Some(commands) = run_command_script(&script, entity.into(), $world, $command)
                {
                    if !commands.is_empty() {
                        use_trigger_limit(entity, $world);
                    }
                    for c in commands {
                        c.send($cx);
                    }
                }
            }
        }
    }};
}

macro_rules! handle_on_self {
    ($world:ident, $cx:ident, $handler:ident, $entity:expr, $command:expr) => {{
        check_trigger_limit($entity, $world)?;

        let component = $world
            .components
            .$handler
            .get($entity)
            .ok_or(CommandError::Continue)?
            .clone();

        if let Some(commands) = run_command_script(&component, $entity.into(), $world, $command) {
            if !commands.is_empty() {
                use_trigger_limit($entity, $world);
            }
            for c in commands {
                c.send($cx);
            }
        }
    }};
}

// Handlers

fn change_food(cmd: &mut ChangeFood, world: &mut World) -> Result<(), CommandError> {
    if cmd.0 < 0 {
        world.resources.player_data.food = world
            .resources
            .player_data
            .food
            .saturating_sub((-cmd.0) as u32);
    } else {
        world.resources.player_data.food += cmd.0 as u32;
    }
    Ok(())
}

fn handle_on_ally_gain_food(
    cmd: &mut ChangeFood,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    // Handle only on gain
    if cmd.0 <= 0 {
        return Ok(());
    };
    let Some(entity) = cmd.1 else {
        return Ok(());
    };

    handle_on_ally!(
        world,
        cx,
        on_ally_gain_food,
        entity,
        RuneCommand::ChangeFood(cmd.0, cmd.1.map(|a| a.into()))
    );

    Ok(())
}

fn redraw_hand(
    _: &mut RedrawHand,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    let cost = 1;
    if world.resources.player_data.food < cost {
        return Err(CommandError::Break);
    }

    crate::player::draw_hand(world);

    cx.send(ChangeFood(-(cost as i32), None));
    Ok(())
}

fn summon_player(
    cmd: &mut SummonPlayer,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    if get_unit_at(world, cmd.1).is_some() {
        return Err(CommandError::Break);
    }
    let &cost = world
        .components
        .cost
        .get(cmd.0)
        .ok_or(CommandError::Break)?;

    let data = &mut world.resources.player_data;

    if !data.hand.contains(&cmd.0) {
        return Err(CommandError::Break);
    }
    if cost > data.food {
        return Err(CommandError::Break);
    }

    data.hand.retain(|a| *a != cmd.0);

    cx.send(SpawnUnit(cmd.0, cmd.1));
    cx.send(ChangeFood(-(cost as i32), None));

    Ok(())
}

fn spawn_unit(cmd: &mut SpawnUnit, world: &mut World) -> Result<(), CommandError> {
    if let Some(existing) = get_unit_at(world, cmd.1) {
        if world.components.killed.get(existing).is_none() && existing != cmd.0 {
            return Err(CommandError::Break);
        }
    }
    world.components.position.insert(cmd.0, cmd.1);
    Ok(())
}

fn handle_on_spawn(
    cmd: &mut SpawnUnit,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    handle_on_self!(
        world,
        cx,
        on_spawn,
        cmd.0,
        RuneCommand::SpawnUnit(cmd.0.into(), cmd.1)
    );
    Ok(())
}

fn move_unit(
    cmd: &mut MoveUnit,
    world: &mut World,
    _: &mut SchedulerContext,
) -> Result<(), CommandError> {
    if let Some(tags) = world.components.tags.get(cmd.0) {
        if tags.contains(&Tag::Heavy) {
            return Err(CommandError::Break);
        }
    }
    if get_unit_at(world, cmd.1).is_some() {
        return Err(CommandError::Break);
    }
    world.components.position.insert(cmd.0, cmd.1);
    Ok(())
}

fn attack(
    cmd: &mut Attack,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    let health_0 = world
        .components
        .health
        .get(cmd.0)
        .ok_or(CommandError::Break)?;
    let health_1 = world
        .components
        .health
        .get(cmd.1)
        .ok_or(CommandError::Break)?;

    cx.send(ChangeHealth(cmd.1, -(health_0.current() as i32)));
    cx.send(ChangeHealth(cmd.0, -(health_1.current() as i32)));
    Ok(())
}

fn handle_on_attack(
    cmd: &mut Attack,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    handle_on_self!(
        world,
        cx,
        on_attack,
        cmd.0,
        RuneCommand::Attack(cmd.0.into(), cmd.1.into())
    );

    Ok(())
}

fn attack_town(
    cmd: &mut AttackTown,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    let health = world
        .components
        .health
        .get(cmd.0)
        .ok_or(CommandError::Break)?;
    world.resources.player_data.health = world
        .resources
        .player_data
        .health
        .saturating_sub(health.current());

    // npc is removed
    cx.send(Kill(cmd.0));
    Ok(())
}

fn change_health(
    cmd: &mut ChangeHealth,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    let health = world
        .components
        .health
        .get_mut(cmd.0)
        .ok_or(CommandError::Break)?;

    if cmd.1 < 0 {
        health.sub((-cmd.1) as u32);
        if health.current() == 0 {
            cx.send(Kill(cmd.0));
        }
    } else {
        health.add(cmd.1 as u32);
    }
    Ok(())
}

fn handle_on_damage(
    cmd: &mut ChangeHealth,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    // Handle only damage
    if cmd.1 >= 0 {
        return Ok(());
    }

    // Only trigger when the unit is still alive
    if world
        .components
        .health
        .get(cmd.0)
        .ok_or(CommandError::Break)?
        .current()
        == 0
    {
        return Ok(());
    }

    handle_on_self!(
        world,
        cx,
        on_damage,
        cmd.0,
        RuneCommand::ChangeHealth(cmd.0.into(), cmd.1)
    );

    Ok(())
}

fn handle_on_ally_heal(
    cmd: &mut ChangeHealth,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    // Handle only heal
    if cmd.1 <= 0 {
        return Ok(());
    }

    handle_on_ally!(
        world,
        cx,
        on_ally_heal,
        cmd.0,
        RuneCommand::ChangeHealth(cmd.0.into(), cmd.1)
    );

    Ok(())
}

fn kill(cmd: &mut Kill, world: &mut World) -> Result<(), CommandError> {
    world.components.killed.insert(cmd.0, ());
    Ok(())
}

fn handle_on_kill(
    cmd: &mut Kill,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    handle_on_self!(world, cx, on_kill, cmd.0, RuneCommand::Kill(cmd.0.into()));

    Ok(())
}

fn handle_on_ally_kill(
    cmd: &mut Kill,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    handle_on_ally!(
        world,
        cx,
        on_ally_kill,
        cmd.0,
        RuneCommand::Kill(cmd.0.into())
    );

    Ok(())
}

fn remove_unit(cmd: &mut RemoveUnit, world: &mut World) -> Result<(), CommandError> {
    if world.components.player.get(cmd.0).is_some() {
        remove_player_from_board(cmd.0, world);
    } else {
        world.despawn(cmd.0);
    }
    Ok(())
}

fn respawn_player(
    cmd: &mut RespawnPlayer,
    world: &mut World,
    cx: &mut SchedulerContext,
) -> Result<(), CommandError> {
    // Only respawn killed units
    if world.components.killed.get(cmd.0).is_none() {
        return Err(CommandError::Break);
    }
    reset_player(cmd.0, world);
    cx.send(SpawnUnit(cmd.0, cmd.1));
    Ok(())
}

// Utils

pub(crate) fn check_trigger_limit(entity: Entity, world: &World) -> Result<(), CommandError> {
    let Some(limit) = world.components.trigger_limit.get(entity) else {
        return Ok(());
    };
    if limit.current() > 0 {
        Ok(())
    } else {
        Err(CommandError::Continue)
    }
}

pub(crate) fn use_trigger_limit(entity: Entity, world: &mut World) {
    if let Some(limit) = world.components.trigger_limit.get_mut(entity) {
        limit.sub(1);
    }
}
