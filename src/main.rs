#[macro_use]
extern crate glium;
extern crate image;
extern crate font;
extern crate fnv;

use font::{Rasterize, FontDesc};
use std::{time};

mod auto_glyph;
mod glyph_atlas;

use auto_glyph::*;
use glyph_atlas::*;

fn main() {

    //TODO: Allow character loads
    
    use glium::{glutin, Surface};

    let window_width = 1900.;
    let window_height = 1000.;

    ///**************FONTS
    let rasterizer = font::Rasterizer::new(108., 110., 1., false).unwrap();

    let font = FontDesc::new(String::from("monospace"),
                             font::Style::Description {slant: font::Slant::Normal, weight: font::Weight::Normal});
    let size = font::Size::new(14.);

    //    let foo_tex = glium::texture::RgbTexture2d::new(&display, foo).unwrap();
    ///***************END FONTS

    
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello world!")
        .with_dimensions(window_width as u32, window_height as u32);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut atlas = GlyphAtlas::new(rasterizer, &font, size, &display);
    
    let char_width = atlas.char_width();
    let char_height = atlas.char_height();
    let num_cols = (window_width/char_width) as u32;
    let num_rows = (window_height/char_height) as u32;

    let num_cells = num_rows*num_cols;
    let mut boxes = VertexList::with_capacity(6*num_cells as usize);
    for r in 0..num_rows {
        for c in 0..num_cols {
            let atlas_entry;
            let letter = 'A' as u8 + ((c + r) % 58) as u8;
            atlas_entry = atlas.get_entry(&display, letter as char);

            
            let r = r as f32;
            let c = c as f32; 
            let ag = AutoGlyph::new(&atlas_entry, r, c);
            //if(r <= 0. && c == 2.) {
                ag.add_background_to_vertex_list(&mut boxes);
            //}

            ag.add_to_vertex_list(&mut boxes);
        }
    }
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &boxes).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 pos;
        in vec2 tex_o;
        in float seed;
        in int bg;
        out float fseed;
        out vec2 ftex_o;
        flat out int fbg;
        uniform mat4 matrix;


        void main() {
            fseed = seed;
            ftex_o = tex_o;
            fbg = bg;
     
            gl_Position = matrix * vec4(pos[0], pos[1], 0.0, 1.0);
     
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in float fseed;
        flat in int fbg;
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
            vec4 fg = vec4(1.,1.,1.,1.);
            vec4 bg = vec4(0.,0.0,0.,1.);
            if(fbg == 0) {
                vec4 alpha = texture(tex, vec3(ftex_o[0], ftex_o[1], 0.));
                color = fg*alpha + (1-alpha)*bg;
                color = vec4(fg.xyz, alpha.x);
            } else {
                color = bg;
            }
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let mut closed = false;

    let num_cols_f = window_width as f32/char_width as f32;
    let num_rows_f = window_height as f32/char_height as f32;

    let matrix =  [
        [2.0/num_cols_f as f32, 0.0, 0.0, 0.0],
        [0.0, -2.0/num_rows_f as f32, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-1.0 , 1.0, 0.0, 1.0f32],
    ];

    let start = time::Instant::now();
    while !closed {

        let mut target = display.draw();
        let now = time::Instant::now();
        let dur = now - start;
        let t:f32 = dur.as_secs() as f32 + dur.subsec_nanos() as f32 * 1e-9;
        let uniforms = uniform! { t: t,
                                  matrix : matrix,
                                  tex : atlas.texture()
        };
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let params = glium::DrawParameters {
            blend: glium::draw_parameters::Blend::alpha_blending(),
            .. Default::default()
        };
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &params).unwrap();
        target.finish().unwrap();

        //println!("{}", 1./(now.elapsed().subsec_nanos() as f64 * 1e-9));
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

