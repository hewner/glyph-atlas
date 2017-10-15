use glium::{self, Surface};
use glium::texture::{Texture2dArray};
use font::{self, Rasterize, Rasterizer, FontDesc, FontKey, GlyphKey, RasterizedGlyph};
use std::collections::HashMap;

const TEXTURE_SIZE:u32 = 1024;
const NUM_PAGES:u32 = 10;

pub struct GlyphAtlas {
    rasterizer: Rasterizer,
    font: FontKey,
    size: font::Size,
    texture_atlas: Texture2dArray,
    map: HashMap<char,AtlasEntry>
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
                     map: HashMap::new() }
    }

    pub fn char_width(&self) -> f64 {
        self.rasterizer.metrics(self.font).unwrap().average_advance
    }

    pub fn char_height(&self) -> f64 {
        self.rasterizer.metrics(self.font).unwrap().line_height
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
            let fb = glium::framebuffer::SimpleFrameBuffer::new(display,
                                                                self.texture_atlas.layer(0).unwrap().main_level()).unwrap();
            let rect = glium::Rect {left: 0, bottom: 0,
                                    width: glyph.width as u32, height: glyph.height as u32};
            let rect2 = glium::BlitTarget {left: 0 as u32, bottom: 0 as u32,
                                           width: glyph.width, height: glyph.height};
            src_texture.as_surface().blit_color(&rect, &fb, &rect2, glium::uniforms::MagnifySamplerFilter::Nearest);
    
            let entry = AtlasEntry {
                page:0,
                left:0.,
                right:glyph.width as f32/TEXTURE_SIZE as f32,
                top:0.,
                bottom:glyph.height as f32/TEXTURE_SIZE as f32
            };
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
    pub left:f32,
    pub right:f32,
    pub top:f32,
    pub bottom:f32
}
