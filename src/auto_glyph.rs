
#[derive(RustcDecodable, RustcEncodable)]
pub struct AutoGlyph {
    pub(crate) glyph:char,
    pub(crate) pos: TimeVaryingVal,
    pub(crate) start_t:f32,
    pub(crate) end_t:f32,
    pub(crate) randomizations: TimeVaryingVal,
    pub(crate) fg : TimeVaryingVal,
    pub(crate) bg : TimeVaryingVal
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TimeVaryingVal {
    pub(crate) data: [[f32; 4]; 4]
}

enum VaryingType {
    NonVarying = 0,
    Linear,
    Chs
}




impl AutoGlyph {
    pub fn new(glyph:char, pos:TimeVaryingVal, fg: TimeVaryingVal, bg: TimeVaryingVal, start_t:f32, end_t:f32) -> AutoGlyph {        

        AutoGlyph {
            glyph : glyph,
            pos : pos,
            fg : fg,
            bg : bg, 
            start_t:start_t,
            end_t:end_t,
            randomizations: TimeVaryingVal::new(0.,0.,0.,0.)
        }
    }

    pub fn set_randomizations(&mut self, num:u32) {
        self.randomizations = TimeVaryingVal::new(num as f32,0.,0.,0.);
    }

    pub fn set_nonlinear_randomizations(&mut self, num:u32, param1:f32, param2:f32) {
        let mut value = TimeVaryingVal::new(num as f32,0.,0.,0.);
        value.set_chs_params(param1,param2);
        self.randomizations = value;
    }

    pub fn end_t(&self) -> f32 {
        self.end_t
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

    pub(crate) fn data(&self) -> [[f32; 4]; 4] {
        self.data.clone()
            
    }    

}
