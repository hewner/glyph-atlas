use font::{Metrics, RasterizedGlyph};
use glyph_atlas::AtlasEntry;

pub struct AutoGlyph {
    r:f32,
    c:f32,
    tex_left:f32, 
    tex_top:f32,
    tex_right:f32,
    tex_bottom:f32
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
    // pub fn new(metrics : &Metrics, rg: &RasterizedGlyph, r:f32, c:f32) -> AutoGlyph {
    //     let offset_w = rg.left as f32/metrics.average_advance as f32;
    //     let offset_h = rg.top as f32/metrics.line_height as f32;
    //     let newR = r + offset_h;
    //     let newC = c + offset_w;
        
    //     //println!("height {} top {}", rg.height, rg.top);
    //     AutoGlyph { r: newR, c: newC,
    //                 w: rg.width as f32/metrics.average_advance as f32,
    //                 h: rg.height as f32/metrics.line_height as f32}
    // }

    pub fn new2(entry:&AtlasEntry, r:f32, c:f32) -> AutoGlyph {        
        //println!("height {} top {}", rg.height, rg.top);
        AutoGlyph { r: r, c: c,
                    tex_left: entry.tex_left(),
                    tex_right: entry.tex_right(),
                    tex_top: entry.tex_top(),
                    tex_bottom: entry.tex_bottom()
        }
    }

    
    pub fn addToVertexList(&self, list:&mut  VertexList) {
        let seed = 7.;
        let row = self.r;
        let col = self.c;
        let l = self.tex_left;
        let r = self.tex_right;
        let t = self.tex_top;
        let b = self.tex_bottom;
        let w = 1.; //self.w;
        let h = 1.; //self.h;
        list.push(AutoGlyphV { pos : [col,row], tex_o: [l,t], seed : seed});
        list.push(AutoGlyphV { pos : [col, row+h], tex_o: [l,b], seed : seed });
        list.push(AutoGlyphV { pos : [col+w,row], tex_o: [r,t], seed : seed });
        list.push(AutoGlyphV { pos : [col,row+h], tex_o: [l,b], seed : seed });
        list.push(AutoGlyphV { pos : [col+w,row], tex_o: [r,t], seed : seed });
        list.push(AutoGlyphV { pos : [col+w, row+h], tex_o: [r,b], seed : seed });

    }
}
