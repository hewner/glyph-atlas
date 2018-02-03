use byteorder::{WriteBytesExt, BigEndian};
use std::io::{Write};

pub struct GlyphBuilder {
    glyph : Option<char>,
    start_t: Option<f64>,
    end_t: Option<f64>,
    pos: Option<[f32; 2]>,
    fg: Option<[f32; 3]>,
    bg: Option<[f32; 3]>,
}

trait Sendable {
    fn send<W:Write>(&self,writer:&mut W);
}

impl Sendable for Option<f64> {
    fn send<W:Write>(&self,writer:&mut W) {
        let val = self.unwrap();
        writer.write_f64::<BigEndian>(val).unwrap();
    }
}

impl Sendable for Option<[f32; 2]> {
    fn send<W:Write>(&self,writer:&mut W) {
        let val = self.unwrap();
        writer.write_f32::<BigEndian>(val[0]).unwrap();
        writer.write_f32::<BigEndian>(val[1]).unwrap();
    }
}

impl Sendable for Option<[f32; 3]> {
    fn send<W:Write>(&self,writer:&mut W) {
        let val = self.unwrap();
        writer.write_f32::<BigEndian>(val[0]).unwrap();
        writer.write_f32::<BigEndian>(val[1]).unwrap();
        writer.write_f32::<BigEndian>(val[2]).unwrap();
    }
}

impl Sendable for [u8] {
    fn send<W:Write>(&self,writer:&mut W) {
        writer.write(self).unwrap();
    }
}

impl Sendable for Option<char> {
    fn send<W:Write>(&self,writer:&mut W) {
        writer.write_u32::<BigEndian>(self.unwrap() as u32).unwrap();
    }
}



impl GlyphBuilder {
    pub fn new() -> GlyphBuilder {
        GlyphBuilder {
            glyph : None,
            start_t: None,
            end_t: None,
            pos: None,
            fg: None,
            bg: None,
        }       
    }

    pub fn glyph(mut self, g:char) -> GlyphBuilder {
        self.glyph = Some(g);
        self
    }

    pub fn start(mut self, start_t:f64) -> GlyphBuilder {
        self.start_t = Some(start_t);
        self
    }

    pub fn end(mut self, end_t:f64) -> GlyphBuilder {
        self.end_t = Some(end_t);
        self
    }

    pub fn pos(mut self, r:f32, c:f32) -> GlyphBuilder{
        self.pos = Some([r,c]);
        self
    }


    pub fn fg(mut self, r:f32, g:f32, b:f32) -> GlyphBuilder {
        self.fg = Some([r,g,b]);
        self
    }

    pub fn bg(mut self, r:f32, g:f32, b:f32) -> GlyphBuilder {
        self.bg = Some([r,g,b]);
        self
    }

    pub fn send_linear_random<W:Write>(&self,
                                       randomizations:u32,
                                       writer:&mut W) {
        b"lr".send(writer);
        self.pos.send(writer);
        self.start_t.send(writer);
        self.end_t.send(writer);
        self.fg.send(writer);
        self.bg.send(writer);
        writer.write_u32::<BigEndian>(randomizations).unwrap();

    }

    pub fn send_linear_bg<W:Write>(&self,
                                   start:[f32; 3],
                                   end:[f32; 3],
                                   writer:&mut W) {
        b"bg".send(writer);
        self.glyph.send(writer);
        self.pos.send(writer);
        self.start_t.send(writer);
        self.end_t.send(writer);
        self.fg.send(writer);
        Some(start).send(writer);
        Some(end).send(writer);

    }
    
    pub fn send_basic<W:Write>(&self,
                               writer:&mut W) {
        b"ba".send(writer);
        self.glyph.send(writer);
        self.pos.send(writer);
        self.start_t.send(writer);
        self.end_t.send(writer);
        self.fg.send(writer);
        self.bg.send(writer);
    }


}
