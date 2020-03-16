#[macro_use]
extern crate glium;
mod gl;

use gl::window::animation;

fn main() {
    animation(move |mut painter| {
        painter.draw_circle(0.0, 0.0, 1.0, [1.0, 1.0, 0.0f32]);
    });
}
