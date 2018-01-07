
use glyph_atlas::AtlasEntry;
use rand;

pub struct AutoGlyph {
    index:u32,
    pos: TimeVaryingVal,
    start_t:f32,
    end_t:f32,
    randomizations: TimeVaryingVal,
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
    randomizations : u32,
    seed: f32,
    pos: [f32; 4],
    start_t: f32,
    end_t: f32,
    fg: [f32; 4],
    bg: [f32; 4],
    special: u32,
    special_data: [[f32; 4]; 4],
}

implement_vertex!(AutoGlyphV,
                  pos,
                  seed,
                  bg,
                  start_t,
                  end_t,
                  index,
                  fg,
                  special,
                  special_data,
                  randomizations);

pub type VertexList = Vec<AutoGlyphV>;


impl AutoGlyph {
    pub fn new(entry:&AtlasEntry, pos:TimeVaryingVal, fg: TimeVaryingVal, bg: TimeVaryingVal, start_t:f32, end_t:f32) -> AutoGlyph {        

        AutoGlyph {
            index: entry.index,
            pos : pos,
            fg : fg,
            bg : bg, 
            start_t:start_t,
            end_t:end_t,
            randomizations: TimeVaryingVal::new(0.,0.,0.,0.)
        }
    }

    pub fn add_to_vertex_list(&self, list:&mut  VertexList) {
        list.push(AutoGlyphV::from_ag(self));

    }

    pub fn set_randomizations(&mut self, num:u32) {
        self.randomizations = TimeVaryingVal::new(num as f32,0.,0.,0.);
    }

    pub fn set_nonlinear_randomizations(&mut self, num:u32, param1:f32, param2:f32) {
        let mut value = TimeVaryingVal::new(num as f32,0.,0.,0.);
        value.set_chs_params(param1,param2);
        self.randomizations = value;
    }

}

impl AutoGlyphV {
    pub fn from_ag(ag:&AutoGlyph) -> AutoGlyphV {
        let mut special = 0;
        let mut special_data = [[0.; 4]; 4];
        let mut num_specials = 0;

        if ag.randomizations.is_variable() {
            special = 4;
            special_data = ag.randomizations.data();
            num_specials += 1;
        }
        
        if ag.pos.is_variable() {
            special = 3;
            special_data = ag.pos.data();
            num_specials += 1;
        }

        if ag.fg.is_variable() {
            special = 2;
            special_data = ag.fg.data();
            num_specials += 1;
        }

        if ag.bg.is_variable() {
            special = 1;
            special_data = ag.bg.data();
            num_specials += 1;
        }

        if num_specials > 1 {
            println!("More than 1 time varying not supported!");
        }
        AutoGlyphV {
            index : ag.index,
            pos : ag.pos.data()[0],
            bg : ag.bg.data()[0],
            fg : ag.fg.data()[0],
            seed : rand::random::<f32>(),
            randomizations : ag.randomizations.data()[0][0] as u32,
            start_t: ag.start_t,
            end_t: ag.end_t,
            special : special,
            special_data: special_data,
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


    pub fn is_variable(&self) -> bool {
        self.data[3][3] != 0.
    }

    pub fn make_linear(&mut self) {
        self.set_varying(VaryingType::Linear);
    }

    fn data(&self) -> [[f32; 4]; 4] {
        self.data.clone()
            
    }    

}
