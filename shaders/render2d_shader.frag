#version 330 core

in vec2 v_texture_coords;
in vec4 v_color;
in float v_tex_intex;

out vec4 o_color;

uniform sampler2D u_textures[32];

void main()
{   
    int index = int(v_tex_intex);
    
    o_color = texture(u_textures[index], v_texture_coords) * v_color;
}