#[macro_use]
extern crate glium;
extern crate rand;
extern crate rand_distr;
mod gl;
mod vec2;

use gl::window::animation;
use vec2::V;
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

fn main() {
    let mut dots = Vec::new();

    let mut rng = thread_rng();
    for _ in 0..3000 {
        let x = V::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0));
        let v = V::new(rng.sample(StandardNormal), rng.sample(StandardNormal));
        dots.push((x, v, false));
    }

    animation(move |mut painter, dt, cursor, left, _right| {
        for (_pos, _vel, stop) in &mut dots {
            *stop = false;
        }

        if let Some((x, y)) = cursor {
            let r = 0.2;

            for px in &[-2.0, 0.0, 2.0] {
                for py in &[-2.0, 0.0, 2.0] {
                    // periodic mouse
                    let x = x + px;
                    let y = y + py;

                    if left {
                        for (pos, _vel, stop) in &mut dots {
                            *stop |= (V::new(x, y) - *pos).norm() < r;
                        }

                        painter.draw_circle(x as f32, y as f32, r as f32, [0.2, 0.0, 0.0]);
                    } else {
                        painter.draw_circle(x as f32, y as f32, r as f32, [0.1, 0.1, 0.1]);
                    }
                }
            }
        }

        for (pos, vel, stop) in &mut dots {
            // move the dot
            if !*stop {
                *pos += dt * *vel;
            }

            // priodic boundary condition
            if pos.0 < -1.0 { pos.0 += 2.0; }
            if pos.0 > 1.0 { pos.0 -= 2.0; }
            if pos.1 < -1.0 { pos.1 += 2.0; }
            if pos.1 > 1.0 { pos.1 -= 2.0; }

            painter.draw_circle(pos.0 as f32, pos.1 as f32, 0.005, [0.0, 1.0, 1.0]);
        }

        painter.draw_rect(-2.0, 1.0, 4.0, 4.0, [0.0, 0.0, 0.0]);
        painter.draw_rect(-2.0, -1.0, 4.0, -4.0, [0.0, 0.0, 0.0]);

        painter.draw_rect(-1.0, -2.0, -4.0, 4.0, [0.0, 0.0, 0.0]);
        painter.draw_rect(1.0, -2.0, 4.0, 4.0, [0.0, 0.0, 0.0]);
    });
}
