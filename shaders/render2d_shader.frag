#version 450

layout(location=0) in vec4 v_color;
layout(location=1) in vec2 v_texture_coord;
layout(location=2) flat in uint v_tex_index;

layout(location=0) out vec4 o_color;

layout(set = 0, binding = 0) uniform sampler s_diffuse;
layout(set = 0, binding = 1) uniform texture2D t_diffuse_1;
layout(set = 0, binding = 2) uniform texture2D t_diffuse_2;

void main() {
    vec2 duvdx = dFdx(v_texture_coord);
    vec2 duvdy = dFdy(v_texture_coord);

    vec4 texColor = v_color;    
	switch(v_tex_index)
	{
        case 0: texColor *= textureGrad(sampler2D(t_diffuse_1, s_diffuse), v_texture_coord, duvdx, duvdy); break;
        case 1: texColor *= textureGrad(sampler2D(t_diffuse_2, s_diffuse), v_texture_coord, duvdx, duvdy); break;
	}
    
    o_color = texColor;
}