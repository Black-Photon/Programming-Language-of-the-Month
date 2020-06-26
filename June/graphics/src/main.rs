#[macro_use]
extern crate glium;
use glium::{glutin, Surface};
use glium::backend::glutin::glutin::ContextCurrentState;
use glium::index::{IndicesSource, NoIndices};

use std::fs::read_to_string;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

fn main() {
    // Setup event loop and widow params
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let (wb, cb) = setup_window(wb, cb);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let (vertex_buffer, indices, program) = prepare(&display);

    event_loop.run(move |ev, _, control_flow| {

        main_loop(&display, &vertex_buffer, indices.into(), &program);

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);


        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}

fn setup_window
    <T: ContextCurrentState>
    (mut wb: glutin::window::WindowBuilder, cb: glutin::ContextBuilder<T>) ->
        (glutin::window::WindowBuilder, glutin::ContextBuilder<T>) {
    wb = wb.with_title("My awesome spinning cube!");
    return (wb, cb)
}

fn prepare(display: &glium::Display) -> (glium::VertexBuffer<Vertex>, NoIndices, glium::Program) {
    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = read_to_string("shaders/vertex.vert ").unwrap();
    let fragment_shader_src = read_to_string("shaders/fragment.frag").unwrap();

    let program = glium::Program::from_source(display, vertex_shader_src.as_ref(), fragment_shader_src.as_ref(), None).unwrap();

    return (vertex_buffer, indices, program);
}

fn main_loop(display: &glium::Display, vertex_buffer: &glium::VertexBuffer<Vertex>, indices: IndicesSource, program: &glium::Program) {
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(vertex_buffer, indices, program, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();
    target.finish().unwrap();
}
