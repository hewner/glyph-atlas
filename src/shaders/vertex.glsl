#version 330
in mat4 pos;
in float seed;
in float start_t;
in float end_t;
in int index;
in mat4 fg;
in mat4 bg;

out VertexData {
    mat4 fg;
    mat4 bg;
    float seed;
    flat int index;
    float start_t;
    float end_t;
    mat4 pos;
} geom;


void main() {
    geom.seed = seed;
    geom.bg = bg;
    geom.index = index;
    geom.fg = fg;
    geom.bg = bg;
    geom.start_t = start_t;
    geom.end_t = end_t;
    geom.pos = pos;

}
