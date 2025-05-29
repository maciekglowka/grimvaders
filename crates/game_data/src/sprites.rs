use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct SpriteSheetData {
    pub meta: SpriteSheetMeta,
}

#[derive(Clone, Deserialize)]
pub struct SpriteSheetMeta {
    frameTags: Vec<FrameTag>,
    pub size: SheetSize,
}

#[derive(Clone, Deserialize)]
pub struct FrameTag {
    name: String,
    from: usize,
    to: usize,
}

#[derive(Clone, Deserialize)]
pub struct SheetSize {
    pub w: usize,
    pub h: usize,
}

pub fn load_sprite_sheet_data(s: &str) -> SpriteSheetData {
    serde_yaml::from_str(s).expect("Could not parse sprite sheet data!")
}

pub fn update_sprite_data(game_data: &mut super::GameData, sheet_data: &SpriteSheetData) {
    for entity in game_data.entities.values_mut() {
        if let Some(tag) = &entity.sprite.tag {
            if let Some(tag_data) = find_tag_data(tag, sheet_data) {
                entity.sprite.index = tag_data.from;
                let d = tag_data.to - tag_data.from;
                if d > 0 {
                    entity.sprite.frames = Some(tag_data.to - tag_data.from + 1);
                }
            }
        }
    }
}

fn find_tag_data<'a>(tag: &str, sheet_data: &'a SpriteSheetData) -> Option<&'a FrameTag> {
    sheet_data.meta.frameTags.iter().find(|a| a.name == tag)
}
