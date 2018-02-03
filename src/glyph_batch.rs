use glyph_atlas::GlyphAtlas;
use glium::backend::Facade;
use glium::{self, VertexBuffer};
use rand;

#[derive(Copy, Clone)]
pub struct AutoGlyphV {
    glyph : char,
    index : u32,
    randomizations : u32,
    seed: f32,
    pos: [f32; 4],
    start_t: f64,
    end_t: f64,
    fg: [f32; 4],
    bg: [f32; 4],
    special: u32,
    special_data: [[f32; 4]; 4],
}

implement_vertex!(AutoGlyphV,
                  pos,
                  seed,
                  bg,
                  start_t,
                  end_t,
                  index,
                  fg,
                  special,
                  special_data,
                  randomizations);


impl AutoGlyphV {

    pub fn basic(g: char,
                 r:f32,
                 c:f32,
                 st:f64,
                 et:f64,
                 fg:[f32; 4],
                 bg:[f32; 4],
                 randomizations: u32
    ) -> AutoGlyphV{
        
        AutoGlyphV {
            glyph : g,
            index : 0,
            pos : [r,c,0.,0.],
            bg :  bg,
            fg : fg,
            seed : rand::random::<f32>(),
            randomizations : randomizations,
            start_t: st,
            end_t: et,
            special : 0,
            special_data: [[0.; 4]; 4],
        }
    }

    pub fn set_special(&mut self,
                       special: u32,
                       special_data: [[f32; 4]; 4]) {
        self.special = special;
        self.special_data = special_data;
    }

    pub fn make_tranfer_ready(&mut self,
                       display:&glium::backend::Facade,
                       atlas:&mut GlyphAtlas) {
        let atlas_entry = atlas.get_entry(display, self.glyph);
        self.index = atlas_entry.index;
    }
}

 

pub struct GlyphBatch {
    buffer : VertexBuffer<AutoGlyphV>,
    latest_end : f64
}

impl GlyphBatch {

    pub fn new(display:&Facade,
               atlas:&mut GlyphAtlas,
               mut glyphs:Vec<AutoGlyphV>) -> GlyphBatch {
        let mut latest_end:f64 = 0.;
        for g in &mut glyphs {
                if latest_end <  g.end_t { latest_end = g.end_t }
                g.make_tranfer_ready(display, atlas);
        }
        let buffer = VertexBuffer::new(display, &glyphs).unwrap();
        GlyphBatch { buffer : buffer,
                     latest_end : latest_end
        }
    }

    pub fn buffer(&self) -> &VertexBuffer<AutoGlyphV> {
        &self.buffer
    }

    pub fn latest_end(&self) -> f64 {
        self.latest_end
    }
    
}
