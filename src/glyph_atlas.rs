use glium::{self, Surface};
use glium::texture::{Texture2dArray, Texture2d};
use font::{self, Rasterize, Rasterizer, FontDesc, FontKey, GlyphKey};
use std::collections::HashMap;
use std::cmp::max;

const TEXTURE_SIZE:u32 = 1024;
const NUM_PAGES:u32 = 10;
const NUM_ATTRIBUTES:usize = 8; //this should be a power of two for max opengl compatability

pub struct GlyphAtlas {
    rasterizer: Rasterizer,
    font: FontKey,
    size: font::Size,
    texture_atlas: Texture2dArray,
    attribute_textures: Texture2d,
    map: HashMap<char,AtlasEntry>,
    current_page:u32,
    current_h_pos:u32,
    current_v_pos:u32,
    line_height:u32,
    next_index:u32
        
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


        let attributes = Texture2d::empty(display,
                             TEXTURE_SIZE,
                             NUM_ATTRIBUTES as u32).unwrap();
        rasterizer.get_glyph(&GlyphKey { font_key: font, c: 'X', size: size }).unwrap();
        GlyphAtlas { rasterizer: rasterizer,
                     font: font,
                     size: size,
                     texture_atlas: texture,
                     attribute_textures: attributes,
                     map: HashMap::new(),
                     current_h_pos: 0,
                     current_v_pos: 0,
                     line_height:0,
                     current_page:0,
                     next_index:0
                     
        }
    }

    pub fn char_width(&self) -> f64 {
        self.rasterizer.metrics(self.font).unwrap().average_advance
    }

    pub fn char_height(&self) -> f64 {
         self.rasterizer.metrics(self.font).unwrap().line_height
    }

    fn char_descent(&self) -> f32 {
         self.rasterizer.metrics(self.font).unwrap().descent
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

        let mut result = AtlasEntry::new(self.next_index);
        result.set_texture_positions(self.current_page,
                                     self.current_h_pos,
                                     self.current_h_pos + slot_width,
                                     self.current_v_pos + slot_height,
                                     self.current_v_pos);
        result.set_font_data(self.char_width(),
                             self.char_height(),
                             self.char_descent());
                             
        self.next_index += 1;
        self.line_height = max(self.line_height, slot_height);
        self.current_h_pos += slot_width + 1;
        return result;
    }
    
    pub fn get_entry<F>(&mut self, display:&F, c:char) -> AtlasEntry
        where F:glium::backend::Facade
    {
        if self.map.contains_key(&c) {
            self.map.get(&c).unwrap().clone()
        } else {
            let glyph = self.rasterizer.get_glyph(&GlyphKey { font_key: self.font,
                                                                  c: c, size:
                                                                  self.size }).unwrap();
            let image = glium::texture::RawImage2d::from_raw_rgb(glyph.buf,
                                                                 (glyph.width as u32, glyph.height as u32));
            let src_texture = glium::texture::Texture2d::new(display, image).unwrap();
            let mut entry = self.allocate_next_slot(glyph.width as u32, glyph.height as u32);
            entry.set_glyph_offset(glyph.left, glyph.top);

            let fb = glium::framebuffer::SimpleFrameBuffer::new(display,
                                                                self.texture_atlas.layer(entry.page).unwrap().main_level()).unwrap();
            let rect = glium::Rect {left: 0, bottom: 0,
                                    width: glyph.width as u32, height: glyph.height as u32};
            let rect2 = glium::BlitTarget {left: entry.left as u32, bottom: entry.bottom as u32,
                                           width: glyph.width, height: glyph.height};
            src_texture.as_surface().blit_color(&rect, &fb, &rect2, glium::uniforms::MagnifySamplerFilter::Nearest);
    

            let index  = entry.attribute_index();
            self.attribute_textures.write(
                glium::Rect {left: index, bottom: 0,
                             width: 1, height: 1},
                vec![vec![entry.tex_left()]]
                );
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
    bottom:u32,

    attributes:[f32;NUM_ATTRIBUTES],

    font_height: f64,
    font_width: f64,
    font_descent: f32,
    
    index: u32 
}

enum AttributeSlots {
    Left = 0,
    Right,
    Top,
    Bottom,
    Width,
    Height,
    LeftOffset,
    TopOffset,
}


impl AtlasEntry {

    pub fn new(index:u32) -> AtlasEntry {
        AtlasEntry {
            page:1000,
            left:0,
            right:0,
            top:0,
            bottom:0,

            attributes:[0.; NUM_ATTRIBUTES],

            font_height: 0.,
            font_width: 0.,
            font_descent: 0.,
            
            index: index 
        }
    }


    pub fn set_texture_positions(&mut self, page:u32, left:u32, right:u32, top:u32, bottom:u32) {
        use self::AttributeSlots::*;


        self.page = page;

        self.attributes[Left as usize] = left as f32/TEXTURE_SIZE as f32;
        self.attributes[Right as usize] = right as f32/TEXTURE_SIZE as f32;
        self.attributes[Top as usize] = top as f32/TEXTURE_SIZE as f32;
        self.attributes[Bottom as usize] = bottom as f32/TEXTURE_SIZE as f32;
    }

    pub fn set_font_data(&mut self, font_width:f64, font_height:f64, font_descent:f32) {
        use self::AttributeSlots::*;
        
        self.font_width = font_width;
        self.font_height = font_height;
        self.font_descent = font_descent;


        let pixel_size = (self.tex_right() - self.tex_left()) * TEXTURE_SIZE as f32;
        let width = pixel_size/self.font_width as f32;

        
        self.attributes[Width as usize] = width;

        let pixel_size = (self.tex_top() - self.tex_bottom()) * TEXTURE_SIZE as f32;
        let height = pixel_size/self.font_height as f32;
        self.attributes[Height as usize] = height;
        
    }

    pub fn set_glyph_offset(&mut self, left_offset: i32, top_offset: i32) {
        use self::AttributeSlots::*;
        assert!(self.font_width != 0., "font data not set");
        self.attributes[LeftOffset as usize] = left_offset as f32/self.font_width as f32;
        self.attributes[TopOffset as usize] = (top_offset as f32 - self.font_descent as f32)/self.font_height as f32;
    }
    
    pub fn attribute_index(&self) -> u32 {
        self.index
    }
    
    pub fn tex_left(&self) -> f32 {
        self.attributes[self::AttributeSlots::Left as usize]
    }

    pub fn tex_right(&self) -> f32 {
        self.attributes[self::AttributeSlots::Right as usize]
    }

    pub fn tex_top(&self) -> f32 {
        self.attributes[self::AttributeSlots::Top as usize]
    }

    pub fn tex_bottom(&self) -> f32 {
        self.attributes[self::AttributeSlots::Bottom as usize]
    }

    pub fn width(&self) -> f32 {
        self.attributes[self::AttributeSlots::Width as usize]
    }

    pub fn height(&self) -> f32 {
        self.attributes[self::AttributeSlots::Height as usize]
    }

    pub fn left(&self) -> f32 {
        self.attributes[self::AttributeSlots::LeftOffset as usize]
    }

    pub fn top(&self) -> f32 {
        self.attributes[self::AttributeSlots::TopOffset as usize]
    }

}
