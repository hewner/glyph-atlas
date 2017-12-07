
use glyph_atlas::AtlasEntry;
use rand;

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


#[derive(Copy, Clone)]
pub struct AutoGlyphV {
    index : u32,
    bg : u32,
    pos: [f32; 2],
    seed: f32,
    end_pos: [f32; 2],
    start_t: f32,
    end_t: f32
}

implement_vertex!(AutoGlyphV, pos, end_pos, seed, bg, start_t, end_t, index);

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
        let w = self.width;
        let h = self.height;
        list.push(AutoGlyphV::from_ag(self, 0));

    }

    pub fn add_background_to_vertex_list(&self, list:&mut  VertexList) {
        let w =   self.width.ceil(); //makes the bg right for extra with characters
        let h = 1.;

        list.push(AutoGlyphV::from_ag(self, 1));
    }

}

impl AutoGlyphV {
    fn from_ag(ag:&AutoGlyph, bg:u32) -> AutoGlyphV {
        

        AutoGlyphV {
                     index : ag.index,
                     pos : [ag.raw_r, ag.raw_c],
                     end_pos : [ag.end_r, ag.end_c],
                     bg : bg,
                     seed : rand::random::<f32>(),
                     start_t: ag.start_t,
                     end_t: ag.end_t
        }
    }
}
