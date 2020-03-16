#[macro_use]
extern crate glium;
mod gl;

use gl::window::animation;

fn main() {
    let mut dots = Vec::new();

    animation(move |mut painter, _dt, cursor, left, _right| {
        painter.draw_circle(0.0, 0.0, 1.0, [1.0, 1.0, 0.0f32]);

        for (x, y) in &dots {
            painter.draw_circle(*x as f32, *y as f32, 0.05, [0.0, 1.0, 1.0f32]);
        }

        if let Some((x, y)) = cursor {
            painter.draw_circle(x as f32, y as f32, 0.01, [1.0, 1.0, 1.0f32]);
            if left {
                dots.push((x, y));
            }
        }
    });
}
