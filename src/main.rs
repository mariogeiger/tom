#[macro_use]
extern crate glium;
extern crate rand;
extern crate rand_distr;
mod gl;
mod vec2;

use gl::window::animation;
use rand::{thread_rng, Rng};
use rand_distr::{Bernoulli, Cauchy, Uniform};
use vec2::V;

#[derive(Clone)]
struct Dot {
    pos: V,
    vel: V,
    stop: bool,
    state: bool,
}

fn montecarlo(dots: &mut Vec<Dot>) {
    let mut rng = thread_rng();
    for i in 0..dots.len() {
        let phi = rng.sample(Uniform::new(0.0, 2.0 * std::f64::consts::PI));
        let dx = rng.sample(Cauchy::new(0.0, 0.05).unwrap()) * V::new(phi.cos(), phi.sin());

        fn pairwise_potential(r: f64) -> f64 {
            let d = 0.04;
            4.0 * ((d / r).powi(12) - (d / r).powi(6))
        }

        let a = dots[i].clone();

        let mut d_energy = 0.0;
        for b in dots.iter() {
            let r1 = (a.pos - b.pos).norm();
            let r2 = (a.pos + dx - b.pos).norm();

            if r1 == 0.0 || r2 == 0.0 {
                continue;
            }

            d_energy += pairwise_potential(r1) - pairwise_potential(r2);
        }

        // global potential
        let x = V::new(0.0, 0.0);
        let r1 = (a.pos - x).norm();
        let r2 = (a.pos + dx - x).norm();
        d_energy += 10.0 * r1.powi(2) - 10.0 * r2.powi(2);
        let p = if d_energy > 0.0 { 1.0 } else { d_energy.exp() };

        if rng.sample(Bernoulli::new(p).unwrap()) {
            dots[i].pos += dx;
        }
    }
}

fn main() {
    let mut dots = Vec::new();

    let mut rng = thread_rng();
    for _ in 0..100 {
        let x = V::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0));
        let v = V::new(0.0, 0.0);
        dots.push(Dot {
            pos: x,
            vel: v,
            stop: false,
            state: false,
        });
    }
    dots[0].state = true;

    let mut t = 0.0;
    let mut t_montecarlo = 0.0;

    animation(move |mut painter, dt, cursor, left, _right| {
        t += dt;

        // for a in &mut dots {
        //     a.stop = false;
        // }

        // if let Some((x, y)) = cursor {
        //     let r = 0.2;

        //     for px in &[-2.0, 0.0, 2.0] {
        //         for py in &[-2.0, 0.0, 2.0] {
        //             // periodic mouse
        //             let x = x + px;
        //             let y = y + py;

        //             if left {
        //                 for a in &mut dots {
        //                     a.stop |= (V::new(x, y) - a.pos).norm() < r;
        //                     if a.stop {
        //                         a.state = false;
        //                     }
        //                 }

        //                 painter.draw_circle(x as f32, y as f32, r as f32, [0.4, 0.0, 0.2]);
        //             } else {
        //                 painter.draw_circle(x as f32, y as f32, r as f32, [0.1, 0.1, 0.1]);
        //             }
        //         }
        //     }
        // }

        let r = 0.02;

        for a in dots.iter_mut() {
            // move the dot
            // if !a.stop {
            //     a.pos += dt * a.vel;
            // }

            // let color = if a.state {
            //     [0.6, 0.85, 1.0]
            // } else {
            //     [0.3, 0.4, 0.5]
            // };

            painter.draw_circle(a.pos.0 as f32, a.pos.1 as f32, r as f32, [0.6, 0.85, 1.0]);
        }

        if t > t_montecarlo {
            let dmt = 1.0;
            t_montecarlo = t + dmt;

            montecarlo(&mut dots);
        }
    });
}
