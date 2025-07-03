use rogalik::prelude::*;

mod battle;
mod deck;
mod game;
mod game_over;
mod main_menu;
mod shop;
mod win;

pub(crate) use main_menu::MainMenu;

pub(crate) fn draw_centered_text(text: &str, context: &mut Context) {
    let bounds = game_graphics::utils::get_viewport_bounds(context);
    let center = 0.5 * (bounds.0 + bounds.1);

    let size = context.graphics.text_dimensions("default", text, 18.);

    let _ = context.graphics.draw_text(
        "default",
        text,
        center - 0.5 * size,
        0,
        18.,
        SpriteParams::default(),
    );
}
