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
use std::time::{Duration, Instant};
use vec2::V;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Susceptible,
    Asymptomatic(Instant),
    Infected(Instant),
    Recovered,
    Dead,
}

impl State {
    fn color(&self) -> [f32; 3] {
        match self {
            State::Susceptible => [1.0, 1.0, 1.0],
            State::Asymptomatic(_) => [1.0, 1.0, 1.0],
            State::Infected(_) => [1.0, 0.0, 0.0],
            State::Recovered => [0.0, 1.0, 0.0],
            State::Dead => [1.0, 0.0, 1.0],
        }
    }
}

#[derive(Clone, PartialEq)]
struct Dot {
    new_pos: V,
    new_t: Instant,

    last_pos: V,
    last_t: Instant,

    state: State,
}

impl Dot {
    fn new(pos: V) -> Dot {
        Dot {
            new_pos: pos,
            new_t: Instant::now() + Duration::from_secs_f64(1.0),

            last_pos: pos,
            last_t: Instant::now(),

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
        self.last_t = Instant::now();

        self.new_pos = new_pos;
        self.new_t = Instant::now() + Duration::from_secs_f64(dt);
    }
}

fn montecarlo(dots: &mut Vec<Dot>) {
    let mut rng = thread_rng();
    for i in 0..dots.len() {
        let a = dots[i].clone();

        if a.state == State::Dead {
            continue;
        }

        let mut dx;
        loop {
            let phi = rng.sample(Uniform::new(0.0, 2.0 * std::f64::consts::PI));
            dx = rng.sample(Cauchy::new(0.0, 0.10).unwrap()) * V::new(phi.cos(), phi.sin());
            let new_pos = a.new_pos + dx;

            if new_pos.norm() < 5.0 {
                break;
            }
        }

        fn pairwise_attractive(r: f64) -> f64 {
            let d = 0.04;
            3.0 * ((d / r).powi(12) - (d / r).powi(6))
        }
        fn pairwise_repulsive(r: f64) -> f64 {
            let d = 0.04;
            3.0 * (d / r).powi(2)
        }

        let mut d_energy = 0.0;
        for b in dots.iter() {
            let r1 = (a.new_pos - b.new_pos).norm();
            let r2 = (a.new_pos + dx - b.new_pos).norm();

            if r1 == 0.0 || r2 == 0.0 {
                continue;
            }

            d_energy += match (a.state, b.state) {
                (State::Infected(_), State::Infected(_)) => {
                    pairwise_attractive(r1) - pairwise_attractive(r2)
                }
                (State::Infected(_), _) => pairwise_repulsive(r1) - pairwise_repulsive(r2),
                (_, State::Infected(_)) => pairwise_repulsive(r1) - pairwise_repulsive(r2),
                (_, _) => pairwise_attractive(r1) - pairwise_attractive(r2),
            };
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
        let phi = rng.sample(Uniform::new(0.0, 2.0 * std::f64::consts::PI));
        let x = rng.sample(Uniform::new(0.0, 5.0)) * V::new(phi.cos(), phi.sin());
        dots.push(Dot::new(x));
    }
    dots[0].state = State::Asymptomatic(Instant::now() + Duration::from_secs_f64(5.0));

    let mut t = 0.0;
    let mut t_montecarlo = 0.0;

    animation(move |mut painter, dt, _cursor, _left, _right| {
        t += dt;

        *painter.view = Mat4::scale(1.0 / 5.0);

        let r = 0.02;

        for a in dots.iter_mut() {
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
                            (State::Infected(_), State::Susceptible)
                            | (State::Asymptomatic(_), State::Susceptible) => {
                                b.state = State::Asymptomatic(
                                    Instant::now() + Duration::from_secs_f64(5.0),
                                );
                            }
                            (State::Susceptible, State::Infected(_))
                            | (State::Susceptible, State::Asymptomatic(_)) => {
                                a.state = State::Asymptomatic(
                                    Instant::now() + Duration::from_secs_f64(5.0),
                                );
                            }
                            _ => (),
                        }
                    }
                }

                if let State::Asymptomatic(t) = dots[i].state {
                    if t < Instant::now() {
                        dots[i].state =
                            State::Infected(Instant::now() + Duration::from_secs_f64(10.0));
                    }
                }
                if let State::Infected(t) = dots[i].state {
                    if t < Instant::now() {
                        if rng.sample(Bernoulli::new(0.5).unwrap()) {
                            dots[i].state = State::Dead;
                        } else {
                            dots[i].state = State::Recovered;
                        }
                    }
                }
            }

            montecarlo(&mut dots);
        }
    });
}
