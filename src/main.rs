#[macro_use]
extern crate glium;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct V {
        pos: [f32; 2],
    }

    implement_vertex!(V, pos);

    let mut boxes = Vec::with_capacity(100);
    for i in 0..3 {
        let i = i as f32;
        let w = 0.25;
        boxes.push(V { pos : [i*w,0.] });
        boxes.push(V { pos : [(i+1.)*w,0.] });
        boxes.push(V { pos : [0.,0.5] });
        boxes.push(V { pos : [(i+1.)*w,0.] });
        boxes.push(V { pos : [0.,0.5] });
        boxes.push(V { pos : [(i+1.)*w,0.5] });
    }
    

    let vertex_buffer = glium::VertexBuffer::new(&display, &boxes).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 pos;

        uniform float t;

        void main() {
            gl_Position = vec4(pos[0] - t, pos[1], 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        let t: f32 = 0.0;
        let uniforms = uniform! { t: t };
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

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

