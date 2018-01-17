#![feature(rustc_private)]

extern crate rand;
extern crate rustc_serialize;

use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use rustc_serialize::json;


mod auto_glyph;
use auto_glyph::*;

fn generate_batch(num_rows: u32,
                  num_cols: u32,
                  start_t: f32) -> Vec<AutoGlyph> {
   let mut boxes = Vec::new();
    for r in 0..num_rows {
        for c in 0..num_cols {
            let letter = 'A' as u8 + ((c + r) % 58) as u8;

            let r_mod = rand::random::<f32>();
            if r_mod > 0.1 { continue; } 
         
            let mut pos = TimeVaryingVal::new(r as f32,c as f32,0.,0.);

            let mut fg = TimeVaryingVal::new(1.,1.,1.,1.0);

            let mut bg = TimeVaryingVal::new(0.,0.,0.,1.0);

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
    println!("Hello");
    let mut stream = UnixStream::connect("/tmp/sock2").unwrap();
    let result = generate_batch(30,30,0.);
    let encoded = json::encode(&result).unwrap();
    stream.write_all(&encoded.into_bytes()).unwrap();

}
