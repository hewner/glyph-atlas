#version 330
in float fseed2;
flat in int fbg2;
in vec2 ftex_o2;
out vec4 color;
uniform float t;

const int DONT_DRAW = -1;
uniform sampler2DArray tex;

float rand(float fseed, float seed){
  return fract(sin(dot(vec2(fseed,seed),vec2(12.9898,78.233))) * 43758.5453);
}

void main() {
  if(fbg2 == DONT_DRAW) {
    discard;
  }
  float r = rand(fseed2, t);
  int letter = int(r * 52);
  float totalxOffset = (letter + ftex_o2[0])/52.0;
  float g = rand(fseed2, r);
  float b = rand(fseed2, g);
  //color = vec4(r, g, b, 1.0);
  //color = texture(tex, vec3(totalxOffset, ftex_o[1], 1.));
  vec4 fg = vec4(0.,0.8,0.,1.);
  vec4 bg = vec4(0.,0.0,0.,1.);
  if(fbg2 == 0) {
    vec4 alpha = texture(tex, vec3(ftex_o2[0], ftex_o2[1], 0.));
    color = fg*alpha + (1-alpha)*bg;
    color = vec4(fg.xyz, alpha.x);
  } else {
    color = bg;
  }
}
