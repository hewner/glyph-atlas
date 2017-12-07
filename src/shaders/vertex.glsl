#version 330
in vec2 pos;
in vec2 end_pos;
in float seed;
in float start_t;
in float end_t;
in int bg;
in int index;
out float fseed;
out vec2 ftex_o;
flat out int fbg;
flat out int findex;

void main() {
  fseed = seed;
  //ftex_o = tex_o;
  fbg = bg;
  findex = index;
  gl_Position = vec4(pos[0],pos[1],0.,0.);

}
