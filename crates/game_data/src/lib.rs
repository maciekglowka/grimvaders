use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct GameData {
    pub entities: HashMap<String, EntityData>,
    pub players: Vec<String>,
    pub npcs: Vec<String>,
}
impl GameData {
    pub fn add_entities(&mut self, s: &str) -> Vec<String> {
        let map: HashMap<String, EntityData> =
            serde_yaml::from_str(s).expect("Can't parse yaml data!");
        let inserted = map.keys().map(|s| s.to_string()).collect();
        self.entities.extend(map);
        inserted
    }
}

#[derive(Clone, Deserialize)]
pub struct EntityData {
    pub components: HashMap<String, serde_yaml::Value>,
    pub sprite: SpriteData,
    pub min_level: Option<u32>,
    pub max_level: Option<u32>,
    #[serde(default = "one")]
    pub chance: f32,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

fn one() -> f32 {
    1.
}

#[derive(Clone, Deserialize, Default)]
pub struct SpriteData {
    pub atlas: String,
    pub index: usize,
}
