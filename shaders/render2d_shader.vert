#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec4 a_color;
layout(location=2) in vec2 a_texture_coord;
layout(location=3) in uint a_tex_index;

layout(set=1, binding=0) 
uniform Uniforms {
    mat4 projection;
};

layout(location=0) out vec4 v_color;
layout(location=1) out vec2 v_texture_coord;
layout(location=2) out uint v_tex_index;

void main() {
    v_color = a_color;
    v_texture_coord = a_texture_coord;
    v_tex_index = a_tex_index;
    gl_Position = projection * vec4(a_position, 1.0);
}