#version 450

layout(location = 0) in vec4 in_position;
layout(location = 1) in vec4 in_color;

//layout (location = 0) out vec4 out_color;

layout(push_constant) uniform Push {
    vec4 offset;
    vec4 color;
} push;

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    gl_Position = in_position + push.offset;

    //out_color = in_color;
}