
use glyph_atlas::AtlasEntry;

pub struct AutoGlyph {
    index:u32,
    raw_r:f32,
    raw_c:f32,
    r_adjust:f32,
    c_adjust:f32,
    width:f32, //as fraction of cell
    height:f32, //as fraction of cell
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
    seed: f32,
    end_pos: [f32; 2],
    start_t: f32,
    end_t: f32
}

implement_vertex!(AutoGlyphV, pos, end_pos, seed, bg, start_t, end_t, corner, index);

pub type VertexList = Vec<AutoGlyphV>;


impl AutoGlyph {
    pub fn new(entry:&AtlasEntry, r:f32, c:f32, start_t:f32, end_t:f32) -> AutoGlyph {        

        AutoGlyph {
                    index: entry.index,
                    r_adjust: 1. - entry.top(),
                    c_adjust: entry.left(),
                    raw_r: r,
                    raw_c: c,
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
        let w = self.width;
        let h = self.height;
        list.push(AutoGlyphV::from_ag(self, self.r_adjust, self.c_adjust, 0, UpperLeft));
        list.push(AutoGlyphV::from_ag(self, self.r_adjust + h, self.c_adjust, 0, LowerLeft));
        list.push(AutoGlyphV::from_ag(self, self.r_adjust, self.c_adjust + w, 0, UpperRight));
        list.push(AutoGlyphV::from_ag(self, self.r_adjust + h, self.c_adjust, 0, LowerLeft));
        list.push(AutoGlyphV::from_ag(self, self.r_adjust, self.c_adjust + w, 0, UpperRight));
        list.push(AutoGlyphV::from_ag(self, self.r_adjust + h, self.c_adjust +w, 0, LowerRight));

    }

    pub fn add_background_to_vertex_list(&self, list:&mut  VertexList) {
        use self::Corner::*;
        let seed = 7.;
        let w =   self.width.ceil(); //makes the bg right for extra with characters
        let h = 1.;

        list.push(AutoGlyphV::from_ag(self, 0.,0., 1, UpperLeft));
        list.push(AutoGlyphV::from_ag(self, h,0., 1, LowerLeft));
        list.push(AutoGlyphV::from_ag(self, 0.,w, 1, UpperRight));
        list.push(AutoGlyphV::from_ag(self, h,0., 1, LowerLeft));
        list.push(AutoGlyphV::from_ag(self, 0.,w, 1, UpperRight));
        list.push(AutoGlyphV::from_ag(self, h, w, 1, LowerRight));        

    }

}

impl AutoGlyphV {
    fn from_ag(ag:&AutoGlyph, r_adjust:f32, c_adjust:f32, bg:u32, corner:Corner) -> AutoGlyphV {
        AutoGlyphV {
                     index : ag.index,
                     pos : [c_adjust+ag.raw_c, r_adjust+ag.raw_r],
                     corner : corner as u32,
                     end_pos : [c_adjust+ag.end_c, r_adjust+ag.end_r],
                     bg : bg,
                     seed : 7.,
                     start_t: ag.start_t,
                     end_t: ag.end_t
        }
    }
}
