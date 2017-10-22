#[macro_use]
extern crate glium;
extern crate image;
extern crate font;
extern crate fnv;
extern crate rand;

use font::{Rasterize, FontDesc};
use std::{time};
use rand::Rng;

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
    let size = font::Size::new(40.);

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
    let mut rng = rand::thread_rng();
    // for r in 0..num_rows {
    //     for c in 0..num_cols {
    //         let atlas_entry;
    //         let letter = 'A' as u8 + ((c + r) % 58) as u8;
    //         atlas_entry = atlas.get_entry(&display, letter as char);

            
    //         let r = r as f32;
    //         let c = c as f32; 
    //         let mut ag = AutoGlyph::new(&atlas_entry, r, c);
    //         let r_vel = rng.gen::<f32>()*2. - 1.;
    //         let c_vel = rng.gen::<f32>()*2. - 1.;
    //         ag.set_vel(&r_vel,&c_vel);
    //         ag.add_background_to_vertex_list(&mut boxes);
    //         ag.add_to_vertex_list(&mut boxes);
            
    //     }
    // }


    
    // plot (x^3-2x^2+x)*.5 + (-2x^3+3x^2) + (x^3 - x^2)*-.6 from x=0 to 1
    // https://en.wikipedia.org/wiki/Cubic_Hermite_spline

    let atlas_entry = atlas.get_entry(&display, 'の');
    let mut ag = AutoGlyph::new(&atlas_entry, 5., 5., 0., 5.);
    ag.set_end(0.,0.);
    //let r_vel = rng.gen::<f32>()*2. - 1.;
    //let c_vel = rng.gen::<f32>()*2. - 1.;

    ag.add_background_to_vertex_list(&mut boxes);
    ag.add_to_vertex_list(&mut boxes);

    
    let vertex_buffer = glium::VertexBuffer::new(&display, &boxes).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 pos;
        in vec2 end_pos;
        in vec2 tex_o;
        in float seed;
        in float r_vel;
        in float c_vel;
        in float start_t;
        in float end_t;
        uniform float t;
        in int bg;
        out float fseed;
        out vec2 ftex_o;
        flat out int fbg;
        uniform mat4 matrix;

        const int DONT_DRAW = -1;

        void main() {
            fseed = seed;
            ftex_o = tex_o;
            fbg = bg;
            if(start_t >= t || end_t < t) {
               fbg = DONT_DRAW;
               gl_Position = vec4(0.,0.,0.,0.);
            } else {

               float progress = (t - start_t)/(end_t - start_t); // a percent
               float r = pos[0]*(1 - progress) + end_pos[0]*progress; 
               float c = pos[1]*(1 - progress) + end_pos[1]*progress; 

               gl_Position = matrix * vec4(r, c, 0.0, 1.0);
            }
     
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in float fseed;
        flat in int fbg;
        in vec2 ftex_o;
        out vec4 color;
        uniform float t;

        const int DONT_DRAW = -1;
        uniform sampler2DArray tex;

        float rand(float fseed, float seed){
             return fract(sin(dot(vec2(fseed,seed),vec2(12.9898,78.233))) * 43758.5453);
        }

        void main() {
            if(fbg == DONT_DRAW) {
               discard;
            }
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

