#version 330
layout(points) in;
layout(triangle_strip, max_vertices=8) out;

in VertexData {
    mat4 fg;
    mat4 bg;
    float seed;
    flat int index;
    float start_t;
    float end_t;
    mat4 pos;
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

// pos in attribute array
const int TEX_LEFT = 0;
const int TEX_RIGHT = 1;
const int TEX_TOP = 2;
const int TEX_BOTTOM = 3;
const int GLYPH_WIDTH = 4;
const int GLYPH_HEIGHT = 5;
const int GLYPH_LEFT_OFFSET = 6;
const int GLPYH_TOP_OFFSET = 7;

// settings for time varying values
const int NON_VARYING = 0;
const int LINEAR = 1;
const int CHS = 2;

float getAttribute(int slot, int index) {
    return texture(attributes, vec2((slot + .5)/8., (index + .5)/1024.))[0];
    //return texelFetch(attributes, ivec2(slot, index), 0)[0];
}

float rand(float fseed, float seed){
    return fract(sin(dot(vec2(fseed,seed),vec2(12.9898,78.233))) * 43758.5453);
}

float progress(mat4 params) {
    if(params[3][3] == NON_VARYING) return 0.;
    float p = (t - data[0].start_t)/(data[0].end_t - data[0].start_t); // percent of total time
    if(p > 1) return 1.;
    if(params[3][3] == LINEAR) return p;
    float p_2 = p*p;
    float p_3 = p_2*p;
    return (p_3-2*p_2+p)*params[2][0] + (-2*p_3+3*p_2) + (p_3 - p_2)*params[2][1];
}

vec4 interpolate(float progress, vec4 v1, vec4 v2) {
    return v1*(1-progress) + v2*progress;
}

void main()
{


    int index = data[0].index;
    //index = int(rand(data[0].seed,t)*(max_index+1));
    float width = getAttribute(GLYPH_WIDTH, index);
    float height = getAttribute(GLYPH_HEIGHT, index);
    vec4 mod_pos = interpolate(progress(data[0].pos),
                               data[0].pos[0],
                               data[0].pos[1]);
    float start_r = mod_pos[0];
    float start_c = mod_pos[1];

    fseed2 = data[0].seed;
    // FIRST OUTPUT BACKGROUND
    
    fbg2 = 1;

    fg = interpolate(progress(data[0].bg), data[0].bg[0], data[0].bg[1]);
    
    float bgwidth = ceil(width);
    float bgheight = 1;
    gl_Position = matrix * vec4(start_c, start_r, 0.0, 1.);
    EmitVertex();
    gl_Position = matrix * vec4(start_c, start_r + bgheight, 0.0, 1.);
    EmitVertex();
    gl_Position = matrix * vec4(start_c + bgwidth, start_r, 0.0, 1.);
    EmitVertex();
    gl_Position = matrix * vec4(start_c + bgwidth, start_r + bgheight, 0.0, 1.);
    EmitVertex();

    EndPrimitive();

    
    //if(data[0].bg != 0) {
    //    width = ceil(width);
    //    height = 1;
    //} else {
    float left_offset = getAttribute(GLYPH_LEFT_OFFSET, index);
    float top_offset = 1 - getAttribute(GLPYH_TOP_OFFSET, index);

    start_r += top_offset;
    start_c += left_offset;
        //}

    fg = interpolate(progress(data[0].fg), data[0].fg[0], data[0].fg[1]);
    fbg2 = 0;
            
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

    start_r = mod_pos[0];
    start_c = mod_pos[1];


}  
