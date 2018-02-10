use glyph_atlas::GlyphAtlas;
use glium::backend::Facade;
use glium::{self, VertexBuffer};
use rand;

#[derive(Copy, Clone, Debug)]
pub struct AutoGlyphV {
    glyph : char,
    index : u32,
    randomizations : u32,
    seed: f32,
    pos: [f32; 4],
    start_t: f64,
    end_t: f64,
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


impl AutoGlyphV {

    pub fn basic(g: char,
                 r:f32,
                 c:f32,
                 st:f64,
                 et:f64,
                 fg:[f32; 4],
                 bg:[f32; 4],
                 randomizations: u32
    ) -> AutoGlyphV{
        
        AutoGlyphV {
            glyph : g,
            index : 0,
            pos : [r,c,0.,0.],
            bg :  bg,
            fg : fg,
            seed : rand::random::<f32>(),
            randomizations : randomizations,
            start_t: st,
            end_t: et,
            special : 0,
            special_data: [[0.; 4]; 4],
        }
    }

    pub fn set_special(&mut self,
                       special: u32,
                       special_data: [[f32; 4]; 4]) {
        self.special = special;
        self.special_data = special_data;
    }

    pub fn make_tranfer_ready(&mut self,
                       display:&glium::backend::Facade,
                       atlas:&mut GlyphAtlas) {
        let atlas_entry = atlas.get_entry(display, self.glyph);
        self.index = atlas_entry.index;
    }
}


pub struct PreInstallGlyphBatch {
    glyphs : Vec<AutoGlyphV>,
    name : Option<String>
}

impl PreInstallGlyphBatch {
    pub fn new(name: Option<String>,
               glyphs: Vec<AutoGlyphV>) -> PreInstallGlyphBatch {
        PreInstallGlyphBatch  {
            glyphs : glyphs,
            name : name,
        }
    
    }

    pub fn install(self, display:&Facade, atlas:&mut GlyphAtlas)
                   -> InstalledGlyphBatch {
        InstalledGlyphBatch::new(display,
                                 atlas,
                                 self.name,
                                 self.glyphs)
    }
}

pub struct InstalledGlyphBatch {
    buffer : VertexBuffer<AutoGlyphV>,
    latest_end : f64,
    name : Option<String>
}

impl InstalledGlyphBatch {

    fn new(display:&Facade,
           atlas:&mut GlyphAtlas,
           name:Option<String>,
           mut glyphs:Vec<AutoGlyphV>) -> InstalledGlyphBatch {
        let mut latest_end:f64 = 0.;
        for g in &mut glyphs {
                if latest_end <  g.end_t { latest_end = g.end_t }
                g.make_tranfer_ready(display, atlas);
        }
        let buffer = VertexBuffer::new(display, &glyphs).unwrap();
        InstalledGlyphBatch { buffer : buffer,
                              name : name,
                              latest_end : latest_end
        }
    }

    pub fn buffer(&self) -> &VertexBuffer<AutoGlyphV> {
        &self.buffer
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }


    pub fn name_matches(&self, other_name:&String) -> bool {
        match self.name {
            None => false,
            Some(ref name_string) => name_string == other_name
        }
    }

    pub fn latest_end(&self) -> f64 {
        self.latest_end
    }
    
}
