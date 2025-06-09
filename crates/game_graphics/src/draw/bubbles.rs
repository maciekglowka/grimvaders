use rand::prelude::*;
use rogalik::prelude::*;

use crate::{
    globals::{
        BUBBLE_MAX_AGE, BUBBLE_SPEED, BUBBLE_WAIT_AGE, BUBBLE_Z, DIGITS_TEXT_SIZE, ICON_SIZE,
        SPRITE_SIZE,
    },
    ui::Span,
};

pub struct Bubble {
    pub origin: Vector2f,
    pub age: f32,
    pub color: Color,
    pub text: Option<String>,
    pub icon: Option<usize>,
}
impl Bubble {
    pub fn new(origin: Vector2f, color: Color, text: Option<String>, icon: Option<usize>) -> Self {
        let mut rng = thread_rng();
        let offset =
            SPRITE_SIZE * Vector2f::new(rng.gen_range(0.25..0.75), rng.gen_range(0.25..0.75));
        Self {
            origin: origin + offset,
            color,
            text,
            icon,
            age: 0.,
        }
    }
}

pub(crate) fn update_bubbles(bubbles: &mut Vec<Bubble>, context: &mut Context) -> bool {
    let wait = move_bubbles(bubbles, context.time.get_delta());
    remove_old_bubbles(bubbles);
    draw_bubbles(bubbles, context);
    wait
}

fn remove_old_bubbles(bubbles: &mut Vec<Bubble>) {
    bubbles.retain(|a| a.age < BUBBLE_MAX_AGE);
}

fn move_bubbles(bubbles: &mut Vec<Bubble>, delta: f32) -> bool {
    let mut wait = false;
    for bubble in bubbles.iter_mut() {
        bubble.origin += Vector2f::new(0., BUBBLE_SPEED);
        bubble.age += delta;
        if bubble.age <= BUBBLE_WAIT_AGE {
            wait = true;
        }
    }
    wait
}

fn draw_bubbles(bubbles: &mut Vec<Bubble>, context: &mut Context) {
    for bubble in bubbles.iter() {
        let mut span = Span::new().with_font("digits");
        if let Some(text) = &bubble.text {
            span = span
                .with_text_borrowed(text)
                .with_text_size(DIGITS_TEXT_SIZE)
                .with_text_color(bubble.color)
                .with_spacer(2.);
        };
        if let Some(icon) = bubble.icon {
            span = span
                .with_sprite("icons_small", icon)
                .with_sprite_size(ICON_SIZE);
        };
        span.draw(bubble.origin.round(), BUBBLE_Z, context);
    }
}
