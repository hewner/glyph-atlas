
pub struct TimeVaryingVal {
    pub(crate) data: [[f32; 4]; 4]
}

enum VaryingType {
    NonVarying = 0,
    Linear,
    Chs
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
        result.make_non_varying();
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
        self.make_linear();
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

    pub(crate) fn data(&self) -> [[f32; 4]; 4] {
        // TODO: this should consume the object
        self.data.clone()
            
    }    

}
