use rune::{Any, Module};
use std::collections::*;
use wunderkammer::prelude::*;

use crate::{components::*, player::PlayerData};
use macros::RuneAdapter;

#[derive(Default)]
pub struct GameEnv {
    pub world: World,
    pub scheduler: Scheduler<World>,
    pub input: Option<Observer<crate::InputEvent>>,
}

#[derive(Any, Default)]
pub struct World(pub WorldStorage<Components, Resources>);
impl World {
    #[rune::function]
    fn query(&self, with: Vec<String>, without: Vec<String>) -> Vec<Ent> {
        //
        if with.is_empty() {
            return Vec::new();
        }
        let mut entities = self.0.components.entities_str(&with[0]);
        for component in with.iter().skip(1) {
            entities = entities
                .intersection(&self.0.components.entities_str(component))
                .copied()
                .collect();
        }
        for component in without {
            entities = entities
                .difference(&self.0.components.entities_str(&component))
                .copied()
                .collect();
        }
        entities.iter().map(|&e| e.into()).collect()
    }
    #[rune::function]
    fn get_tile_at(&self, position: &Position) -> Option<Tile> {
        let entity = crate::get_tile_at(self, *position)?;
        self.0.components.tile.get(entity).copied()
    }
    #[rune::function]
    fn get_entity_at(&self, position: &Position) -> Option<Ent> {
        Some(crate::get_entity_at(self, *position)?.into())
    }
}

#[derive(Any, Default, ComponentSet, RuneAdapter)]
pub struct Components {
    pub cost: ComponentStorage<u32>,
    pub health: ComponentStorage<ValueDefault>,
    pub name: ComponentStorage<String>,
    pub npc: ComponentStorage<()>,
    // handlers start
    pub on_spawn: ComponentStorage<String>,
    pub on_fight: ComponentStorage<String>,
    pub on_kill: ComponentStorage<String>,
    // handlers end
    pub player: ComponentStorage<()>,
    pub position: ComponentStorage<Position>,
    pub tile: ComponentStorage<Tile>,
}

#[derive(Default)]
pub struct Resources {
    pub battle_state: crate::battle::BattleState,
    pub data: game_data::GameData,
    pub player_data: PlayerData,
    // serialize as none
    pub(crate) vm: Option<rune::Vm>,
}

#[derive(Any, Clone, Copy, Debug)]
pub struct Ent(u16, u16);
impl From<Entity> for Ent {
    fn from(value: Entity) -> Self {
        Self(value.id, value.version)
    }
}
impl From<&Entity> for Ent {
    fn from(value: &Entity) -> Self {
        Self(value.id, value.version)
    }
}
impl From<Ent> for Entity {
    fn from(value: Ent) -> Self {
        Entity {
            id: value.0,
            version: value.1,
        }
    }
}
impl From<&Ent> for Entity {
    fn from(value: &Ent) -> Self {
        Entity {
            id: value.0,
            version: value.1,
        }
    }
}
