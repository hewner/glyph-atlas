#![feature(rustc_private)]
#[macro_use]
extern crate glium;
extern crate image;
extern crate font;
extern crate fnv;
extern crate rand;
extern crate rustc_serialize;

use font::{Rasterize, FontDesc};
use std::{time, env, thread};
use std::fs::File;
use std::io::Read;
use std::sync::mpsc;

mod auto_glyph;
mod glyph_atlas;
mod glyph_batch;
    
use auto_glyph::*;
use glyph_atlas::*;
use glyph_batch::*;


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

fn generate_batch(num_rows: u32,
                  num_cols: u32,
                  start_t: f32) -> Vec<AutoGlyph> {
   let mut boxes = Vec::new();
    //let mut rng = rand::thread_rng();
    for r in 0..num_rows {
        for c in 0..num_cols {
            let letter = 'A' as u8 + ((c + r) % 58) as u8;
            //atlas_entry = atlas.get_entry(display, letter as char);

            let r_mod = rand::random::<f32>();
            if r_mod > 0.1 { continue; } 
         
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

            let mut ag = AutoGlyph::new(letter as char,
                                        pos,
                                        fg,
                                        bg,
                                        start_t,
                                        start_t + 10.);
            ag.set_nonlinear_randomizations(45, 0.4, -0.2);
            boxes.push(ag);
         
        }
    }

    boxes
}

fn main() {

    use glium::{glutin, Surface};

    let window_width = 1900.;
    let window_height = 1000.;

    let rasterizer = font::Rasterizer::new(108., 110., 1., false).unwrap();

    let font = FontDesc::new(String::from("monospace"),
                             font::Style::Description {slant: font::Slant::Normal, weight: font::Weight::Normal});
    let size = font::Size::new(12.);

    //    let foo_tex = glium::texture::RgbTexture2d::new(&display, foo).unwrap();

    
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


    let atlas_entry = atlas.get_entry(&display, 'Q');
/*    let mut pos = TimeVaryingVal::new(1.,1.,0.,0.);
    let mut fg = TimeVaryingVal::new(1.,1.,1.,1.0);
    let mut bg = TimeVaryingVal::new(0.,0.,0.,1.0);
    let mut ag = AutoGlyph::new(&atlas_entry, pos, fg, bg, 0., 10.);


    //ag.add_to_vertex_list(&mut boxes);
*/
    
    //let vertex_buffer = glium::VertexBuffer::new(&display, &boxes).unwrap();
    //let mut vertex_buffer = glium::VertexBuffer::empty_dynamic(&display, 1).unwrap();
    //{

    //    let mut map = vertex_buffer.map();
    //    map[0] = AutoGlyphV::from_ag(&ag);
    //    map.z();
    // }
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
    let mut batches:Vec<GlyphBatch> = Vec::new();


    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        use std::os::unix::net::UnixListener;
        use std::io::Read;
        use rustc_serialize::json;
        
        let listener = match UnixListener::bind("/tmp/sock2") {
            Ok(sock) => sock,
            Err(e) => {
                println!("Couldn't connect: {:?}", e);
                return
            }
        };

        println!("listening");
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    /* connection succeeded */
                    let mut response = String::new();
                    stream.read_to_string(&mut response).unwrap();
                    let loaded: Vec<AutoGlyph> = json::decode(&response).unwrap();
                    tx.send(loaded).unwrap();
                }
                Err(_) => {
                    /* connection failed */
                    break;
                }
            }
        }

        
//        let val = generate_batch(num_rows, num_cols, 0.);

    });


    while !closed {
        let mut target = display.draw();
        let now = time::Instant::now();
        let dur = now - start;
        let t:f32 = dur.as_secs() as f32 + dur.subsec_nanos() as f32 * 1e-9;
        {
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
            batches.retain ( |ref b| b.latest_end() > t );
            for batch in &batches {
                target.draw(batch.buffer(), &indices, &program, &uniforms,
                            &params).unwrap();
            }
        }
        target.finish().unwrap();

        let thread_result = rx.try_recv();
        match thread_result {
            Ok(data) => {
                let batch = GlyphBatch::new(&display,
                                            &mut atlas,
                                            &data);
                batches.push(batch);
            },
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // no new data - do nothing
            },
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                println!("Disconnected from data thread"); //should be a panic someday
            }
        }
        
        //println!("{}", 1./(now.elapsed().subsec_nanos() as f64 * 1e-9));
        //let ten_millis = time::Duration::from_millis(500);
        //thread::sleep(ten_millis);
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                glutin::Event::DeviceEvent { event, .. } => match event {
                    glutin::DeviceEvent::Key(input) => {
                        match input.virtual_keycode {
                            Some(glutin::VirtualKeyCode::Escape) => closed = true,
                            Some(glutin::VirtualKeyCode::A) => {
                                println!("YES!\n");
                                let vertexes = generate_batch(num_rows,
                                                              num_cols,
                                                              t);
                                let batch = GlyphBatch::new(&display,
                                                            &mut atlas,
                                                            &vertexes);
                                batches.push(batch);
                                },
                            _ => ()
                        }
                    },
                    _ => ()
                },
                _ => ()
            }
        });
    }
}

