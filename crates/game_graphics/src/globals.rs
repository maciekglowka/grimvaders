use rogalik::prelude::*;

pub const TILE_SIZE: f32 = 32.;
pub const SPRITE_SIZE: f32 = 32.;
pub const SPRITE_OFFSET: Vector2f = Vector2f {
    x: 0.,
    y: 0.5 * TILE_SIZE,
};

pub const BUTTON_SIZE: f32 = 0.5 * SPRITE_SIZE + GAP;
pub const BUTTON_CLICK_SHIFT: f32 = 2.;

pub const BUBBLE_Z: i32 = 150;
pub const OVERLAY_Z: i32 = 100;
pub const UI_Z: i32 = 200;
pub const TILE_Z: i32 = 0;
pub const BACKGROUND_Z: i32 = -1000;

pub const BASE_TEXT_SIZE: f32 = 9.0;
pub const TEXT_LINE_GAP: f32 = 0.1;
pub const DIGITS_TEXT_SIZE: f32 = 6.0;
pub const ICON_SIZE: f32 = 6.0;

pub const GAP: f32 = 4.;

pub const BUBBLE_SPEED: f32 = 0.5;
pub const BUBBLE_MAX_AGE: f32 = 3.;
pub const BUBBLE_WAIT_AGE: f32 = 0.25;

pub const MOVE_SPEED: f32 = 6. * TILE_SIZE;
pub const MOVE_THRESH: f32 = 0.1;
pub const DISINTEGRATE_SPEED: f32 = 2.;

// PALETTE
// pub const BACKGROUND_COLOR: Color = Color(54, 54, 70, 255);
// pub const BACKGROUND_COLOR: Color = Color(56, 112, 127, 255);
pub const BACKGROUND_COLOR: Color = Color(128, 121, 120, 255);
pub const BUTTON_TEXT_COLOR: Color = Color(66, 53, 83, 255);
pub const FOOD_COLOR: Color = Color(229, 189, 126, 255);
pub const RED_COLOR: Color = Color(193, 115, 109, 255);
