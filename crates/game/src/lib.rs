use rogalik::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use game_graphics::{
    globals::{SIDE_PANEL_W, TILE_SIZE},
    utils::tile_to_world,
};
use game_logic::{
    components::Position,
    globals::{BOARD_H, BOARD_W, MAX_WAVE_H},
};

mod assets;
mod input;
mod scenes;

const TOTAL_BOARD_H: usize = BOARD_H + MAX_WAVE_H + 1;

#[derive(Default)]
struct GameState {
    data_assets: assets::DataAssets,
    main_camera: ResourceId,
    env: game_logic::GameEnv,
}

impl Game for GameState {
    fn setup(&mut self, context: &mut Context) {
        assets::load_assets(context);
        self.data_assets = assets::load_data_assets(context);
        self.main_camera = context.graphics.create_camera(1., Vector2f::ZERO);

        context
            .graphics
            .set_clear_color(game_graphics::globals::BACKGROUND_COLOR);
    }
    fn resize(&mut self, context: &mut rogalik::engine::Context) {
        let (w, h) = get_target_resolution(context);
        context.graphics.set_rendering_resolution(w, h);
        context
            .graphics
            .get_camera_mut(self.main_camera)
            .unwrap()
            .set_target(get_camera_center(w, h));
    }
    fn reload_assets(&mut self, context: &mut rogalik::engine::Context) {
        if !assets::load_data(
            &self.data_assets,
            &mut self.env.world.resources.data,
            context,
            true,
        ) {
            return;
        };
        if let Ok(vm) = game_logic::scripting::init_rune(&self.env.world) {
            self.env.world.0.resources.vm = Some(vm);
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    console_log::init_with_level(rogalik::engine::log::Level::Info)
        .expect("Can't init the logger!");
    let engine = EngineBuilder::new()
        .with_audio(AudioDeviceParams {
            buffer_secs: 0.01,
            ..Default::default()
        })
        .build_wasm(GameState::default(), Box::new(scenes::MainMenu));
    engine.run();
}

#[allow(dead_code)]
fn main() {
    env_logger::init();
    let engine = EngineBuilder::new()
        .with_title("Grimvaders".to_string())
        .with_audio(AudioDeviceParams {
            buffer_secs: 0.01,
            ..Default::default()
        })
        .resizable(true)
        .with_logical_size(1280., 720.)
        .build(GameState::default(), Box::new(scenes::MainMenu));
    engine.run();
}

fn get_camera_center(_: u32, _: u32) -> Vector2f {
    let board_center = tile_to_world(Position::new(
        BOARD_W as i32 / 2,
        ((BOARD_H + MAX_WAVE_H - 1) / 2) as i32,
    )) + Vector2f::splat(0.5 * TILE_SIZE);

    Vector2f::new(board_center.x + 0.5 * SIDE_PANEL_W, board_center.y)
}

fn get_target_resolution(context: &Context) -> (u32, u32) {
    let size = context.get_physical_size();
    let target_dim = TILE_SIZE * (1. + TOTAL_BOARD_H as f32) / 1.5;
    let min_dim = size.y.min(size.x);
    let scale = (min_dim / target_dim).floor();
    // only even resolutions
    (
        (size.x / scale).floor() as u32 / 2 * 2,
        (size.y / scale).floor() as u32 / 2 * 2,
    )
}
