#version 450

layout(location = 0) in vec2 in_position;
layout(location = 1) in vec3 in_color;


layout(push_constant) uniform Push {
    mat2 transform;
    vec2 offset;
    vec3 color;
} push;

// out gl_PerVertex {
//     vec2 gl_Position;
// };

void main() {
    gl_Position = vec4(push.transform * in_position + push.offset.xy, 0.0, 1.0);

    //out_color = in_color;
}