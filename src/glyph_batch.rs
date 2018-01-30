use auto_glyph::{AutoGlyph};
use glyph_atlas::GlyphAtlas;
use glium::backend::Facade;
use glium::{self, VertexBuffer};
use rand;

#[derive(Copy, Clone)]
pub struct AutoGlyphV {
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
    pub fn from_ag( display:&glium::backend::Facade,
                    atlas:&mut GlyphAtlas,
                    ag:&AutoGlyph) -> AutoGlyphV {

        let atlas_entry = atlas.get_entry(display, ag.glyph);
        
        let mut special = 0;
        let mut special_data = [[0.; 4]; 4];
        let mut num_specials = 0;

        if ag.randomizations.is_variable() {
            special = 4;
            special_data = ag.randomizations.data();
            num_specials += 1;
        }
        
        if ag.pos.is_variable() {
            special = 3;
            special_data = ag.pos.data();
            num_specials += 1;
        }

        if ag.fg.is_variable() {
            special = 2;
            special_data = ag.fg.data();
            num_specials += 1;
        }

        if ag.bg.is_variable() {
            special = 1;
            special_data = ag.bg.data();
            num_specials += 1;
        }

        if num_specials > 1 {
            println!("More than 1 time varying not supported!");
        }

        AutoGlyphV {
            index : atlas_entry.index,
            pos : ag.pos.data()[0],
            bg : ag.bg.data()[0],
            fg : ag.fg.data()[0],
            seed : rand::random::<f32>(),
            randomizations : ag.randomizations.data()[0][0] as u32,
            start_t: ag.start_t,
            end_t: ag.end_t,
            special : special,
            special_data: special_data,
        }
    }
}

 

pub struct GlyphBatch {
    buffer : VertexBuffer<AutoGlyphV>,
    latest_end : f64
}

impl GlyphBatch {

    pub fn new(display:&Facade,
               atlas:&mut GlyphAtlas,
               unconverted_glyphs:&[AutoGlyph]) -> GlyphBatch {
        let mut latest_end:f64 = 0.;
        let glyphs:Vec<AutoGlyphV>;
        {
            let iter = unconverted_glyphs.iter().map(|g| {
                if latest_end <  g.end_t { latest_end = g.end_t }
                AutoGlyphV::from_ag(display, atlas, g)
            });
            glyphs = iter.collect();
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
