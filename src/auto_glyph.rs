
use glyph_atlas::AtlasEntry;

pub struct AutoGlyph {
    raw_r:f32,
    raw_c:f32,
    r:f32,
    c:f32,
    width:f32, //as fraction of cell
    height:f32, //as fraction of cell
    tex_left:f32, 
    tex_top:f32,
    tex_right:f32,
    tex_bottom:f32,
    //velocity
    r_vel:f32,
    c_vel:f32
}

#[derive(Copy, Clone)]
pub struct AutoGlyphV {
    bg : u32, 
    pos: [f32; 2],
    tex_o: [f32; 2],
    seed: f32,
    r_vel: f32,
    c_vel: f32
}

implement_vertex!(AutoGlyphV, pos, tex_o, seed, bg, r_vel, c_vel);

pub type VertexList = Vec<AutoGlyphV>;


impl AutoGlyph {
    pub fn new(entry:&AtlasEntry, r:f32, c:f32) -> AutoGlyph {        

        AutoGlyph { r: r + entry.descent() + 1. - entry.top()
                    , c: c + entry.left(),
                    raw_r: r,
                    raw_c: c,
                    tex_left: entry.tex_left(),
                    tex_right: entry.tex_right(),
                    tex_top: entry.tex_top(),
                    tex_bottom: entry.tex_bottom(),
                    width: entry.width(),
                    height: entry.height(),
                    r_vel:0.,
                    c_vel:0.
        }
    }

    pub fn set_vel(&mut self, r_vel:&f32, c_vel:&f32) {
        self.r_vel = *r_vel;
        self.c_vel = *c_vel;
    }

    
    
    pub fn add_to_vertex_list(&self, list:&mut  VertexList) {
        let seed = 7.;
        let row = self.r;
        let col = self.c;
        let l = self.tex_left;
        let r = self.tex_right;
        let t = self.tex_top;
        let b = self.tex_bottom;
        let w = self.width;
        let h = self.height;
        list.push(AutoGlyphV { pos : [col,row],
                               bg : 0,
                               tex_o: [l,b],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,
        });
        list.push(AutoGlyphV { pos : [col, row+h],
                               bg : 0,
                               tex_o: [l,t],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,
        });
        list.push(AutoGlyphV { pos : [col+w,row],
                               bg : 0,
                               tex_o: [r,b],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,
        });
        list.push(AutoGlyphV { pos : [col,row+h],
                               bg : 0,
                               tex_o: [l,t],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,

        });
        list.push(AutoGlyphV { pos : [col+w,row],
                               bg : 0,
                               tex_o: [r,b],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,

        });
        list.push(AutoGlyphV { pos : [col+w, row+h],
                               bg : 0,
                               tex_o: [r,t],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,
        });


    }

    pub fn add_background_to_vertex_list(&self, list:&mut  VertexList) {
        let seed = 7.;
        let row = self.raw_r;
        let col = self.raw_c;
        let w = 1.;
        let h = 1.;

        list.push(AutoGlyphV { pos : [col,row],
                               bg : 1, tex_o:
                               [0.,0.],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,

        });
        list.push(AutoGlyphV { pos : [col, row+h],
                               bg : 1,
                               tex_o: [0.,0.],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,
        });
        list.push(AutoGlyphV { pos : [col+w,row],
                               bg : 1,
                               tex_o: [0.,0.],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,

        });
        list.push(AutoGlyphV { pos : [col,row+h],
                               bg : 1,
                               tex_o: [0.,0.],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,
        });
        list.push(AutoGlyphV { pos : [col+w,row],
                               bg : 1,
                               tex_o: [0.,0.],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,
        });
        list.push(AutoGlyphV { pos : [col+w, row+h],
                               bg : 1,
                               tex_o: [0.,0.],
                               seed : seed,
                               r_vel: self.r_vel,
                               c_vel: self.c_vel,
        });

    }

}
