use auto_glyph::*;
use rand;

pub struct DrawContext {
    pub num_rows: u32,
    pub num_cols: u32,
    pub now: SerializableTime,
}

pub fn generate_batch(dc:&DrawContext) -> Vec<AutoGlyph> {
   let mut boxes = Vec::new();
    for r in 0..dc.num_rows {
        for c in 0..dc.num_cols {
            let letter = 'A' as u8 + ((c + r) % 58) as u8;
            //atlas_entry = atlas.get_entry(display, letter as char);

            let start_offset = -1.;
            /* if rand::random::<f32>() < 0.8 {
                start_offset = 1.;
            }
*/
            let end_offset = 0.; //rand::random::<f32>();
         
            let mut pos = TimeVaryingVal::new(r as f32,c as f32,0.,0.);
            let mut fg = TimeVaryingVal::new(1.,1.,1.,1.0);
            let mut bg = TimeVaryingVal::new(0.,0.,0.,1.0);

            let mut ag = AutoGlyph::new(letter as char,
                                        pos,
                                        fg,
                                        bg,
                                        dc.now + start_offset,
                                        dc.now + 3. - end_offset);
            ag.set_nonlinear_randomizations(45, 0.4, -0.2);
            boxes.push(ag);
         
        }
    }

    boxes
}
