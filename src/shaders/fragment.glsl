#version 330
flat in int fbg2;
in vec2 ftex_o2;
in vec4 fg;
out vec4 color;
uniform float t;

const int DONT_DRAW = -1;
uniform sampler2DArray tex;

void main() {
    float alpha;
    if(fbg2 == 0) {
        alpha = texture(tex, vec3(ftex_o2[0], ftex_o2[1], 0.)).x;
    } else {
        alpha = 1.;
    }
    color = vec4(fg.xyz, alpha);
}
