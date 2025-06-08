use rune::{Any, Module};
use wunderkammer::prelude::*;

use crate::{components::*, get_tile_at, get_unit_at, player::PlayerData};
use macros::{ComponentGen, RuneAdapter};

#[derive(Default)]
pub struct GameEnv {
    pub world: World,
    pub scheduler: Scheduler<World>,
    pub input: Option<Observer<crate::InputEvent>>,
}

type WorldInner = WorldStorage<Components, Resources>;

#[derive(Any, Default)]
pub struct World(pub WorldInner);
impl World {
    // Resources
    #[rune::function]
    fn get_current_food(&self) -> u32 {
        self.resources.player_data.food
    }

    // Components
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
        let entity = get_tile_at(self, *position)?;
        self.0.components.tile.get(entity).copied()
    }

    #[rune::function]
    fn get_unit_at(&self, position: &Position) -> Option<Ent> {
        Some(get_unit_at(self, *position)?.into())
    }

    #[rune::function]
    fn get_adjacent_units(&self, entity: &Ent) -> Vec<Ent> {
        let Some(position) = self.0.components.position.get(entity.into()) else {
            return Vec::new();
        };
        ORTHO
            .iter()
            .map(|d| *d + position)
            .filter_map(|p| get_unit_at(self, p))
            .filter(|e| self.components.player.get(*e).is_some())
            .map(|e| e.into())
            .collect()
    }

    #[rune::function]
    fn get_units_with_tag(&self, tag: &Tag) -> Vec<Ent> {
        query_iter!(self.0, With(player, position, tags))
            .filter(|(_, _, _, t)| t.contains(tag))
            .map(|(e, _, _, _)| e.into())
            .collect()
    }

    #[rune::function]
    fn is_in_front(&self, entity: &Ent, other: &Ent) -> bool {
        match (
            self.components.position.get(entity.into()),
            self.components.position.get(other.into()),
        ) {
            (Some(a), Some(b)) => a.x == b.x && b.y - a.y == 1,
            _ => false,
        }
    }

    #[rune::function]
    fn is_adjacent(&self, entity: &Ent, other: &Ent) -> bool {
        match (
            self.components.position.get(entity.into()),
            self.components.position.get(other.into()),
        ) {
            (Some(a), Some(b)) => a.manhattan(b) == 1,
            _ => false,
        }
    }
}
impl std::ops::Deref for World {
    type Target = WorldInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Any, Default, ComponentSet, RuneAdapter, ComponentGen)]
pub struct Components {
    pub cost: ComponentStorage<u32>,
    pub health: ComponentStorage<ValueDefault>,
    // temp marker
    pub killed: ComponentStorage<()>,
    pub name: ComponentStorage<String>,
    pub npc: ComponentStorage<()>,
    // handlers start
    pub on_spawn: ComponentStorage<String>,
    pub on_fight: ComponentStorage<String>,
    pub on_kill: ComponentStorage<String>,
    pub on_ally_kill: ComponentStorage<String>,
    pub on_damage: ComponentStorage<String>,
    pub on_ally_heal: ComponentStorage<String>,
    pub on_ally_gain_food: ComponentStorage<String>,
    // handlers end
    pub player: ComponentStorage<()>,
    pub position: ComponentStorage<Position>,
    pub tags: ComponentStorage<Vec<Tag>>,
    pub tile: ComponentStorage<Tile>,
    pub trigger_limit: ComponentStorage<ValueDefault>,
}

#[derive(Default)]
pub struct Resources {
    pub battle_state: crate::battle::BattleState,
    pub data: game_data::GameData,
    pub player_data: PlayerData,
    // serialize as none
    pub vm: Option<rune::Vm>,
}

#[derive(Any, Clone, Copy, Debug)]
pub struct Ent(u16, u16);
impl Ent {
    #[rune::function]
    pub fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}
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
