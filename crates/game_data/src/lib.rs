use serde::Deserialize;
use std::collections::HashMap;

pub mod sprites;

#[derive(Clone, Default)]
pub struct GameData {
    pub entities: HashMap<String, EntityData>,
    pub categories: HashMap<String, Vec<String>>,
}
impl GameData {
    pub fn add_entities(&mut self, s: &str, category: &str) {
        let map: HashMap<String, EntityData> =
            serde_yaml::from_str(s).expect("Can't parse yaml data!");
        let inserted = map.keys().map(|s| s.to_string()).collect();
        self.entities.extend(map);
        self.categories.insert(category.to_string(), inserted);
    }
}

#[derive(Clone, Deserialize)]
pub struct EntityData {
    pub components: HashMap<String, serde_yaml::Value>,
    pub sprite: SpriteData,
    pub chance: Option<f32>,
    pub tier: Option<u32>,
    pub score: Option<u32>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Clone, Deserialize, Default)]
pub struct SpriteData {
    pub atlas: String,
    #[serde(default)]
    pub index: usize,
    pub frames: Option<usize>,
    pub tag: Option<String>,
}
