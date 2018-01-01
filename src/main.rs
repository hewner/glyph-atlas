#[macro_use]
extern crate glium;
extern crate image;
extern crate font;
extern crate fnv;
extern crate rand;

use font::{Rasterize, FontDesc};
use std::{time};
use std::fs::File;
use std::io::Read;
use std::env;

mod auto_glyph;
mod glyph_atlas;

use auto_glyph::*;
use glyph_atlas::*;

fn file_as_string(filename:&str)->String {
    let result = File::open(filename);
    if result.is_err() {
        let path = env::current_dir().unwrap();
        println!("filename: {} current path: {}",
                filename,
                path.display());
    }
    let mut file = result.unwrap();  // I still want to panic on error
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn main() {

    //TODO: Allow character loads
    
    use glium::{glutin, Surface};

    let window_width = 1900.;
    let window_height = 1000.;

    ///**************FONTS
    let rasterizer = font::Rasterizer::new(108., 110., 1., false).unwrap();

    let font = FontDesc::new(String::from("monospace"),
                             font::Style::Description {slant: font::Slant::Normal, weight: font::Weight::Normal});
    let size = font::Size::new(12.);

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
    for r in 0..num_rows {
        for c in 0..num_cols {
            let atlas_entry;
            let letter = 'A' as u8 + ((c + r) % 58) as u8;
            atlas_entry = atlas.get_entry(&display, letter as char);

            let r_mod = rand::random::<f32>() * 200. - 100.;
            let c_mod = rand::random::<f32>() * 200. - 100.;
         
            let mut pos = TimeVaryingVal::new(r as f32,c as f32,0.,0.);
            //pos.set_end(r as f32 + r_mod,c as f32 + c_mod,0.,0.);
            //pos.set_chs_params(0.4,-0.2);
            //pos.make_linear();

            let mut fg = TimeVaryingVal::new(1.,1.,1.,1.0);
            //fg.set_end(0.,0.3,0.,1.0);
            //fg.set_chs_params(0.4,-0.2);

            let mut bg = TimeVaryingVal::new(0.,0.,0.,1.0);
            //bg.set_end(0.5,0.,0.5,1.0);
            //bg.set_chs_params(0.4,-0.2);

            let mut ag = AutoGlyph::new(&atlas_entry, pos, fg, bg, 0., 10.);
            ag.set_randomizations(45);
            ag.add_to_vertex_list(&mut boxes);
         
        }
    }


    
    // plot (x^3-2x^2+x)*.5 + (-2x^3+3x^2) + (x^3 - x^2)*-.6 from x=0 to 1
    // https://en.wikipedia.org/wiki/Cubic_Hermite_spline


/*
    let atlas_entry = atlas.get_entry(&display, 'の');
    let mut ag = AutoGlyph::new(&atlas_entry, 1., 1., 0., 100.);

    ag.add_background_to_vertex_list(&mut boxes);
    ag.add_to_vertex_list(&mut boxes);


    let atlas_entry = atlas.get_entry(&display, 'q');
    let mut ag = AutoGlyph::new(&atlas_entry, 5., 5., 0., 100.);

    ag.add_background_to_vertex_list(&mut boxes);
    ag.add_to_vertex_list(&mut boxes);


    let atlas_entry = atlas.get_entry(&display, 'Q');
    let mut ag = AutoGlyph::new(&atlas_entry, 5., 6., 0., 100.);

    ag.add_background_to_vertex_list(&mut boxes);
    ag.add_to_vertex_list(&mut boxes);


    let atlas_entry = atlas.get_entry(&display, ')');
    let mut ag = AutoGlyph::new(&atlas_entry, 5., 7., 0., 100.);

    ag.add_background_to_vertex_list(&mut boxes);
    ag.add_to_vertex_list(&mut boxes);
*/
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &boxes).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);


    let program = glium::Program::from_source(&display,
                                              &file_as_string("shaders/vertex.glsl"),
                                              &file_as_string("shaders/fragment.glsl"),
                                              Some(&file_as_string("shaders/geometry.glsl"))).unwrap();


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
                                  tex : atlas.texture(),
                                  attributes : atlas.attribute_texture(),
                                  max_index : (atlas.size() - 1) as i32
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

