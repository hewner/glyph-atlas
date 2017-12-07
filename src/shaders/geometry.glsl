#version 330
layout(triangles) in;
layout(triangle_strip, max_vertices=3) out;

in float fseed[];
flat in int fbg[];
in vec2 ftex_o[];

out float fseed2;
out vec2 ftex_o2;
flat out int fbg2;


void main()
{
  for(int i=0; i<3; i++) {
    gl_Position = gl_in[i].gl_Position;
    fseed2 = fseed[i];
    ftex_o2 = ftex_o[i];
    fbg2 = fbg[i];

    EmitVertex();
  }
  EndPrimitive();
}  
