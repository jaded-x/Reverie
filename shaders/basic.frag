#version 450

layout (location = 0) out vec4 color;

layout(push_constant) uniform Push {
    vec4 offset;
    vec4 color;
} push;

void main() {
    color = push.color;
}