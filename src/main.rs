#[macro_use]
extern crate glium;
mod gl;

use gl::window::animation;

fn main() {
    animation(move |mut painter, _dt, cursor| {
        painter.draw_circle(0.0, 0.0, 1.0, [1.0, 1.0, 0.0f32]);

        if let Some((x, y)) = cursor {
            painter.draw_circle(x as f32, y as f32, 0.1, [0.0, 0.5, 1.0f32]);
        }
    });
}
