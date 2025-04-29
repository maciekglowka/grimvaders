use rune::Any;
use serde::Deserialize;
use wunderkammer::prelude::*;

// #[derive(Any, Clone, Deserialize)]
// pub enum ActionKind {
//     ModifyHealth(i32),
//     // ModifyMana(ModifyMana),
//     // ModifyShield(ModifyShield),
//     // Poison(Poison),
// }
// impl ActionKind {
//     pub fn command(&self, entity: Entity) -> CommandKind {
//         match self {
//             Self::ModifyHealth(v) if v < &0 => CommandKind::Damage(entity,
// (-v) as u32),             Self::ModifyHealth(v) =>
// CommandKind::Heal(entity.into(), *v as u32),         }
//     }
// }
