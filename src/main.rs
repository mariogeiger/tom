#[macro_use]
extern crate glium;
mod gl;

use gl::math::Mat4;
use glium::Surface;

use glium::glutin;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
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
        gl_Position = view * model * vec4(position, 0.0, 1.0);
    }
    "#;
    let fragment = r#"
    #version 150

    uniform vec4 uniform_color;

    out vec4 color;

    void main() {
        color = uniform_color;
    }
    "#;

    let program = glium::Program::from_source(&display, vertex, fragment, None).unwrap();

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
            _ => (),
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let view = Mat4::scale(1.0 / 10.0);

        let uniform = uniform! {
            model: Mat4::identity().as_array(),
            view: view.as_array(),
            uniform_color: [1.0, 0., 0., 1.0f32],
        };

        let params = Default::default();

        target
            .draw(&verticies, &indices, &program, &uniform, &params)
            .unwrap();

        target.finish().unwrap();
    });
}
