#[macro_use]
extern crate glium;

use std::{thread, time};

fn main() {
    use glium::{glutin, Surface};

    let window_width = 1024;
    let window_height = 768;
    let char_width = 7;
    let char_height = 9;
    let num_cols = window_width/char_width;
    let num_rows = window_height/char_height;
    
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello world!")
        .with_dimensions(window_width, window_height);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct V {
        pos: [f32; 2],
        seed: f32
    }

    implement_vertex!(V, pos, seed);
    let num_cells = num_rows*num_cols;
    let mut boxes = Vec::with_capacity(num_cells as usize);
    for r in 0..num_rows {
        for c in 0..num_cols {
            let r = r as f32;
            let c = c as f32;
            let seed = (r*num_cols as f32 + c) / num_cells as f32;
            boxes.push(V { pos : [c,r], seed : seed});
            boxes.push(V { pos : [c, r+1.], seed : seed });
            boxes.push(V { pos : [c+1.,r], seed : seed });
            boxes.push(V { pos : [c,r+1.], seed : seed });
            boxes.push(V { pos : [c+1.,r], seed : seed });
            boxes.push(V { pos : [c+1., r+1.], seed : seed });
        }
    }
    

    let vertex_buffer = glium::VertexBuffer::new(&display, &boxes).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 pos;
        in float seed;
        out float fseed;
        uniform mat4 matrix;


        void main() {
            fseed = seed;
            gl_Position = matrix * vec4(pos[0], pos[1], 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in float fseed;
        out vec4 color;
        uniform float t;

        float rand(float fseed, float seed){
             return fract(sin(dot(vec2(fseed,seed),vec2(12.9898,78.233))) * 43758.5453);
        }

        void main() {
            float r = rand(fseed, t);
            float g = rand(fseed, r);
            float b = rand(fseed, g);
            color = vec4(r, g, b, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut closed = false;
    let mut t: f32 = 0.0;
    let matrix =  [
        [2.0/num_cols as f32, 0.0, 0.0, 0.0],
        [0.0, -2.0/num_rows as f32, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-1.0 , 1.0, 0.0, 1.0f32],
    ];
    
    while !closed {
        let now = time::Instant::now();
        let mut target = display.draw();
        t += 0.01;
        if t > 1. { t = 0.; }
        let uniforms = uniform! { t: t,
                                  matrix : matrix
        };
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        println!("{}", 1./(now.elapsed().subsec_nanos() as f64 * 1e-9));
        //let ten_millis = time::Duration::from_millis(500);
        //thread::sleep(ten_millis);
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                _ => (),
            }
        });
    }
}

