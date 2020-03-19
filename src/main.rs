#![feature(clamp)]

#[macro_use]
extern crate glium;
extern crate rand;
extern crate rand_distr;
mod gl;
mod vec2;

use gl::math::Mat4;
use gl::window::animation;
use rand::{thread_rng, Rng};
use rand_distr::{Bernoulli, Cauchy, Uniform};
use vec2::V;

#[derive(Clone, Copy)]
enum State {
    Susceptible,
    Asymptomatic,
    Infected,
    Recovered,
    Dead,
}

impl State {
    fn color(&self) -> [f32; 3] {
        match self {
            State::Susceptible => [1.0, 1.0, 1.0],
            State::Asymptomatic => [1.0, 1.0, 1.0],
            State::Infected => [1.0, 0.0, 0.0],
            State::Recovered => [1.0, 1.0, 1.0],
            State::Dead => [1.0, 1.0, 1.0],
        }
    }
}

#[derive(Clone)]
struct Dot {
    new_pos: V,
    new_t: std::time::Instant,

    last_pos: V,
    last_t: std::time::Instant,

    state: State,
}

impl Dot {
    fn new(pos: V) -> Dot {
        Dot {
            new_pos: pos,
            new_t: std::time::Instant::now() + std::time::Duration::from_secs_f64(1.0),

            last_pos: pos,
            last_t: std::time::Instant::now(),

            state: State::Susceptible,
        }
    }

    fn pos(&self) -> V {
        let x = self.last_t.elapsed().as_secs_f64() / (self.new_t - self.last_t).as_secs_f64();
        let x = x.clamp(0.0, 1.0);
        x * self.new_pos + (1.0 - x) * self.last_pos
    }
    fn mov(&mut self, new_pos: V, dt: f64) {
        self.last_pos = self.pos();
        self.last_t = std::time::Instant::now();

        self.new_pos = new_pos;
        self.new_t = std::time::Instant::now() + std::time::Duration::from_secs_f64(dt);
    }
}

fn montecarlo(dots: &mut Vec<Dot>) {
    let mut rng = thread_rng();
    for i in 0..dots.len() {
        let phi = rng.sample(Uniform::new(0.0, 2.0 * std::f64::consts::PI));
        let dx = rng.sample(Cauchy::new(0.0, 0.10).unwrap()) * V::new(phi.cos(), phi.sin());

        fn pairwise_potential(r: f64) -> f64 {
            let d = 0.04;
            3.0 * ((d / r).powi(12) - (d / r).powi(6))
        }

        let a = dots[i].clone();

        let mut d_energy = 0.0;
        for b in dots.iter() {
            let r1 = (a.new_pos - b.new_pos).norm();
            let r2 = (a.new_pos + dx - b.new_pos).norm();

            if r1 == 0.0 || r2 == 0.0 {
                continue;
            }

            d_energy += pairwise_potential(r1) - pairwise_potential(r2);
        }

        fn global_potential(mut x: V) -> f64 {
            x *= std::f64::consts::PI;
            3.0 * (x.0.cos() + x.1.cos())
        }

        // global potential
        d_energy += global_potential(a.new_pos) - global_potential(a.new_pos + dx);
        let p = if d_energy > 0.0 { 1.0 } else { d_energy.exp() };

        if rng.sample(Bernoulli::new(p).unwrap()) {
            let p = dots[i].new_pos + dx;
            dots[i].mov(p, 0.2);
        }
    }
}

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

fn main() {
    let mut dots = Vec::new();

    let mut rng = thread_rng();
    for _ in 0..1000 {
        let x = V::new(rng.gen_range(-3.0, 3.0), rng.gen_range(-3.0, 3.0));
        dots.push(Dot::new(x));
    }
    dots[0].state = State::Infected;

    let mut t = 0.0;
    let mut t_montecarlo = 0.0;

    animation(move |mut painter, dt, _cursor, _left, _right| {
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

        *painter.view = Mat4::scale(1.0 / 3.0);

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
            let x = a.pos();
            painter.draw_circle(x.0 as f32, x.1 as f32, r as f32, a.state.color());
        }

        if t > t_montecarlo {
            let dmt = 0.3;
            t_montecarlo = t + dmt;

            for i in 0..dots.len() {
                for j in i + 1..dots.len() {
                    let (a, b) = index_twice(&mut dots, i, j).unwrap();
                    if (a.pos() - b.pos()).norm() < 3.0 * r {
                        match (a.state, b.state) {
                            (State::Infected, State::Susceptible) => {
                                b.state = State::Infected;
                            }
                            (State::Susceptible, State::Infected) => {
                                a.state = State::Infected;
                            }
                            _ => ()
                        }
                    }
                }
            }

            montecarlo(&mut dots);
        }
    });
}
