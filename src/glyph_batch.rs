use auto_glyph::{AutoGlyph, AutoGlyphV};
use glium::backend::Facade;
use glium::VertexBuffer;

pub struct GlyphBatch {
    buffer : VertexBuffer<AutoGlyphV>
}

impl GlyphBatch {

    pub fn new(display:&Facade, unconverted_glyphs:&[AutoGlyph]) -> GlyphBatch {
        let iter = unconverted_glyphs.iter().map(|g| AutoGlyphV::from_ag(g));
        let glyphs:Vec<AutoGlyphV> = iter.collect();
        let buffer = VertexBuffer::new(display, &glyphs).unwrap();
        GlyphBatch { buffer : buffer }
    }

    pub fn buffer(&self) -> &VertexBuffer<AutoGlyphV> {
        &self.buffer
    }
    
}
