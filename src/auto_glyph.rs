use font::{Metrics, RasterizedGlyph};

pub struct AutoGlyph {
    r:f32,
    c:f32,
    w:f32, //width, expressed as fraction of 1 character
    h:f32 //height expressed as fraction of 1 character
}

#[derive(Copy, Clone)]
pub struct AutoGlyphV {
    pos: [f32; 2],
    tex_o: [f32; 2],
    seed: f32
}

implement_vertex!(AutoGlyphV, pos, tex_o, seed);

pub type VertexList = Vec<AutoGlyphV>;


impl AutoGlyph {
    pub fn new(metrics : &Metrics, rg: &RasterizedGlyph, r:f32, c:f32) -> AutoGlyph {
        let offset_w = rg.left as f32/metrics.average_advance as f32;
        let offset_h = rg.top as f32/metrics.line_height as f32;
        let newR = r + offset_h;
        let newC = c + offset_w;
        
        //println!("height {} top {}", rg.height, rg.top);
        AutoGlyph { r: newR, c: newC,
                    w: rg.width as f32/metrics.average_advance as f32,
                    h: rg.height as f32/metrics.line_height as f32}
    }

    pub fn addToVertexList(&self, list:&mut  VertexList) {
        let seed = 7.;
        let r = self.r;
        let c = self.c;
        let w = self.w;
        let h = self.h;
        list.push(AutoGlyphV { pos : [c,r], tex_o: [1.,1.], seed : seed});
        list.push(AutoGlyphV { pos : [c, r+h], tex_o: [1.,0.], seed : seed });
        list.push(AutoGlyphV { pos : [c+w,r], tex_o: [0.,1.], seed : seed });
        list.push(AutoGlyphV { pos : [c,r+h], tex_o: [1.,0.], seed : seed });
        list.push(AutoGlyphV { pos : [c+w,r], tex_o: [0.,1.], seed : seed });
        list.push(AutoGlyphV { pos : [c+w, r+h], tex_o: [0.,0.], seed : seed });

    }
}
