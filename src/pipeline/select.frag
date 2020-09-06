#version 450

layout(set = 0, binding = 1) uniform Locals {
    vec4 u_color;
};

layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = u_color;
}
