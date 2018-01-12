use auto_glyph::{AutoGlyph, AutoGlyphV};
use glium::backend::Facade;
use glium::VertexBuffer;

pub struct GlyphBatch {
    buffer : VertexBuffer<AutoGlyphV>,
    latest_end : f32
}

impl GlyphBatch {

    pub fn new(display:&Facade, unconverted_glyphs:&[AutoGlyph]) -> GlyphBatch {
        let mut latest_end:f32 = 0.;
        let glyphs:Vec<AutoGlyphV>;
        {
            let iter = unconverted_glyphs.iter().map(|g| {
                if latest_end <  g.end_t() { latest_end = g.end_t() }
                AutoGlyphV::from_ag(g)
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

    pub fn latest_end(&self) -> f32 {
        self.latest_end
    }
    
}
