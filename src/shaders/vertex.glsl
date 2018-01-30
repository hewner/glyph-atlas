#version 410
in vec4 pos;
in float seed;
in double start_t;
in double end_t;
in int index;
in int randomizations;
in vec4 fg;
in vec4 bg;

in int special;
in mat4 special_data;

out VertexData {
    vec4 fg;
    vec4 bg;
    float seed;
    flat int index;
    flat int special;
    flat int randomizations;
    double start_t;
    double end_t;
    vec4 pos;
    mat4 special_data;
} geom;


void main() {
    geom.seed = seed;
    geom.randomizations = randomizations;
    geom.bg = bg;
    geom.index = index;
    geom.fg = fg;
    geom.bg = bg;
    geom.start_t = start_t;
    geom.end_t = end_t;
    geom.pos = pos;
    geom.special = special;
    geom.special_data = special_data;
}
