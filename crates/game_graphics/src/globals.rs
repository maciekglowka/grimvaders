use rogalik::prelude::*;

pub const TILE_SIZE: f32 = 32.;
pub const SPRITE_SIZE: f32 = 16.;
pub const SPRITE_OFFSET: Vector2f = Vector2f {
    x: 0.5 * (TILE_SIZE - SPRITE_SIZE),
    y: 0.5 * (TILE_SIZE - SPRITE_SIZE),
};
pub const CARD_GAP: f32 = 3.;

pub const BUTTON_SIZE: f32 = SPRITE_SIZE + 2. * GAP;

pub const BUBBLE_Z: i32 = 150;
pub const OVERLAY_Z: i32 = 100;
pub const UI_Z: i32 = 200;

pub const UNIT_Z: i32 = 25;
pub const TILE_Z: i32 = 0;

pub const BASE_TEXT_SIZE: f32 = 8.0;
pub const DIGITS_TEXT_SIZE: f32 = 6.0;
pub const ICON_SIZE: f32 = 5.0;

pub const GAP: f32 = 4.;

pub const BUBBLE_SPEED: f32 = 0.5;
pub const BUBBLE_AGE: u32 = 100;

pub const MOVE_SPEED: f32 = 6. * TILE_SIZE;
pub const MOVE_THRESH: f32 = 0.1;

// PALETTE
pub const BACKGROUND_COLOR: Color = Color(40, 35, 40, 255);
pub const PRIMARY_COLOR: Color = Color(163, 162, 154, 255);
pub const RED_COLOR: Color = Color(197, 105, 129, 255);
pub const BLUE_COLOR: Color = Color(84, 92, 126, 255);
