use crate::gl::math::Mat4;
use glium::Surface;

use glium::glutin;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Painter<'a> {
    view: Mat4,
    target: &'a mut glium::Frame,
    verticies: &'a glium::VertexBuffer<Vertex>,
    indices: &'a glium::index::NoIndices,
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
                self.verticies,
                self.indices,
                self.program,
                &uniform,
                &params,
            )
            .unwrap();
    }
}

pub fn animation<F>(mut draw: F) -> !
where
    F: 'static + FnMut(Painter),
{
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut verticies = Vec::new();
    verticies.push(Vertex {
        position: [0.0, 0.0],
    });
    {
        let n = 30;
        for i in 0..(n + 1) {
            let a = 2.0 * std::f32::consts::PI * i as f32 / n as f32;
            verticies.push(Vertex {
                position: [a.cos(), a.sin()],
            });
        }
    }
    let verticies = glium::VertexBuffer::new(&display, &verticies).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

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
                glutin::event::WindowEvent::CursorMoved { .. } => (),
                glutin::event::WindowEvent::Resized { .. } => (),
                glutin::event::WindowEvent::Moved { .. } => (),
                _ => return,
            },
            glutin::event::Event::NewEvents(glutin::event::StartCause::ResumeTimeReached {
                ..
            }) => (),
            _ => return,
        }

        println!("{}", (std::time::Instant::now() - last_time).as_secs_f64());
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

        draw(Painter {
            view: view,
            target: &mut target,
            verticies: &verticies,
            indices: &indices,
            program: &program,
        });

        target.finish().unwrap();
    });
}
