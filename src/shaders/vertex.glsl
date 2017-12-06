#version 140
in vec2 pos;
in vec2 end_pos;
in float seed;
in float r_vel;
in float c_vel;
in float start_t;
in float end_t;
uniform float t;
in int bg;
in int index;
in int corner;
out float fseed;
out vec2 ftex_o;
flat out int fbg;
uniform mat4 matrix;
uniform sampler2D attributes;

const int DONT_DRAW = -1;

// corner
const int UPPER_LEFT = 0;
const int UPPER_RIGHT = 1;
const int LOWER_LEFT = 2;
const int LOWER_RIGHT = 3;

// pos in attribute array
const int TEX_LEFT = 0;
const int TEX_RIGHT = 1;
const int TEX_TOP = 2;
const int TEX_BOTTOM = 3;
const int GLYPH_WIDTH = 4;
const int GLYPH_HEIGHT = 5;
const int GLYPH_LEFT_OFFSET = 6;
const int GLPYH_TOP_OFFSET = 7;

float getAttribute(int slot, int index) {
  return texture(attributes, vec2((slot + .5)/8., (index + .5)/1024.))[0];
  //return texelFetch(attributes, ivec2(slot, index), 0)[0];
}

void main() {
  fseed = seed;
  //ftex_o = tex_o;
  fbg = bg;
  if(start_t >= t || end_t < t) {
    fbg = DONT_DRAW;
    ftex_o = vec2(0.,0.);
    gl_Position = vec4(0.,0.,0.,0.);
  } else {
    float width = getAttribute(GLYPH_WIDTH, index);
    float height = getAttribute(GLYPH_HEIGHT, index);
    float start_r = pos[0];
    float start_c = pos[1]; 

    if(fbg != 0) {
      width = ceil(width);
      height = 1;
    } else {
      float left_offset = getAttribute(GLYPH_LEFT_OFFSET, index);
      float top_offset = 1 - getAttribute(GLPYH_TOP_OFFSET, index);

      start_r += top_offset;
      start_c += left_offset;
    }
    if(corner == UPPER_LEFT) {
      ftex_o = vec2(getAttribute(TEX_LEFT, index),getAttribute(TEX_BOTTOM, index));
    }
    if(corner == LOWER_LEFT) {
      ftex_o = vec2(getAttribute(TEX_LEFT, index),getAttribute(TEX_TOP, index));

      start_r = start_r + height;
    } 
    if(corner == UPPER_RIGHT) {
      start_c = start_c + width;
      ftex_o = vec2(getAttribute(TEX_RIGHT, index),getAttribute(TEX_BOTTOM, index));
    } 
    if(corner == LOWER_RIGHT) {
      start_r = start_r + height;
      start_c = start_c + width;
      ftex_o = vec2(getAttribute(TEX_RIGHT, index),getAttribute(TEX_TOP, index));
    }


    float p = (t - start_t)/(end_t - start_t); // percent of total time
    float p_2 = p*p;
    float p_3 = p_2*p;
    float progress = (p_3-2*p_2+p)*.4 + (-2*p_3+3*p_2) + (p_3 - p_2)*-.2;
    // this needs to be fixed to use start_r & start_c
    float r = pos[0]*(1 - progress) + end_pos[0]*progress; 
    float c = pos[1]*(1 - progress) + end_pos[1]*progress; 
    r = start_r;
    c = start_c;
    gl_Position = matrix * vec4(c, r, 0.0, 1.0);
  }

}
