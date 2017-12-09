#version 330
layout(points) in;
layout(triangle_strip, max_vertices=4) out;

in VertexData {
    mat4 fg;
    float seed;
    flat int bg;
    flat int index;

} data[];


out float fseed2;
out vec2 ftex_o2;
out vec4 fg;
flat out int fbg2;

uniform float t;
uniform int max_index;
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

float rand(float fseed, float seed){
    return fract(sin(dot(vec2(fseed,seed),vec2(12.9898,78.233))) * 43758.5453);
}


void main()
{


    int index = data[0].index;
    //index = int(rand(fseed[0],t)*(max_index+1));
    float width = getAttribute(GLYPH_WIDTH, index);
    float height = getAttribute(GLYPH_HEIGHT, index);
    float start_r = gl_in[0].gl_Position[0];
    float start_c = gl_in[0].gl_Position[1];

    if(data[0].bg != 0) {
        width = ceil(width);
        height = 1;
    } else {
        float left_offset = getAttribute(GLYPH_LEFT_OFFSET, index);
        float top_offset = 1 - getAttribute(GLPYH_TOP_OFFSET, index);

        start_r += top_offset;
        start_c += left_offset;
    }

    // float p = (t - start_t)/(end_t - start_t); // percent of total time
    // float p_2 = p*p;
    // float p_3 = p_2*p;
    // float progress = (p_3-2*p_2+p)*.4 + (-2*p_3+3*p_2) + (p_3 - p_2)*-.2;
 
    fseed2 = data[0].seed;
    fbg2 = data[0].bg;
    fg = data[0].fg[0];
            
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
