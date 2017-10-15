use glium::{self, Surface};
use glium::texture::{Texture2dArray};
use font::{self, Rasterize, Rasterizer, FontDesc, FontKey, GlyphKey, RasterizedGlyph};
use std::collections::HashMap;
use std::cmp::max;

const TEXTURE_SIZE:u32 = 1024;
const NUM_PAGES:u32 = 10;

pub struct GlyphAtlas {
    rasterizer: Rasterizer,
    font: FontKey,
    size: font::Size,
    texture_atlas: Texture2dArray,
    map: HashMap<char,AtlasEntry>,
    current_page:u32,
    current_h_pos:u32,
    current_v_pos:u32,
    line_height:u32
        
}

impl GlyphAtlas {
    pub fn new<F>(mut rasterizer: Rasterizer,
                  font_desc: &FontDesc,
                  size: font::Size,
                  display: &F) -> GlyphAtlas
        where F:glium::backend::Facade
    {
        let font = rasterizer.load_font(&font_desc, size).unwrap();
        let texture = Texture2dArray::empty(display,
                                            TEXTURE_SIZE,
                                            TEXTURE_SIZE,
                                            NUM_PAGES
        ).unwrap();
        rasterizer.get_glyph(&GlyphKey { font_key: font, c: 'X', size: size }).unwrap();
        GlyphAtlas { rasterizer: rasterizer,
                     font: font,
                     size: size,
                     texture_atlas: texture,
                     map: HashMap::new(),
                     current_h_pos: 0,
                     current_v_pos: 0,
                     line_height:0,
                     current_page:0
        }
    }

    pub fn char_width(&self) -> f64 {
        self.rasterizer.metrics(self.font).unwrap().average_advance
    }

    pub fn char_height(&self) -> f64 {
        self.rasterizer.metrics(self.font).unwrap().line_height
    }


    fn can_be_added_to_current_line(&mut self,slot_width:u32,slot_height:u32) -> bool {
        self.current_h_pos + slot_width < TEXTURE_SIZE
            && self.current_v_pos + slot_height < TEXTURE_SIZE
            && self.current_page < NUM_PAGES
    }
    
    fn allocate_next_slot(&mut self,slot_width:u32,slot_height:u32) -> AtlasEntry {

        if !self.can_be_added_to_current_line(slot_width, slot_height) {
            //try updating to the next line
            self.current_h_pos = 0;
            self.current_v_pos += self.line_height;
            self.line_height = 0;
            if !self.can_be_added_to_current_line(slot_width, slot_height) {
                //try moving to the next page
                self.current_v_pos = 0;
                self.current_page += 1;
                if !self.can_be_added_to_current_line(slot_width, slot_height) {
                    panic!("texture atlas full")
                }
                
            }

        }

        let result = AtlasEntry {
            page:self.current_page,
            left:self.current_h_pos,
            right:self.current_h_pos + slot_width,
            top:self.current_v_pos + slot_height,
            bottom:self.current_v_pos 
        };
        self.line_height = max(self.line_height, slot_height);
        self.current_h_pos += slot_width;
        return result;
    }
    
    pub fn get_entry<F>(&mut self, display:&F, c:char) -> AtlasEntry
        where F:glium::backend::Facade
    {
        if self.map.contains_key(&c) {
            self.map.get(&c).unwrap().clone()
        } else {
            let mut glyph = self.rasterizer.get_glyph(&GlyphKey { font_key: self.font,
                                                                  c: c, size:
                                                                  self.size }).unwrap();
            let image = glium::texture::RawImage2d::from_raw_rgb(glyph.buf,
                                                                 (glyph.width as u32, glyph.height as u32));
            let src_texture = glium::texture::Texture2d::new(display, image).unwrap();
            let entry = self.allocate_next_slot(glyph.width as u32, glyph.height as u32);

            let fb = glium::framebuffer::SimpleFrameBuffer::new(display,
                                                                self.texture_atlas.layer(entry.page).unwrap().main_level()).unwrap();
            let rect = glium::Rect {left: 0, bottom: 0,
                                    width: glyph.width as u32, height: glyph.height as u32};
            let rect2 = glium::BlitTarget {left: entry.left as u32, bottom: entry.bottom as u32,
                                           width: glyph.width, height: glyph.height};
            src_texture.as_surface().blit_color(&rect, &fb, &rect2, glium::uniforms::MagnifySamplerFilter::Nearest);
    
            self.map.insert(c, entry.clone());
            entry            
        }
    }

    pub fn texture(&self) -> &Texture2dArray {
        &self.texture_atlas
    }
}

#[derive(Clone)]
pub struct AtlasEntry {
    pub page:u32,
    left:u32,
    right:u32,
    top:u32,
    bottom:u32
}

impl AtlasEntry {
    pub fn tex_left(&self) -> f32 {
        self.left as f32/TEXTURE_SIZE as f32
    }

    pub fn tex_right(&self) -> f32 {
        self.right as f32/TEXTURE_SIZE as f32
    }

    pub fn tex_top(&self) -> f32 {
        self.top as f32/TEXTURE_SIZE as f32
    }

    pub fn tex_bottom(&self) -> f32 {
        self.bottom as f32/TEXTURE_SIZE as f32
    }


}
