#version 330
in vec2 pos;
in vec2 end_pos;
in float seed;
in float start_t;
in float end_t;
in int bg;
in int index;
in mat4 fg;


out VertexData {
    mat4 fg;
    float seed;
    flat int bg;
    flat int index;
    
} geom;


void main() {
    geom.seed = seed;
    geom.bg = bg;
    geom.index = index;
    geom.fg = fg;
    gl_Position = vec4(pos[0],pos[1],0.,0.);

}
