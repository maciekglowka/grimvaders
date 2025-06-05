use rogalik::prelude::*;

use super::Span;
use crate::{
    globals::SPRITE_SIZE,
    input::{ButtonState, InputState},
};

#[derive(Clone)]
pub struct Button<'a> {
    pub origin: Vector2f,
    size: Vector2f,
    z: i32,
    sprite_atlas: &'a str,
    sprite_index: usize,
    span: Option<Span<'a>>,
    slice: Option<(usize, Vector2f)>,
}
impl<'a> Button<'a> {
    pub fn new(origin: Vector2f, size: Vector2f, z: i32) -> Self {
        Button {
            origin,
            size,
            z,
            sprite_atlas: "ui",
            sprite_index: 0,
            span: None,
            slice: Some((8, Vector2f::splat(SPRITE_SIZE))),
        }
    }
    pub fn with_span(mut self, span: Span<'a>) -> Self {
        self.span = Some(span);
        self
    }
    pub fn with_sprite(mut self, atlas: &'a str, index: usize) -> Self {
        self.sprite_atlas = atlas;
        self.sprite_index = index;
        self
    }
    // pub fn with_slice(mut self, slice: (usize, Vector2f)) -> Self {
    //     self.slice = Some(slice);
    //     self
    // }
    pub fn draw(&self, context: &mut Context, state: &InputState) {
        let mut idx = self.sprite_index;
        let mut text_shift = 0.;
        if self.pressed(state) {
            idx = self.sprite_index + 1;
            text_shift = -2.;
        }

        let _ = context.graphics.draw_atlas_sprite(
            self.sprite_atlas,
            idx,
            self.origin,
            self.z,
            self.size,
            SpriteParams {
                slice: self.slice,
                ..Default::default()
            },
        );
        if let Some(span) = &self.span {
            let span_offset = Vector2f::new(
                0.5 * (self.size.x - span.width(context)),
                0.5 * (self.size.y - span.height()) + text_shift + 1.,
            );
            span.draw(self.origin + span_offset, self.z + 1, context);
        }
    }

    pub fn clicked(&self, state: &InputState) -> bool {
        state.click == ButtonState::Released && self.mouse_over(state)
    }

    fn pressed(&self, state: &InputState) -> bool {
        state.click == ButtonState::Down && self.mouse_over(state)
    }

    pub fn mouse_over(&self, state: &InputState) -> bool {
        crate::utils::is_mouse_over(self.origin, self.size, state)
    }
}
