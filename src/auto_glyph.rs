
use glyph_atlas::AtlasEntry;
use rand;

pub struct AutoGlyph {
    index:u32,
    raw_r:f32,
    raw_c:f32,
    end_r:f32,
    end_c:f32,
    start_t:f32,
    end_t:f32,
    fg : TimeVaryingVal
}

pub struct TimeVaryingVal {
    data: [[f32; 4]; 4]
}

#[derive(Copy, Clone)]
pub struct AutoGlyphV {
    index : u32,
    bg : u32,
    pos: [f32; 2],
    seed: f32,
    end_pos: [f32; 2],
    start_t: f32,
    end_t: f32,
    fg: [[f32; 4]; 4]
}

implement_vertex!(AutoGlyphV, pos, end_pos, seed, bg, start_t, end_t, index, fg);

pub type VertexList = Vec<AutoGlyphV>;


impl AutoGlyph {
    pub fn new(entry:&AtlasEntry, r:f32, c:f32, fg: TimeVaryingVal, start_t:f32, end_t:f32) -> AutoGlyph {        

        AutoGlyph {
                    index: entry.index,
                    raw_r: r,
                    raw_c: c,
                    end_r: r,
                    end_c: c,
                    fg : fg, 
                    start_t:start_t,
                    end_t:end_t
        }
    }

    pub fn set_end(&mut self,r:f32,c:f32) {
        self.end_r = r;
        self.end_c = c;
    }

    pub fn add_to_vertex_list(&self, list:&mut  VertexList) {
        list.push(AutoGlyphV::from_ag(self, 0));

    }

    pub fn add_background_to_vertex_list(&self, list:&mut  VertexList) {
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
            fg : ag.fg.data(),
            seed : rand::random::<f32>(),
            start_t: ag.start_t,
            end_t: ag.end_t
        }
    }
}

impl TimeVaryingVal {
    pub fn new(v1:f32, v2:f32, v3:f32, v4:f32) -> TimeVaryingVal {
        let mut vals = [[0.; 4]; 4];
        vals[0][0] = v1;
        vals[0][1] = v2;
        vals[0][2] = v3;
        vals[0][3] = v4;
        vals[3][3] = -1.;
        TimeVaryingVal {
            data : vals
        }
    }

    pub fn set_end(&mut self, v1:f32, v2:f32, v3:f32, v4:f32) {
        self.data[1][0] = v1;
        self.data[1][1] = v2;
        self.data[1][2] = v3;
        self.data[1][3] = v4;
    }

    pub fn set_params(&mut self, v1:f32, v2:f32, v3:f32) {
        self.data[2][0] = v1;
        self.data[2][1] = v2;
        self.data[2][2] = v3;
        self.data[3][3] = 1.;
    }    

    fn data(&self) -> [[f32; 4]; 4] {
        self.data.clone()
    }    

}
