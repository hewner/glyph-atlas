use glyph_batch::{AutoGlyphV, PreInstallGlyphBatch};
use std::io::{Read};
use std;
use auto_glyph::TimeVaryingVal;

use nom::{be_f32, be_f64, be_u32};

named!(color<[f32; 4]>,
       do_parse!(r:be_f32 >>
                 g:be_f32 >>
                 b:be_f32 >>    
                 ([r,g,b,1.])
       ));


named!(varying<TimeVaryingVal>,
       do_parse!(s1:be_f32 >>
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
       do_parse!(tag!("ba") >>
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
       do_parse!(tag!("lr") >>
                 r:be_f32 >>
                 c:be_f32 >>
                 st:be_f64 >>
                 et:be_f64 >>
                 fg:color >>
                 bg:color >>
                 rnum:be_u32 >>
                 (AutoGlyphV::basic('?',r,c,st,et,fg,bg,rnum))
       ));


named!(utf8char<char>,
       map_opt!(be_u32, std::char::from_u32));

named!(bg<AutoGlyphV>,
       do_parse!(tag!("bg") >>
                 glyph:utf8char >>
                 r:be_f32 >>
                 c:be_f32 >>
                 st:be_f64 >>
                 et:be_f64 >>
                 fg:color >>
                 vary_val:varying >>
                 ({  let mut g = AutoGlyphV::basic(glyph,r,c,st,et,fg,[0.,0.,0.,0.],0);
                     g.set_special(1,vary_val.data());
                     g
                 })
       ));


named!(ag<AutoGlyphV>,
       alt!(
           b | lr | bg
       ));

named!(multi< Vec<AutoGlyphV> >, many0!(ag));

named!(batch<PreInstallGlyphBatch>,
       do_parse!(name:utf8char >>
                 glyphs:multi >>
                 ({if name == '\0' { PreInstallGlyphBatch::new(None, glyphs) }
                   else {PreInstallGlyphBatch::new(Some(name.to_string()), glyphs)}})
       ));

pub fn receive_glyphs<R>(mut stream:R) -> PreInstallGlyphBatch where R:Read {
    /* connection succeeded */
    let mut buffer = Vec::new();
    //TODO: process like a stream
    stream.read_to_end(&mut buffer).unwrap(); 

    let (_, batch_result) = batch(&buffer).unwrap();
//    println!("{:?}", &glyphs);
    //PreInstallGlyphBatch::new(None, glyphs)
    batch_result
}
