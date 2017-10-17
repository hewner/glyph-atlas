use font::{Metrics, RasterizedGlyph};
use glyph_atlas::AtlasEntry;

pub struct AutoGlyph {
    r:f32,
    c:f32,
    width:f32, //as fraction of cell
    height:f32, //as fraction of cell
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
    pub fn new2(entry:&AtlasEntry, r:f32, c:f32) -> AutoGlyph {        

        AutoGlyph { r: r + entry.descent() + 1. - entry.top()
                    , c: c + entry.left(),
                    tex_left: entry.tex_left(),
                    tex_right: entry.tex_right(),
                    tex_top: entry.tex_top(),
                    tex_bottom: entry.tex_bottom(),
                    width: entry.width(),
                    height: entry.height()
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
        let w = self.width;
        let h = self.height;
        list.push(AutoGlyphV { pos : [col,row], tex_o: [l,t], seed : seed});
        list.push(AutoGlyphV { pos : [col, row+h], tex_o: [l,b], seed : seed });
        list.push(AutoGlyphV { pos : [col+w,row], tex_o: [r,t], seed : seed });
        list.push(AutoGlyphV { pos : [col,row+h], tex_o: [l,b], seed : seed });
        list.push(AutoGlyphV { pos : [col+w,row], tex_o: [r,t], seed : seed });
        list.push(AutoGlyphV { pos : [col+w, row+h], tex_o: [r,b], seed : seed });

    }
}
