use rogalik::{input::MouseButton, prelude::*};

use game_graphics::input::{ButtonState, InputState};

pub fn get_input_state(camera: ResourceId, context: &Context) -> InputState {
    let mut click = ButtonState::Up;

    if context.input.is_mouse_button_down(MouseButton::Left) {
        click = ButtonState::Down
    }
    if context.input.is_mouse_button_released(MouseButton::Left) {
        click = ButtonState::Released
    }
    if context.input.is_mouse_button_pressed(MouseButton::Left) {
        click = ButtonState::Pressed
    }

    let m = context.input.get_mouse_physical_position();
    let mut w = Vector2f::ZERO;
    if let Some(camera) = context.graphics.get_camera(camera) {
        w = camera.camera_to_world(m);
    }

    InputState {
        click,
        mouse_world_position: w,
    }
}
