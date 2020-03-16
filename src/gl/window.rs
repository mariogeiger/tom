use crate::gl::math::Mat4;
use glium::Surface;

use glium::glutin;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn new(x: f32, y: f32) -> Vertex {
        Vertex { position: [x, y] }
    }
}

implement_vertex!(Vertex, position);

pub struct Painter<'a> {
    view: Mat4,
    target: &'a mut glium::Frame,
    circle_verticies: &'a glium::VertexBuffer<Vertex>,
    square_verticies: &'a glium::VertexBuffer<Vertex>,
    program: &'a glium::Program,
}

impl<'a> Painter<'a> {
    pub fn draw_circle(&mut self, x: f32, y: f32, r: f32, color: [f32; 3]) {
        let uniform = uniform! {
            model: (Mat4::translation(x, y, 0.0) * Mat4::scale(r)).as_array(),
            view: self.view.as_array(),
            uniform_color: color,
        };

        let params = Default::default();

        self.target
            .draw(
                self.circle_verticies,
                &glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
                self.program,
                &uniform,
                &params,
            )
            .unwrap();
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 3]) {
        let uniform = uniform! {
            model: (Mat4::translation(x, y, 0.0) * Mat4::diag(w, h, 1.0, 1.0)).as_array(),
            view: self.view.as_array(),
            uniform_color: color,
        };

        let params = Default::default();

        self.target
            .draw(
                self.square_verticies,
                &glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
                self.program,
                &uniform,
                &params,
            )
            .unwrap();
    }
}

pub fn animation<F>(mut draw: F) -> !
where
    F: 'static + FnMut(Painter, f64, Option<(f64, f64)>, bool, bool),
{
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut circle_verticies = Vec::new();
    circle_verticies.push(Vertex::new(0.0, 0.0));
    {
        let n = 30;
        for i in 0..(n + 1) {
            let a = 2.0 * std::f32::consts::PI * i as f32 / n as f32;
            circle_verticies.push(Vertex::new(a.cos(), a.sin()));
        }
    }
    let circle_verticies = glium::VertexBuffer::new(&display, &circle_verticies).unwrap();

    let mut square_verticies = Vec::new();
    square_verticies.push(Vertex::new(0.0, 0.0));
    square_verticies.push(Vertex::new(1.0, 0.0));
    square_verticies.push(Vertex::new(1.0, 1.0));
    square_verticies.push(Vertex::new(0.0, 1.0));
    let square_verticies = glium::VertexBuffer::new(&display, &square_verticies).unwrap();

    let vertex = r#"
    #version 150

    in vec2 position;

    uniform mat4 model;
    uniform mat4 view;

    void main() {
        gl_Position = view * model * vec4(position, 0, 1);
    }
    "#;
    let fragment = r#"
    #version 150

    uniform vec3 uniform_color;

    out vec4 color;

    void main() {
        color = vec4(uniform_color, 1);
    }
    "#;

    let program = glium::Program::from_source(&display, vertex, fragment, None).unwrap();

    let mut last_time = std::time::Instant::now();

    let mut cursor = None;
    let mut left = false;
    let mut right = false;

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match &event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => (),
            },
            glutin::event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached {
                ..
            }) => (),
            _ => return,
        }

        let dt = (std::time::Instant::now() - last_time).as_secs_f64();
        last_time = std::time::Instant::now();

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let (width, height) = target.get_dimensions();

        let aspect_ratio = width as f32 / height as f32;
        let view = if aspect_ratio < 1.0 {
            Mat4::diag(1.0, aspect_ratio, 1.0, 1.0)
        } else {
            Mat4::diag(1.0 / aspect_ratio, 1.0, 1.0, 1.0)
        };

        match &event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    let x = position.x / width as f64 * 2.0 - 1.0;
                    let y = 1.0 - position.y / height as f64 * 2.0;
                    cursor = Some((
                        x / view.as_array()[0][0] as f64,
                        y / view.as_array()[1][1] as f64,
                    ));
                }
                glutin::event::WindowEvent::MouseInput { state, button, .. } => match button {
                    glutin::event::MouseButton::Left => {
                        left = state == &glutin::event::ElementState::Pressed
                    }
                    glutin::event::MouseButton::Right => {
                        right = state == &glutin::event::ElementState::Pressed
                    }
                    _ => (),
                },
                glutin::event::WindowEvent::CursorEntered { .. } => {}
                glutin::event::WindowEvent::CursorLeft { .. } => {
                    cursor = None;
                }
                _ => (),
            },
            _ => (),
        }

        draw(
            Painter {
                view: view,
                target: &mut target,
                circle_verticies: &circle_verticies,
                square_verticies: &square_verticies,
                program: &program,
            },
            dt,
            cursor,
            left,
            right,
        );

        target.finish().unwrap();
    });
}
