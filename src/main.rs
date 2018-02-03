#![feature(rustc_private)]
#[macro_use]
extern crate glium;
extern crate image;
extern crate font;
extern crate fnv;
extern crate rand;

#[macro_use]
extern crate nom;

extern crate byteorder;

use font::{Rasterize, FontDesc};
use std::{env, thread};
use std::fs::File;
use std::io::{Read};
use std::sync::mpsc;

use std::os::unix::net::{UnixListener, UnixStream};


mod auto_glyph;
mod glyph_atlas;
mod glyph_batch;
mod glyph_builder;
mod effects;

use auto_glyph::*;
use glyph_atlas::*;
use glyph_batch::*;
use glyph_builder::GlyphBuilder;


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

    //need to start with at least one entry in the atlas or you have problems
    atlas.get_entry(&display, 'Q');
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

    let mut batches:Vec<GlyphBatch> = Vec::new();


    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        
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
                    let mut buffer = Vec::new();
                    //TODO: process like a stream
                    stream.read_to_end(&mut buffer).unwrap(); 

                    use nom::{be_f32, be_f64, be_u32};

                    named!(color<[f32; 4]>,
                           do_parse!(
                               r:be_f32 >>
                               g:be_f32 >>
                               b:be_f32 >>    
                               ([r,g,b,1.])
                           ));


                    named!(varying<TimeVaryingVal>,
                           do_parse!(
                               s1:be_f32 >>
                                   s2:be_f32 >>
                                   s3:be_f32 >>
                                   e1:be_f32 >>
                                   e2:be_f32 >>
                                   e3:be_f32 >>
                                   ({let mut val = TimeVaryingVal::new(s1,s2,s3,0.);
                                     val.set_end(e1,e2,e3,0.);
                                     val})
                           ));

                    named!(b<AutoGlyphV>,
                           do_parse!(
                               tag!("ba") >>
                                   glyph:be_u32 >>
                                   r:be_f32 >>
                                   c:be_f32 >>
                                   st:be_f64 >>
                                   et:be_f64 >>
                                   fg:color >>
                                   bg:color >>
                                   ({let converted = std::char::from_u32(glyph).unwrap();
                                     AutoGlyphV::basic(converted,r,c,st,et,fg,bg,0)})
                           ));

                    
                    named!(lr<AutoGlyphV>,
                           do_parse!(
                               tag!("lr") >>
                                   r:be_f32 >>
                                   c:be_f32 >>
                                   st:be_f64 >>
                                   et:be_f64 >>
                                   fg:color >>
                                   bg:color >>
                                   rnum:be_u32 >>
                                   (AutoGlyphV::basic('?',r,c,st,et,fg,bg,rnum))
                           ));


                    named!(bg<AutoGlyphV>,
                           do_parse!(
                               tag!("bg") >>
                                   glyph:be_u32 >>
                                   r:be_f32 >>
                                   c:be_f32 >>
                                   st:be_f64 >>
                                   et:be_f64 >>
                                   fg:color >>
                                   vary_val:varying >>
                                   ({  let converted = std::char::from_u32(glyph).unwrap();
                                       let mut g = AutoGlyphV::basic(converted,r,c,st,et,fg,[0.,0.,0.,0.],0);
                                     g.set_special(1,vary_val.data());
                                     g
                                   })
                           ));


                    named!(ag<AutoGlyphV>,
                           alt!(
                               b | lr | bg
                           ));

                    
                    named!(multi< Vec<AutoGlyphV> >, many0!(ag));
                    let (_, glyphs) = multi(&buffer).unwrap();

                    tx.send(glyphs).unwrap();
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

        let t = now_as_double();
        {
            let uniforms = uniform! { t: t,
                                      matrix : matrix,
                                      tex : atlas.texture(),
                                      attributes : atlas.attribute_texture(),
                                      max_index : (atlas.size() - 1) as i32
            };
        
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            let params = glium::DrawParameters {
                blend: glium::draw_parameters::Blend::alpha_blending(),
                .. Default::default()
            };
            batches.retain ( |ref b| b.latest_end() >= t );
            for batch in &batches {
                target.draw(batch.buffer(), &indices, &program, &uniforms,
                            &params).unwrap();
            }
        }
        target.finish().unwrap();

        let thread_result = rx.try_recv();

        //json parser was 1.0x seconds
        //cbor parser was 1.3x seconds
        //bincode parser was 0.5-0.6 seconds
        //raw reads is .05-.06
        match thread_result {
            Ok(data) => {
                //let dur = parseTimer.elapsed();
                let batch = GlyphBatch::new(&display,
                                            &mut atlas,
                                            data);
                batches.push(batch);
            },
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // no new data - do nothing
            },
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                println!("Disconnected from data thread"); //should be a panic someday
                std::process::exit(0);
            }
        }
        
        //println!("{}", 1./(now.elapsed().subsec_nanos() as f64 * 1e-9));
        //let ten_millis = time::Duration::from_millis(500);
        //thread::sleep(ten_millis);

        // we are cheating a bit by passing these along, but not much

        let dc = effects::DrawContext { num_rows : num_rows,
                                        num_cols : num_cols,
                                        now : now_as_double()
        };
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => ()
                },
                glutin::Event::DeviceEvent { event, .. } => match event {
                    glutin::DeviceEvent::Key(input) => {
                        if input.state == glutin::ElementState::Pressed {
                            match input.virtual_keycode {
                                Some(glutin::VirtualKeyCode::Escape) => closed = true,
                                Some(glutin::VirtualKeyCode::A) => {
                                    let mut stream = UnixStream::connect("/tmp/sock2").unwrap();
                                    let mut glyph = GlyphBuilder::new()
                                        .fg(0.,0.,0.)
                                        .bg(0.,1.,0.)
                                        .glyph('❤');

                                    let mut fade = GlyphBuilder::new()
                                        .fg(0.,0.,0.)
                                        .glyph('-');

                                    
                                    for c in 0..dc.num_cols {
                                        let step = 0.01;
                                        let move_on = dc.now + c as f64*step;
                                        let move_off = move_on + step;
                                        let fade_out = move_off + 30.*step;
                                        glyph = glyph
                                            .pos(0., c as f32)
                                            .start(move_on)
                                            .end(move_off);        
                                        glyph.send_basic(&mut stream);

                                        fade = fade
                                            .pos(0., c as f32)
                                            .start(move_off)
                                            .end(fade_out);

                                        fade.send_linear_bg([0.,0.3,0.],
                                                             [0.,0.,0.],
                                                             &mut stream);
                                    }
                                    
                                },
                            _ => ()
                            }
                        }
                    },
                    _ => ()
                },
                _ => ()
            }
        });
    }
    std::fs::remove_file("/tmp/sock2").unwrap();
}

