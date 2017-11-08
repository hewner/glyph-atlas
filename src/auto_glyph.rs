
use glyph_atlas::AtlasEntry;

pub struct AutoGlyph {
    index:u32,
    raw_r:f32,
    raw_c:f32,
    r_adjust:f32,
    c_adjust:f32,
    width:f32, //as fraction of cell
    height:f32, //as fraction of cell
    tex_left:f32, 
    tex_top:f32,
    tex_right:f32,
    tex_bottom:f32,
    //velocity
    end_r:f32,
    end_c:f32,
    start_t:f32,
    end_t:f32
}

enum Corner {
    UpperLeft = 0,
    UpperRight,
    LowerLeft,
    LowerRight
}


#[derive(Copy, Clone)]
pub struct AutoGlyphV {
    index : u32,
    bg : u32,
    corner : u32,
    pos: [f32; 2],
    tex_o: [f32; 2],
    seed: f32,
    end_pos: [f32; 2],
    start_t: f32,
    end_t: f32
}

implement_vertex!(AutoGlyphV, pos, end_pos, tex_o, seed, bg, start_t, end_t, corner, index);

pub type VertexList = Vec<AutoGlyphV>;


impl AutoGlyph {
    pub fn new(entry:&AtlasEntry, r:f32, c:f32, start_t:f32, end_t:f32) -> AutoGlyph {        

        AutoGlyph {
                    index: entry.index,
                    r_adjust: 1. - entry.top(),
                    c_adjust: entry.left(),
                    raw_r: r,
                    raw_c: c,
                    tex_left: entry.tex_left(),
                    tex_right: entry.tex_right(),
                    tex_top: entry.tex_top(),
                    tex_bottom: entry.tex_bottom(),
                    width: entry.width(),
                    height: entry.height(),
                    end_r:r,
                    end_c:c,
                    start_t:start_t,
                    end_t:end_t
        }
    }

    pub fn set_end(&mut self,r:f32,c:f32) {
        self.end_r = r;
        self.end_c = c;
    }

    pub fn add_to_vertex_list(&self, list:&mut  VertexList) {
        use self::Corner::*;
        let seed = 7.;
        let l = self.tex_left;
        let r = self.tex_right;
        let t = self.tex_top;
        let b = self.tex_bottom;
        let w = self.width;
        let h = self.height;
        list.push(AutoGlyphV::from_ag(self, self.r_adjust, self.c_adjust, 0, [l,b], UpperLeft));
        list.push(AutoGlyphV::from_ag(self, self.r_adjust + h, self.c_adjust, 0, [l,t], LowerLeft));
        list.push(AutoGlyphV::from_ag(self, self.r_adjust, self.c_adjust + w, 0, [r,b], UpperRight));
        list.push(AutoGlyphV::from_ag(self, self.r_adjust + h, self.c_adjust, 0, [l,t], LowerLeft));
        list.push(AutoGlyphV::from_ag(self, self.r_adjust, self.c_adjust + w, 0, [r,b], UpperRight));
        list.push(AutoGlyphV::from_ag(self, self.r_adjust + h, self.c_adjust +w, 0, [r,t], LowerRight));

    }

    pub fn add_background_to_vertex_list(&self, list:&mut  VertexList) {
        use self::Corner::*;
        let seed = 7.;
        let w =   self.width.ceil(); //makes the bg right for extra with characters
        let h = 1.;

        list.push(AutoGlyphV::from_ag(self, 0.,0., 1, [0.,0.], UpperLeft));
        list.push(AutoGlyphV::from_ag(self, h,0., 1, [0.,0.], LowerLeft));
        list.push(AutoGlyphV::from_ag(self, 0.,w, 1, [0.,0.], UpperRight));
        list.push(AutoGlyphV::from_ag(self, h,0., 1, [0.,0.], LowerLeft));
        list.push(AutoGlyphV::from_ag(self, 0.,w, 1, [0.,0.], UpperRight));
        list.push(AutoGlyphV::from_ag(self, h, w, 1, [0.,0.], LowerRight));        

    }

}

impl AutoGlyphV {
    fn from_ag(ag:&AutoGlyph, r_adjust:f32, c_adjust:f32, bg:u32, tex_o:[f32; 2], corner:Corner) -> AutoGlyphV {
        AutoGlyphV {
                     index : ag.index,
                     pos : [c_adjust+ag.raw_c, r_adjust+ag.raw_r],
                     corner : corner as u32,
                     end_pos : [c_adjust+ag.end_c, r_adjust+ag.end_r],
                     bg : bg,
                     tex_o: tex_o,
                     seed : 7.,
                     start_t: ag.start_t,
                     end_t: ag.end_t
        }
    }
}
