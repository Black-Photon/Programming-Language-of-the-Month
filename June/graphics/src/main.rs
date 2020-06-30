#[macro_use]
extern crate glium;
use glium::{glutin, Surface};
use glium::backend::glutin::glutin::ContextCurrentState;
use glium::index::IndicesSource;

use std::fs::read_to_string;
use std::f32::consts::PI;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

fn main() {
    // Setup event loop and widow params
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let (wb, cb) = setup_window(wb, cb);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let (vertex_buffer, indices, program) = prepare(&display);

    let start = std::time::Instant::now();
    let mut current = start;

    event_loop.run(move |ev, _, control_flow| {
        let now = std::time::Instant::now();
        let delta_t = now.duration_since(current);
        let abs_t = now.duration_since(start);
        current = now;

        main_loop(&display, &vertex_buffer, IndicesSource::from(&indices), &program, delta_t.as_secs_f32(), abs_t.as_secs_f32());

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
    wb = wb.with_maximized(true);
    return (wb, cb)
}

fn prepare(display: &glium::Display) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>, glium::Program) {
    implement_vertex!(Vertex, position, normal);

    let shape = vec![
        Vertex { position: [1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, // 0
        Vertex { position: [1.0, -1.0, 1.0], normal: [0.0, 0.0, 1.0] }, // 1
        Vertex { position: [-1.0, -1.0, 1.0], normal: [0.0, 0.0, 1.0] }, // 2
        Vertex { position: [-1.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0] }, // 3

        Vertex { position: [1.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0] }, // 0
        Vertex { position: [1.0, -1.0, 1.0], normal: [1.0, 0.0, 0.0] }, // 1
        Vertex { position: [1.0, 1.0, -1.0], normal: [1.0, 0.0, 0.0] }, // 4
        Vertex { position: [1.0, -1.0, -1.0], normal: [1.0, 0.0, 0.0] }, // 5

        Vertex { position: [-1.0, -1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, // 2
        Vertex { position: [-1.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, // 3
        Vertex { position: [-1.0, -1.0, -1.0], normal: [-1.0, 0.0, 0.0] }, // 6
        Vertex { position: [-1.0, 1.0, -1.0], normal: [-1.0, 0.0, 0.0] }, // 7

        Vertex { position: [1.0, 1.0, -1.0], normal: [0.0, 0.0, -1.0] }, // 4
        Vertex { position: [1.0, -1.0, -1.0], normal: [0.0, 0.0, -1.0] }, // 5
        Vertex { position: [-1.0, -1.0, -1.0], normal: [0.0, 0.0, -1.0] }, // 6
        Vertex { position: [-1.0, 1.0, -1.0], normal: [0.0, 0.0, -1.0] }, // 7

        Vertex { position: [1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0] }, // 0
        Vertex { position: [-1.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0] }, // 3
        Vertex { position: [1.0, 1.0, -1.0], normal: [0.0, 1.0, 0.0] }, // 4
        Vertex { position: [-1.0, 1.0, -1.0], normal: [0.0, 1.0, 0.0] }, // 7

        Vertex { position: [1.0, -1.0, 1.0], normal: [0.0, -1.0, 0.0] }, // 1
        Vertex { position: [-1.0, -1.0, 1.0], normal: [0.0, -1.0, 0.0] }, // 2
        Vertex { position: [1.0, -1.0, -1.0], normal: [0.0, -1.0, 0.0] }, // 5
        Vertex { position: [-1.0, -1.0, -1.0], normal: [0.0, -1.0, 0.0] }, // 6
    ];
    let indices = vec![
        0, 1, 2,
        0, 2, 3,

        4, 5, 6,
        5, 6, 7,

        8, 9, 10,
        9, 10, 11,

        12, 13, 14,
        12, 14, 15,

        16, 17, 18,
        17, 18, 19,

        20, 21, 22,
        21, 22, 23
    ];

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
    let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                          &indices).unwrap();

    let vertex_shader_src = read_to_string("shaders/vertex.vert ").unwrap();
    let fragment_shader_src = read_to_string("shaders/fragment.frag").unwrap();

    let program = glium::Program::from_source(display, vertex_shader_src.as_ref(), fragment_shader_src.as_ref(), None).unwrap();

    return (vertex_buffer, indices, program);
}

fn main_loop(display: &glium::Display, vertex_buffer: &glium::VertexBuffer<Vertex>, indices: IndicesSource, program: &glium::Program, delta_t: f32, abs_t: f32) {
    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

    let model = {
        let mut time = abs_t * 10.0;
        while time > 360.0 {
            time -= 360.0;
        }
        invert_mat4(rotate([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ], time, time, time))
    };
    let view = invert_mat4({
        let cam_pos = [0.0, 0.0, 8.0];
        let cam_target = [0.0, 0.0, 0.0];
        let abs_up = [0.0, 1.0, 0.0];

        look_at(cam_pos, cam_target, abs_up)
    });
    let projection = invert_mat4({
        perspective(90.0, 1920.0/1080.0, 1.0, 10.0)
    });
    #[derive(Copy, Clone)]
    struct Camera {
        position: [f32; 3]
    }
    let uniforms = uniform!{
        model: model,
        view: view,
        projection: projection,
        light: [3.0, 3.0, 3.0f32],
        camera: [0.0, 0.0, 8.0f32]
    };

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    target.draw(vertex_buffer, indices, program, &uniforms, &params).unwrap();
    target.finish().unwrap();
}

fn mat4_mult_f32(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    return [
        [
            a[0][0]*b[0][0] + a[0][1]*b[1][0] + a[0][2]*b[2][0] + a[0][3]*b[3][0],
            a[0][0]*b[0][1] + a[0][1]*b[1][1] + a[0][2]*b[2][1] + a[0][3]*b[3][1],
            a[0][0]*b[0][2] + a[0][1]*b[1][2] + a[0][2]*b[2][2] + a[0][3]*b[3][2],
            a[0][0]*b[0][3] + a[0][1]*b[1][3] + a[0][2]*b[2][3] + a[0][3]*b[3][3]
        ],
        [
            a[1][0]*b[0][0] + a[1][1]*b[1][0] + a[1][2]*b[2][0] + a[1][3]*b[3][0],
            a[1][0]*b[0][1] + a[1][1]*b[1][1] + a[1][2]*b[2][1] + a[1][3]*b[3][1],
            a[1][0]*b[0][2] + a[1][1]*b[1][2] + a[1][2]*b[2][2] + a[1][3]*b[3][2],
            a[1][0]*b[0][3] + a[1][1]*b[1][3] + a[1][2]*b[2][3] + a[1][3]*b[3][3]
        ],
        [
            a[2][0]*b[0][0] + a[2][1]*b[1][0] + a[2][2]*b[2][0] + a[2][3]*b[3][0],
            a[2][0]*b[0][1] + a[2][1]*b[1][1] + a[2][2]*b[2][1] + a[2][3]*b[3][1],
            a[2][0]*b[0][2] + a[2][1]*b[1][2] + a[2][2]*b[2][2] + a[2][3]*b[3][2],
            a[2][0]*b[0][3] + a[2][1]*b[1][3] + a[2][2]*b[2][3] + a[2][3]*b[3][3]
        ],
        [
            a[3][0]*b[0][0] + a[3][1]*b[1][0] + a[3][2]*b[2][0] + a[3][3]*b[3][0],
            a[3][0]*b[0][1] + a[3][1]*b[1][1] + a[3][2]*b[2][1] + a[3][3]*b[3][1],
            a[3][0]*b[0][2] + a[3][1]*b[1][2] + a[3][2]*b[2][2] + a[3][3]*b[3][2],
            a[3][0]*b[0][3] + a[3][1]*b[1][3] + a[3][2]*b[2][3] + a[3][3]*b[3][3]
        ]
    ];
}

fn vec3_norm(vec: [f32; 3]) -> [f32; 3] {
    let sum = vec[0]*vec[0] + vec[1]*vec[1] + vec[2]*vec[2];
    let sqrt = sum.sqrt();
    return vec3_div(vec, sqrt);
}

fn vec3_cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0]
    ];
}

fn vec3_sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

fn vec3_mult(a: [f32; 3], b: f32) -> [f32; 3] {
    return [a[0] * b, a[1] * b, a[2] * b];
}

fn vec3_div(a: [f32; 3], b: f32) -> [f32; 3] {
    return [a[0] / b, a[1] / b, a[2] / b];
}

fn apply_mat4(mat: [[f32; 4]; 4], vec: [f32; 4]) -> [f32; 4] {
    return [
        vec[0] * mat[0][0] + vec[1] * mat[0][1] + vec[2] * mat[0][2] + vec[3] * mat[0][3],
        vec[0] * mat[1][0] + vec[1] * mat[1][1] + vec[2] * mat[1][2] + vec[3] * mat[1][3],
        vec[0] * mat[2][0] + vec[1] * mat[2][1] + vec[2] * mat[2][2] + vec[3] * mat[2][3],
        vec[0] * mat[3][0] + vec[1] * mat[3][1] + vec[2] * mat[3][2] + vec[3] * mat[3][3]
    ];
}

fn restore_vec4(vec: [f32; 4]) -> [f32; 4] {
    return [vec[0]/vec[3], vec[1]/vec[3], vec[2]/vec[3], 1.0];
}

unsafe fn print_sequence(init: [f32; 4], model: [[f32; 4]; 4], view: [[f32; 4]; 4], projection: [[f32; 4]; 4]) {
    let imodel = apply_mat4(model, init);
    let iview = apply_mat4(view, imodel);
    let iproj = restore_vec4(apply_mat4(projection, iview));
    println!("Initial: {:?}", init);
    println!("Model: {:?}", imodel);
    println!("View: {:?}", iview);
    println!("Projection: {:?}", iproj);
}

fn look_at(cam_pos: [f32;3], cam_target: [f32;3], abs_up: [f32;3]) -> [[f32; 4]; 4] {
    let cam_dir = vec3_norm(vec3_sub(cam_pos, cam_target));
    let cam_right = vec3_norm(vec3_cross(abs_up, cam_dir));
    let cam_up = vec3_cross(cam_dir, cam_right);

    return mat4_mult_f32([
        [cam_right[0],  cam_right[1],   cam_right[2],   0.0     ],
        [cam_up[0],     cam_up[1],      cam_up[2],      0.0     ],
        [cam_dir[0],    cam_dir[1],     cam_dir[2],     0.0     ],
        [0.0,           0.0,            0.0,            1.0f32  ]
    ], [
        [1.0, 0.0, 0.0, -cam_pos[0] ],
        [0.0, 1.0, 0.0, -cam_pos[1] ],
        [0.0, 0.0, 1.0, -cam_pos[2] ],
        [0.0, 0.0, 0.0, 1.0f32      ]
    ])
}

fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
    let theta = fov * 2.0 * PI/360.0;

    let n = near;
    let f = far;

    let l = -n * (theta/2.0).tan();
    let b = l / aspect;

    let r = -l;
    let t = -b;

    return [
        [2.0*n/(r - l), 0.0,            (r+l)/(r-l),    0.0             ],
        [0.0,           2.0*n/(t-b),    (t+b)/(t-b),    0.0             ],
        [0.0,           0.0,            -(f+n)/(f-n),   -2.0*f*n/(f-n)  ],
        [0.0,           0.0,            -1.0,           0.0f32          ]
    ];
}

fn invert_mat4(mat: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    return [
        [mat[0][0], mat[1][0], mat[2][0], mat[3][0]],
        [mat[0][1], mat[1][1], mat[2][1], mat[3][1]],
        [mat[0][2], mat[1][2], mat[2][2], mat[3][2]],
        [mat[0][3], mat[1][3], mat[2][3], mat[3][3]]
    ];
}

fn rotate(mat: [[f32; 4]; 4], pitch: f32, roll: f32, head: f32) -> [[f32; 4]; 4] {
    let p = pitch * 2.0 * PI/360.0;
    let r = roll * 2.0 * PI/360.0;
    let h = head * 2.0 * PI/360.0;

    let sp = p.sin();
    let sr = r.sin();
    let sh = h.sin();

    let cp = p.cos();
    let cr = r.cos();
    let ch = h.cos();

    let rotate = [
        [(cr * ch) - (sr * sp * sh),    -sr * cp,   (cr * sh) + (sr * sp * ch), 0.0],
        [(sr * ch) + (cr * sp * sh),    cr * cp,    (sr * sh) - (cr * sp * ch), 0.0],
        [-cp * sh,                      sp,         cp * ch,                    0.0],
        [0.0,                           0.0,        0.0,                        1.0]
    ];

    return mat4_mult_f32(rotate, mat);
}