use rogalik::prelude::*;

mod assets;
mod input;
mod scenes;

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
        self.main_camera = context.graphics.create_camera(1., get_camera_center());
        context
            .graphics
            .set_clear_color(game_graphics::globals::BACKGROUND_COLOR);
    }
    fn resize(&mut self, context: &mut rogalik::engine::Context) {
        let (w, h) = get_target_resolution(context);
        context.graphics.set_rendering_resolution(w, h);
    }
    fn reload_assets(&mut self, context: &mut rogalik::engine::Context) {
        if !assets::load_data(
            &self.data_assets,
            &mut self.env.world.0.resources.data,
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
        .with_logical_size(800., 600.)
        .build(GameState::default(), Box::new(scenes::MainMenu));
    engine.run();
}

fn get_camera_center() -> Vector2f {
    0.5 * Vector2f::new(
        game_graphics::globals::TILE_SIZE * game_logic::globals::BOARD_W as f32,
        game_graphics::globals::TILE_SIZE * game_logic::globals::BOARD_H as f32,
    )
}

fn get_target_resolution(context: &Context) -> (u32, u32) {
    let size = context.get_physical_size();
    let target_dim = game_graphics::globals::TILE_SIZE * (2. * game_logic::globals::BOARD_H as f32);
    let min_dim = size.y.min(size.x);
    let scale = (min_dim / target_dim).floor();
    // only even resolutions
    (
        (size.x / scale).floor() as u32 / 2 * 2,
        (size.y / scale).floor() as u32 / 2 * 2,
    )
}
