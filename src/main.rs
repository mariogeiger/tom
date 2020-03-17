#[macro_use]
extern crate glium;
extern crate rand;
extern crate rand_distr;
mod gl;
mod vec2;

use gl::window::animation;
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;
use std::collections::HashMap;
use vec2::V;

fn index_twice<T>(slc: &mut [T], a: usize, b: usize) -> Option<(&mut T, &mut T)> {
    if a == b {
        None
    } else {
        if a >= slc.len() || b >= slc.len() {
            None
        } else {
            // safe because a, b are in bounds and distinct
            unsafe {
                let ar = &mut *(slc.get_unchecked_mut(a) as *mut _);
                let br = &mut *(slc.get_unchecked_mut(b) as *mut _);
                Some((ar, br))
            }
        }
    }
}

struct Dot {
    pos: V,
    vel: V,
    stop: bool,
    state: bool,
}

fn main() {
    let mut dots = Vec::new();

    let mut rng = thread_rng();
    for _ in 0..6000 {
        let x = V::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0));
        let v = 0.5 * V::new(rng.sample(StandardNormal), rng.sample(StandardNormal));
        dots.push(Dot {
            pos: x,
            vel: v,
            stop: false,
            state: false,
        });
    }
    dots[0].state = true;

    animation(move |mut painter, dt, cursor, left, _right| {
        for a in &mut dots {
            a.stop = false;
        }

        if let Some((x, y)) = cursor {
            let r = 0.2;

            for px in &[-2.0, 0.0, 2.0] {
                for py in &[-2.0, 0.0, 2.0] {
                    // periodic mouse
                    let x = x + px;
                    let y = y + py;

                    if left {
                        for a in &mut dots {
                            a.stop |= (V::new(x, y) - a.pos).norm() < r;
                            if a.stop {
                                a.state = false;
                            }
                        }

                        painter.draw_circle(x as f32, y as f32, r as f32, [0.4, 0.0, 0.2]);
                    } else {
                        painter.draw_circle(x as f32, y as f32, r as f32, [0.1, 0.1, 0.1]);
                    }
                }
            }
        }

        let r = 0.01;

        for a in dots.iter_mut() {
            // move the dot
            if !a.stop {
                a.pos += dt * a.vel;
            }

            // priodic boundary condition
            if a.pos.0 < -1.0 {
                a.pos.0 += 2.0;
            }
            if a.pos.0 > 1.0 {
                a.pos.0 -= 2.0;
            }
            if a.pos.1 < -1.0 {
                a.pos.1 += 2.0;
            }
            if a.pos.1 > 1.0 {
                a.pos.1 -= 2.0;
            }

            let color = if a.state {
                [0.6, 0.85, 1.0]
            } else {
                [0.3, 0.4, 0.5]
            };

            painter.draw_circle(a.pos.0 as f32, a.pos.1 as f32, r as f32, color);
        }

        // collision, for optimization we use a table
        let mut table: HashMap<_, Vec<_>> = HashMap::new();
        for (i, a) in dots.iter().enumerate() {
            let overlap = 1.1 * r;
            let x = (a.pos.0 / overlap) as isize;
            let y = (a.pos.1 / overlap) as isize;
            let size = 4;
            table.entry((x / size, y / size)).or_default().push(i);
            match (x % size == 0, y % size == 0) {
                (false, false) => {}
                (true, false) => {
                    table.entry((x / size - 1, y / size)).or_default().push(i);
                }
                (false, true) => {
                    table.entry((x / size, y / size - 1)).or_default().push(i);
                }
                (true, true) => {
                    table.entry((x / size - 1, y / size)).or_default().push(i);
                    table.entry((x / size, y / size - 1)).or_default().push(i);
                    table
                        .entry((x / size - 1, y / size - 1))
                        .or_default()
                        .push(i);
                }
            }
        }

        for list in table.values_mut() {
            for i in 0..list.len() {
                for j in i + 1..list.len() {
                    if let Some((a, b)) = index_twice(&mut dots, list[i], list[j]) {
                        let n = b.pos - a.pos;
                        let nn = V::dot(n, n);
                        if nn < 4.0 * r * r {
                            let state = a.state || b.state;
                            a.state = state;
                            b.state = state;
                            match (a.stop, b.stop) {
                                (false, false) => {
                                    let vf = (a.vel + b.vel) / 2.0;
                                    a.vel -= vf;
                                    b.vel -= vf;
                                    let van = V::dot(a.vel, n);
                                    if van > 0.0 {
                                        a.vel -= 2.0 * van * n / nn;
                                        b.vel = -a.vel;
                                    }
                                    a.vel += vf;
                                    b.vel += vf;
                                }
                                (false, true) => {
                                    let van = V::dot(a.vel, n);
                                    if van > 0.0 {
                                        a.vel -= 2.0 * van * n / nn;
                                    }
                                }
                                (true, false) => {
                                    let vbn = V::dot(b.vel, n);
                                    if vbn < 0.0 {
                                        b.vel -= 2.0 * vbn * n / nn;
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }

        painter.draw_rect(-2.0, 1.0, 4.0, 4.0, [0.0, 0.0, 0.0]);
        painter.draw_rect(-2.0, -1.0, 4.0, -4.0, [0.0, 0.0, 0.0]);

        painter.draw_rect(-1.0, -2.0, -4.0, 4.0, [0.0, 0.0, 0.0]);
        painter.draw_rect(1.0, -2.0, 4.0, 4.0, [0.0, 0.0, 0.0]);
    });
}
