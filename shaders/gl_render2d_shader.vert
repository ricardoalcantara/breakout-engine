#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;
layout (location = 2) in vec2 texture_coords;
layout (location = 3) in float tex_index;

out vec2 v_texture_coords;
out vec4 v_color;
out float v_tex_intex;

uniform mat4 projection;

void main()
{
    v_texture_coords = texture_coords;
    v_color = color;
    v_tex_intex = tex_index;
    gl_Position = projection * vec4(position, 1.0);
}