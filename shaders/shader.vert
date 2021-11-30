// shader.vert
#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(location=0) out vec2 v_tex_coords;

layout(set=1, binding=0) // 1.
uniform Uniforms {
    mat4 matrix; // 2.
};

void main() {
    v_tex_coords = a_tex_coords;    // UPDATED!
    gl_Position = matrix * vec4(a_position, 1.0); // 3.
}
