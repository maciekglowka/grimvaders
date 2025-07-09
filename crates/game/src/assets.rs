use rogalik::prelude::*;
use std::collections::HashMap;

use game_data::GameData;

const DATA_FILES: [&str; 2] = ["player", "npcs"];
const SPRITE_DATA: &str = include_str!("../../../assets/sprites/units.json");

#[derive(Default)]
pub struct DataAssets {
    pub files: HashMap<String, ResourceId>,
}

pub fn load_assets(context: &mut Context) {
    load_audio(context);
    load_graphics(context);
}

fn load_audio(context: &mut Context) {
    let _ = context.audio.load_source("click", "sfx/click.wav");
    let _ = context.audio.load_source("heal", "sfx/heal.wav");
    let _ = context.audio.load_source("hit", "sfx/hit.wav");
    let _ = context.audio.load_source("jump", "sfx/jump.wav");
    let _ = context.audio.load_source("yield", "sfx/yield.wav");
    let _ = context.audio.load_source("spawn", "sfx/spawn.wav");
}

fn load_graphics(context: &mut Context) {
    let outline_shader = context
        .graphics
        .load_shader(ShaderKind::Sprite, "shaders/outline.wgsl");
    let disintegrate_shader = context
        .graphics
        .load_shader(ShaderKind::Sprite, "shaders/disintegrate.wgsl");

    // Units sprites

    let units_texture = Some(context.graphics.load_texture("sprites/units.png"));
    let units_data = game_data::sprites::load_sprite_sheet_data(SPRITE_DATA);

    let units_atlas = Some(AtlasParams {
        cols: units_data.meta.size.w / game_graphics::globals::SPRITE_SIZE as usize,
        rows: units_data.meta.size.h / game_graphics::globals::SPRITE_SIZE as usize,
        padding: None,
    });

    context.graphics.load_material(
        "units",
        MaterialParams {
            atlas: units_atlas,
            diffuse_texture: units_texture,
            shader: Some(outline_shader),
            ..Default::default()
        },
    );

    context.graphics.load_material(
        "disintegrate",
        MaterialParams {
            atlas: units_atlas,
            diffuse_texture: units_texture,
            shader: Some(disintegrate_shader),
            ..Default::default()
        },
    );

    // Other sprites

    let sprites_texture = Some(context.graphics.load_texture("sprites/sprites.png"));
    context.graphics.load_material(
        "sprites",
        MaterialParams {
            atlas: Some(AtlasParams {
                cols: 49,
                rows: 22,
                padding: Some((1., 1.)),
            }),
            diffuse_texture: sprites_texture,
            ..Default::default()
        },
    );

    let bg_texture = Some(context.graphics.load_texture("sprites/background.png"));
    context.graphics.load_material(
        "background",
        MaterialParams {
            diffuse_texture: bg_texture,
            repeat: TextureRepeat::Repeat,
            ..Default::default()
        },
    );

    let main_title = Some(context.graphics.load_texture("sprites/main.png"));
    context.graphics.load_material(
        "main_title",
        MaterialParams {
            diffuse_texture: main_title,
            ..Default::default()
        },
    );

    // Tiles

    let tiles_texture = Some(context.graphics.load_texture("sprites/tiles.png"));
    context.graphics.load_material(
        "tiles",
        MaterialParams {
            atlas: Some(AtlasParams {
                cols: 12,
                rows: 1,
                padding: None,
            }),
            diffuse_texture: tiles_texture,
            ..Default::default()
        },
    );

    // Ui

    let ui_texture = Some(context.graphics.load_texture("ui/ui.png"));
    context.graphics.load_material(
        "ui",
        MaterialParams {
            atlas: Some(AtlasParams {
                cols: 7,
                rows: 1,
                padding: None,
            }),
            diffuse_texture: ui_texture,
            ..Default::default()
        },
    );

    let icons_texture = Some(context.graphics.load_texture("ui/icons_small.png"));
    context.graphics.load_material(
        "icons_small",
        MaterialParams {
            atlas: Some(AtlasParams {
                cols: 5,
                rows: 1,
                padding: None,
            }),
            diffuse_texture: icons_texture,
            ..Default::default()
        },
    );

    let outline_text = context
        .graphics
        .load_shader(ShaderKind::Sprite, "shaders/outline_text.wgsl");

    context
        .graphics
        .load_font("default", "ui/font.png", 16, 16, Some((11., 7.)), None);

    context
        .graphics
        .load_font("digits", "ui/digits.png", 16, 16, Some((4., 2.)), None);

    context.graphics.load_font(
        "digits_outline",
        "ui/digits.png",
        16,
        16,
        Some((4., 2.)),
        Some(outline_text),
    );

    // Shaders

    let noise_shader = context
        .graphics
        .load_shader(ShaderKind::PostProcess, "shaders/noise.wgsl");
    let noise_texture = Some(context.graphics.load_texture("shaders/perlin.png"));

    context.graphics.add_post_process(
        "noise",
        PostProcessParams {
            shader: noise_shader,
            texture: noise_texture,
            ..Default::default()
        },
    );

    let post_shader = context
        .graphics
        .load_shader(ShaderKind::PostProcess, "shaders/post.wgsl");

    #[cfg(not(target_arch = "wasm32"))]
    context.graphics.add_post_process(
        "post",
        PostProcessParams {
            shader: post_shader,
            ..Default::default()
        },
    );

    let glow_shader = context
        .graphics
        .load_shader(ShaderKind::PostProcess, "shaders/glow.wgsl");

    context.graphics.add_post_process(
        "glow",
        PostProcessParams {
            shader: glow_shader,
            ..Default::default()
        },
    );
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

    if updated {
        let units_data = game_data::sprites::load_sprite_sheet_data(SPRITE_DATA);
        game_data::sprites::update_sprite_data(data, &units_data);
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
