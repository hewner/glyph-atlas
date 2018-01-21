use auto_glyph::*;
use rand;

pub fn generate_batch(num_rows: u32,
                  num_cols: u32,
                  start_t: f32) -> Vec<AutoGlyph> {
   let mut boxes = Vec::new();
    for r in 0..num_rows {
        for c in 0..num_cols {
            let letter = 'A' as u8 + ((c + r) % 58) as u8;
            //atlas_entry = atlas.get_entry(display, letter as char);

            let r_mod = rand::random::<f32>();
            if r_mod > 0.1 { continue; } 
         
            let mut pos = TimeVaryingVal::new(r as f32,c as f32,0.,0.);
            //pos.set_end(r as f32 + r_mod,c as f32 + c_mod,0.,0.);
            //pos.set_chs_params(0.4,-0.2);
            //pos.make_linear();

            let mut fg = TimeVaryingVal::new(1.,1.,1.,1.0);
            //fg.set_end(0.,0.3,0.,1.0);
            //fg.set_chs_params(0.4,-0.2);

            let mut bg = TimeVaryingVal::new(0.,0.,0.,1.0);
            //bg.set_end(0.5,0.,0.5,1.0);
            //bg.set_chs_params(0.4,-0.2);
            let now = SerializableTime::now();
            let mut ag = AutoGlyph::new(letter as char,
                                        pos,
                                        fg,
                                        bg,
                                        now,
                                        now + 3.);
            ag.set_nonlinear_randomizations(45, 0.4, -0.2);
            boxes.push(ag);
         
        }
    }

    boxes
}
