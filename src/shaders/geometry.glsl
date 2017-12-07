#version 330
layout(points) in;
layout(triangle_strip, max_vertices=4) out;

in float fseed[];
flat in int fbg[];
flat in int findex[];

out float fseed2;
out vec2 ftex_o2;
flat out int fbg2;

uniform float t;
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


void main()
{
  int index = findex[0];
  float width = getAttribute(GLYPH_WIDTH, index);
  float height = getAttribute(GLYPH_HEIGHT, index);
  float start_r = gl_in[0].gl_Position[0];
  float start_c = gl_in[0].gl_Position[1];

  if(fbg[0] != 0) {
    width = ceil(width);
    height = 1;
  } else {
    float left_offset = getAttribute(GLYPH_LEFT_OFFSET, index);
    float top_offset = 1 - getAttribute(GLPYH_TOP_OFFSET, index);

    start_r += top_offset;
    start_c += left_offset;
  }
  
  fseed2 = fseed[0];
  fbg2 = fbg[0];
  ftex_o2 = vec2(getAttribute(TEX_LEFT, index),getAttribute(TEX_BOTTOM, index));
  gl_Position = matrix * vec4(start_c, start_r, 0.0, 1.0);
  EmitVertex();
  ftex_o2 = vec2(getAttribute(TEX_LEFT, index),getAttribute(TEX_TOP, index));
  gl_Position = matrix * vec4(start_c, start_r + height, 0.0, 1.0);
  EmitVertex();
  ftex_o2 = vec2(getAttribute(TEX_RIGHT, index),getAttribute(TEX_BOTTOM, index));
  gl_Position = matrix * vec4(start_c + width, start_r, 0.0, 1.0);
  EmitVertex();
  ftex_o2 = vec2(getAttribute(TEX_RIGHT, index),getAttribute(TEX_TOP, index));
  gl_Position = matrix * vec4(start_c + width, start_r + height, 0.0, 1.0);
  EmitVertex();

  EndPrimitive();
}  
