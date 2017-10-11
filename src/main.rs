#[macro_use]
extern crate glium;
extern crate image;
extern crate font;
extern crate fnv;

use font::{Rasterize, FontDesc, GlyphKey};
use std::{thread, time};

fn main() {
    use glium::{glutin, Surface};

    let window_width = 1900;
    let window_height = 1000;

    ///**************FONTS
    let mut rasterizer = font::Rasterizer::new(108., 110., 1., false).unwrap();

    let font = FontDesc::new(String::from("monospace"),
                             font::Style::Description {slant: font::Slant::Normal, weight: font::Weight::Normal});
    let size = font::Size::new(8.);
    let regular = rasterizer.load_font(&font, size).unwrap();

    let mut glyph = rasterizer.get_glyph(&GlyphKey { font_key: regular, c: 'Q', size: size }).unwrap();

    glyph.buf.reverse();
    let foo = glium::texture::RawImage2d::from_raw_rgb(glyph.buf, (glyph.width as u32, glyph.height as u32));

    let char_width = glyph.width as u32; //7*2;
    let char_height = glyph.height as u32; //9*2;
    let num_cols = window_width/char_width;
    let num_rows = window_height/char_height;

    //    let foo_tex = glium::texture::RgbTexture2d::new(&display, foo).unwrap();
    ///***************END FONTS

    
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello world!")
        .with_dimensions(window_width, window_height);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct V {
        pos: [f32; 2],
        tex_o: [f32; 2],
        seed: f32
    }

    implement_vertex!(V, pos, tex_o, seed);
    let num_cells = num_rows*num_cols;
    let mut boxes = Vec::with_capacity(num_cells as usize);
    for r in 0..num_rows {
        for c in 0..num_cols {
            let r = r as f32;
            let c = c as f32;
            let seed = (r*num_cols as f32 + c) / num_cells as f32;
            boxes.push(V { pos : [c,r], tex_o: [0.,1.], seed : seed});
            boxes.push(V { pos : [c, r+1.], tex_o: [0.,0.], seed : seed });
            boxes.push(V { pos : [c+1.,r], tex_o: [1.,1.], seed : seed });
            boxes.push(V { pos : [c,r+1.], tex_o: [0.,0.], seed : seed });
            boxes.push(V { pos : [c+1.,r], tex_o: [1.,1.], seed : seed });
            boxes.push(V { pos : [c+1., r+1.], tex_o: [1.,0.], seed : seed });
        }
    }
    

    let vertex_buffer = glium::VertexBuffer::new(&display, &boxes).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 pos;
        in vec2 tex_o;
        in float seed;
        out float fseed;
        out vec2 ftex_o;
        uniform mat4 matrix;


        void main() {
            fseed = seed;
            ftex_o = tex_o;
            gl_Position = matrix * vec4(pos[0], pos[1], 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in float fseed;
        in vec2 ftex_o;
        out vec4 color;
        uniform float t;

        uniform sampler2DArray tex;

        float rand(float fseed, float seed){
             return fract(sin(dot(vec2(fseed,seed),vec2(12.9898,78.233))) * 43758.5453);
        }

        void main() {
            float r = rand(fseed, t);
            int letter = int(r * 52);
            float totalxOffset = (letter + ftex_o[0])/52.0;
            float g = rand(fseed, r);
            float b = rand(fseed, g);
            //color = vec4(r, g, b, 1.0);
            //color = texture(tex, vec3(totalxOffset, ftex_o[1], 1.));
            color = texture(tex, vec3(ftex_o[0], ftex_o[1], 0.)); 
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    use std::io::Cursor;
    let image = image::load(Cursor::new(&include_bytes!("../proggyclean.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let image2 = image::load(Cursor::new(&include_bytes!("../proggyclean2.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions2 = image2.dimensions();
    let image2 = glium::texture::RawImage2d::from_raw_rgba_reversed(&image2.into_raw(), image_dimensions2);

//    let textures = vec![image, image2];
    let textures = vec![foo];
    let texture = glium::texture::SrgbTexture2dArray::new(&display, textures).unwrap();
    
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
                                  matrix : matrix,
                                  tex : &texture
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

