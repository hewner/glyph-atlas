use byteorder::{WriteBytesExt, BigEndian};
use std::io::{Write, Result};
use std::os::unix::net::UnixStream;
use std::time;

pub struct GlyphSender {
    glyph : Option<char>,
    start_t: Option<f64>,
    end_t: Option<f64>,
    pos: Option<[f32; 2]>,
    fg: Option<[f32; 3]>,
    bg: Option<[f32; 3]>,
    now : f64
}

pub fn now_as_double() -> f64 {
    let now = time::SystemTime::now();
    let dur = now.duration_since(time::UNIX_EPOCH).unwrap();
    dur.as_secs() as f64 + dur.subsec_nanos() as f64 * 1e-9
}


pub fn start_batch() -> Result<UnixStream>{
    start_batch_with_name('\0')
}

pub fn start_batch_with_name(name:char) -> Result<UnixStream>{
    let mut result = UnixStream::connect("/tmp/sock2") ?;
    result.write_u32::<BigEndian>(name as u32)?;
    Ok(result)
}

trait Sendable {
    fn send<W:Write>(&self,writer:&mut W) -> Result<()>;
}

impl Sendable for Option<f64> {
    fn send<W:Write>(&self,writer:&mut W) -> Result<()>{
        let val = self.unwrap();
        writer.write_f64::<BigEndian>(val)
    }
}

impl Sendable for Option<[f32; 2]> {
    fn send<W:Write>(&self,writer:&mut W) -> Result<()> {
        let val = self.unwrap();
        writer.write_f32::<BigEndian>(val[0])?;
        writer.write_f32::<BigEndian>(val[1])
    }
}

impl Sendable for Option<[f32; 3]> {
    fn send<W:Write>(&self,writer:&mut W) -> Result<()> {
        let val = self.unwrap();
        writer.write_f32::<BigEndian>(val[0])?;
        writer.write_f32::<BigEndian>(val[1])?;
        writer.write_f32::<BigEndian>(val[2])
    }
}

impl Sendable for [u8] {
    fn send<W:Write>(&self,writer:&mut W) -> Result<()>{
        writer.write(self)?;
        Ok(())
    }
}

impl Sendable for Option<char> {
    fn send<W:Write>(&self,writer:&mut W)  -> Result<()>{
        writer.write_u32::<BigEndian>(self.unwrap() as u32)
    }
}



impl GlyphSender {
    pub fn new() -> GlyphSender {
        GlyphSender {
            glyph : None,
            start_t: None,
            end_t: None,
            pos: None,
            fg: None,
            bg: None,
            now : now_as_double()
        }       
    }

    pub fn glyph(mut self, g:char) -> GlyphSender {
        self.glyph = Some(g);
        self
    }

    pub fn start(mut self, start_t:f64) -> GlyphSender {
        self.start_t = Some(start_t);
        self
    }

    pub fn end(mut self, end_t:f64) -> GlyphSender {
        self.end_t = Some(end_t);
        self
    }

    pub fn start_rel(self, adjust:f64) -> GlyphSender {
        let now = self.now;
        self.start(now + adjust)
    }

    pub fn end_rel(self, adjust:f64) -> GlyphSender {
        let now = self.now;
        self.end(now + adjust)
    }

    
    pub fn pos(mut self, r:f32, c:f32) -> GlyphSender{
        self.pos = Some([r,c]);
        self
    }


    pub fn fg(mut self, r:f32, g:f32, b:f32) -> GlyphSender {
        self.fg = Some([r,g,b]);
        self
    }

    pub fn bg(mut self, r:f32, g:f32, b:f32) -> GlyphSender {
        self.bg = Some([r,g,b]);
        self
    }

    pub fn send_linear_random<W:Write>(&self,
                                       randomizations:u32,
                                       writer:&mut W) -> Result<()>{
        b"lr".send(writer)?;
        self.pos.send(writer)?;
        self.start_t.send(writer)?;
        self.end_t.send(writer)?;
        self.fg.send(writer)?;
        self.bg.send(writer)?;
        writer.write_u32::<BigEndian>(randomizations)?;
        Ok(())

    }

    pub fn send_linear_bg<W:Write>(&self,
                                   start:[f32; 3],
                                   end:[f32; 3],
                                   writer:&mut W) -> Result<()>{
        b"bg".send(writer)?;
        self.glyph.send(writer)?;
        self.pos.send(writer)?;
        self.start_t.send(writer)?;
        self.end_t.send(writer)?;
        self.fg.send(writer)?;
        Some(start).send(writer)?;
        Some(end).send(writer)?;
        Ok(())

    }
    
    pub fn send_basic<W:Write>(&self,
                               writer:&mut W) -> Result<()>{
        b"ba".send(writer)?;
        self.glyph.send(writer)?;
        self.pos.send(writer)?;
        self.start_t.send(writer)?;
        self.end_t.send(writer)?;
        self.fg.send(writer)?;
        self.bg.send(writer)?;
        Ok(())
    }


}
