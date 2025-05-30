use rogalik::prelude::*;

use game_graphics::utils::tile_to_world;
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

#[allow(dead_code)]
fn main() {
    env_logger::init();
    let engine = EngineBuilder::new()
        .with_title("RGLK".to_string())
        .resizable(true)
        .with_logical_size(1280., 720.)
        .build(GameState::default(), Box::new(scenes::MainMenu));
    engine.run();
}

fn get_camera_center(vw: u32, vh: u32) -> Vector2f {
    let bottom_left = tile_to_world(Position::new(0, 0));
    let bottom_right = tile_to_world(Position::new(BOARD_W as i32, 0));
    let upper_left = tile_to_world(Position::new(0, TOTAL_BOARD_H as i32));
    let upper_right = tile_to_world(Position::new(BOARD_W as i32, TOTAL_BOARD_H as i32));

    // let mut board_inner_size = (game_graphics::utils::tile_to_world(
    //     game_logic::components::Position::new(game_logic::globals::BOARD_W as
    // i32, 0), ) - game_graphics::utils::tile_to_world(
    //     game_logic::components::Position::new(0, TOTAL_BOARD_H as i32 + 1),
    // ));

    let board_inner_size =
        Vector2f::new(bottom_right.x - upper_left.x, upper_right.y - bottom_left.y);

    // 0.5 * game_graphics::utils::tile_to_world(game_logic::components::Position::new(
    //     game_logic::globals::BOARD_W as i32,
    //     TOTAL_BOARD_H as i32 - 3,
    // ))
    Vector2f::new(
        0.5 * (vw as f32 - board_inner_size.x),
        0.5 * board_inner_size.y,
    )
}

fn get_target_resolution(context: &Context) -> (u32, u32) {
    let size = context.get_physical_size();
    let target_dim = game_graphics::globals::TILE_SIZE * TOTAL_BOARD_H as f32 / 1.5;
    let min_dim = size.y.min(size.x);
    let scale = (min_dim / target_dim).floor();
    // only even resolutions
    (
        (size.x / scale).floor() as u32 / 2 * 2,
        (size.y / scale).floor() as u32 / 2 * 2,
    )
}
