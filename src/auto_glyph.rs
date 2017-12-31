
use glyph_atlas::AtlasEntry;
use rand;

pub struct AutoGlyph {
    index:u32,
    pos: TimeVaryingVal,
    start_t:f32,
    end_t:f32,
    fg : TimeVaryingVal,
    bg : TimeVaryingVal
}

pub struct TimeVaryingVal {
    data: [[f32; 4]; 4]
}

enum VaryingType {
    NonVarying = 0,
    Linear,
    Chs
}


#[derive(Copy, Clone)]
pub struct AutoGlyphV {
    index : u32,
    seed: f32,
    pos: [[f32; 4]; 4],
    start_t: f32,
    end_t: f32,
    fg: [f32; 4],
    bg: [f32; 4],
    special: u32,
    special_data: [[f32; 4]; 4],
}

implement_vertex!(AutoGlyphV, pos, seed, bg, start_t, end_t, index, fg, special, special_data);

pub type VertexList = Vec<AutoGlyphV>;


impl AutoGlyph {
    pub fn new(entry:&AtlasEntry, pos:TimeVaryingVal, fg: TimeVaryingVal, bg: TimeVaryingVal, start_t:f32, end_t:f32) -> AutoGlyph {        

        AutoGlyph {
            index: entry.index,
            pos : pos,
            fg : fg,
            bg : bg, 
            start_t:start_t,
            end_t:end_t
        }
    }

    pub fn add_to_vertex_list(&self, list:&mut  VertexList) {
        list.push(AutoGlyphV::from_ag(self));

    }

}

impl AutoGlyphV {
    fn from_ag(ag:&AutoGlyph) -> AutoGlyphV {
        

        AutoGlyphV {
            index : ag.index,
            pos : ag.pos.data(),
            bg : ag.bg.data()[0],
            fg : ag.fg.data()[0],
            seed : rand::random::<f32>(),
            start_t: ag.start_t,
            end_t: ag.end_t,
            special : 2,
            special_data: ag.fg.data(),
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
        let mut result = TimeVaryingVal {
            data : vals
        };
        result.set_varying(VaryingType::NonVarying);
        result
    }

    fn set_varying(&mut self, t:VaryingType) {
        self.data[3][3] = t as i32 as f32;
    }

    pub fn set_end(&mut self, v1:f32, v2:f32, v3:f32, v4:f32) {
        self.data[1][0] = v1;
        self.data[1][1] = v2;
        self.data[1][2] = v3;
        self.data[1][3] = v4;
        self.set_varying(VaryingType::Linear);
    }

    // these params are for a cubic hermite spline
    // https://en.wikipedia.org/wiki/Cubic_Hermite_spline
    // plot (x^3-2x^2+x)*v1 + (-2x^3+3x^2) + (x^3 - x^2)*v2 from x=0 to 1
    pub fn set_chs_params(&mut self, v1:f32, v2:f32) {
        self.data[2][0] = v1;
        self.data[2][1] = v2;
        self.set_varying(VaryingType::Chs);
    }    

    pub fn make_non_varying(&mut self) {
        self.set_varying(VaryingType::NonVarying);
    }


    pub fn make_linear(&mut self) {
        self.set_varying(VaryingType::Linear);
    }

    fn data(&self) -> [[f32; 4]; 4] {
        self.data.clone()
            
    }    

}
