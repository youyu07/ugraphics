#version 450

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec3 a_Norm;

layout(set = 0, binding = 0) uniform Locals {
    mat4 u_Projection;
    mat4 u_View;
};

layout(location = 0) out vec3 v_Norm;

void main() {
    gl_Position = u_Projection * u_View * vec4(a_Pos, 1.0);
    v_Norm = a_Norm;
}
