#version 140
in float fseed;
flat in int fbg;
in vec2 ftex_o;
out vec4 color;
uniform float t;

const int DONT_DRAW = -1;
uniform sampler2DArray tex;

float rand(float fseed, float seed){
  return fract(sin(dot(vec2(fseed,seed),vec2(12.9898,78.233))) * 43758.5453);
}

void main() {
  if(fbg == DONT_DRAW) {
    discard;
  }
  float r = rand(fseed, t);
  int letter = int(r * 52);
  float totalxOffset = (letter + ftex_o[0])/52.0;
  float g = rand(fseed, r);
  float b = rand(fseed, g);
  //color = vec4(r, g, b, 1.0);
  //color = texture(tex, vec3(totalxOffset, ftex_o[1], 1.));
  vec4 fg = vec4(1.,1.,1.,1.);
  vec4 bg = vec4(0.,0.0,0.,1.);
  if(fbg == 0) {
    vec4 alpha = texture(tex, vec3(ftex_o[0], ftex_o[1], 0.));
    color = fg*alpha + (1-alpha)*bg;
    color = vec4(fg.xyz, alpha.x);
  } else {
    color = bg;
  }
}
