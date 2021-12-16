#version 330 core
in vec2 v_texture_coords;
in vec3 v_color;

out vec4 color;

uniform sampler2D image;

void main()
{    
    color = vec4(v_color, 1.0) * texture(image, v_texture_coords);
}  