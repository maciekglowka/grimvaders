use anyhow::Result;
use rune::{Module, Value, Vm};
use std::sync::Arc;

use crate::{
    commands::RuneCommand,
    components::Tile,
    world::{Ent, World},
};

pub(crate) fn init_rune(world: &World) -> Result<Vm> {
    let mut context = rune_modules::default_context().unwrap();

    let mut command_module = Module::new();
    command_module.ty::<RuneCommand>()?;
    command_module.ty::<Tile>()?;
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

    let result = rune::prepare(&mut sources).with_context(&context).build();
    let unit = result.unwrap();
    let vm = Vm::new(Arc::new(context.runtime()?), Arc::new(unit));

    Ok(vm)
}

pub(crate) fn run_command_script(
    script: &str,
    entity: Ent,
    world: &mut World,
) -> Option<Vec<RuneCommand>> {
    log::debug!("Running script: {}", script);
    let mut vm = world.0.resources.vm.take().unwrap();

    let result = match vm.call([script], (&*world, entity)) {
        Ok(output) => match output {
            Value::Vec(_) => rune::from_value(output).ok(),
            _ => Some(vec![rune::from_value(output).ok()?]),
        },
        Err(e) => {
            log::error!("Script {} failed: {}", script, e);
            None
        }
    };
    world.0.resources.vm = Some(vm);
    result
}
