#version 450

layout(location = 0) in vec3 v_Norm;

layout(set = 0, binding = 1) uniform Locals {
    vec4 u_color;
};

layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = dot(v_Norm, vec3(1.0,1.0,1.0)) * u_color;
}
