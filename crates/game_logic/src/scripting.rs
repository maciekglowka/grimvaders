use anyhow::Result;
use rune::{Diagnostics, Module, ToValue, Value, Vm};
use std::sync::Arc;

use crate::{
    commands::RuneCommand,
    components::{Position, Tag, Tile, ValueDefault},
    world::{Ent, World},
};

pub fn init_rune(world: &World) -> Result<Vm> {
    let mut context = rune_modules::default_context().unwrap();

    let mut command_module = Module::new();
    command_module.ty::<RuneCommand>()?;
    command_module.ty::<Position>()?;
    command_module.function_meta(Position::partial_eq__meta)?;
    command_module.ty::<Tile>()?;
    command_module.function_meta(Tile::partial_eq__meta)?;
    command_module.ty::<Ent>()?;
    command_module.function_meta(Ent::clone)?;
    command_module.function_meta(Ent::partial_eq__meta)?;
    command_module.ty::<Tag>()?;
    command_module.ty::<ValueDefault>()?;
    context.install(command_module)?;

    let world_module = World::module()?;
    context.install(world_module)?;

    let mut sources = rune::Sources::new();

    for (name, data) in world.0.resources.data.entities.iter() {
        if let Some(script) = &data.script {
            let entry = rune::Source::new(name, script)?;
            sources.insert(entry)?;
        }
    }

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer =
            rune::termcolor::StandardStream::stderr(rune::termcolor::ColorChoice::default());
        diagnostics.emit(&mut writer, &sources).unwrap();
    }

    let unit = result.unwrap();
    let vm = Vm::new(Arc::new(context.runtime()?), Arc::new(unit));
    log::debug!("Rune VM created successfully");

    Ok(vm)
}

pub(crate) fn run_command_script(
    script: &str,
    entity: Ent,
    world: &mut World,
    command: RuneCommand,
) -> Option<Vec<RuneCommand>> {
    log::debug!("Running script: {}", script);
    let mut vm = world.0.resources.vm.take().unwrap();

    let result = match vm.call([script], (&*world, entity, command)) {
        // Do not early exit here - it will result in a missing Vm
        Ok(output) => {
            if let Ok(value) = rune::from_value(&output) {
                // If already a vec
                Some(value)
            } else {
                // Otherwise wrap in a vec
                rune::from_value(&output).map(|v| vec![v]).ok()
            }
        }
        Err(e) => {
            log::error!("Script {} failed: {}", script, e);
            None
        }
    };
    log::debug!("{} result: {:?}", script, result);
    world.0.resources.vm = Some(vm);
    result
}
