use rogalik::prelude::*;
use std::collections::HashMap;

use game_data::GameData;

const DATA_FILES: [&str; 2] = ["player", "npcs"];

#[derive(Default)]
pub struct DataAssets {
    pub files: HashMap<String, ResourceId>,
}

pub fn load_assets(context: &mut Context) {
    load_textures(context);
}

fn load_textures(context: &mut Context) {
    context.graphics.load_material(
        "sprites",
        MaterialParams {
            atlas: Some(AtlasParams {
                cols: 49,
                rows: 22,
                padding: Some((1., 1.)),
            }),
            diffuse_path: "sprites/sprites.png",
            ..Default::default()
        },
    );
    context.graphics.load_material(
        "icons_small",
        MaterialParams {
            atlas: Some(AtlasParams {
                cols: 8,
                rows: 8,
                padding: None,
            }),
            diffuse_path: "ui/icons_small.png",
            ..Default::default()
        },
    );
    context
        .graphics
        .load_font("default", "ui/font.png", 16, 16, Some((11., 8.)));

    context
        .graphics
        .load_font("digits", "ui/digits.png", 16, 16, Some((4., 2.)));
}

pub fn load_data_assets(context: &mut Context) -> DataAssets {
    let mut assets = DataAssets::default();
    let mut store = context.assets.lock().unwrap();

    for name in DATA_FILES {
        let path = format!("data/{}.yaml", name);
        let resource_id = store.load(&path).expect(&path);
        assets.files.insert(name.to_string(), resource_id);
    }
    assets
}

pub fn load_data(
    assets: &DataAssets,
    data: &mut GameData,
    context: &mut Context,
    reload: bool,
) -> bool {
    let Ok(mut store) = context.assets.lock() else {
        return false;
    };
    let mut updated = false;

    for (k, v) in assets.files.iter() {
        let Some(asset) = store.get(*v) else {
            continue;
        };
        if reload && asset.state != AssetState::Updated {
            continue;
        }
        updated = true;
        load_data_item(k, asset.data.get(), data);
        store.mark_read(*v);
    }
    updated
}

pub fn load_data_item(name: &str, data: &[u8], game_data: &mut GameData) {
    let Ok(s) = String::from_utf8(data.to_vec()) else {
        log::error!("Can't parse {} as string!", name);
        return;
    };
    game_data.add_entities(&s, name);
}
