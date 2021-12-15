#version 330 core
layout (location = 0) in vec2 position; // <vec2 position, vec2 texCoords>
layout (location = 1) in vec3 color; // <vec2 position, vec2 texCoords>
layout (location = 2) in vec2 texture_coords; // <vec2 position, vec2 texCoords>

out vec2 v_texture_coords;
out vec3 v_color;

uniform mat4 model;
uniform mat4 projection;

void main()
{
    v_texture_coords = texture_coords;
    v_color = color;
    gl_Position = projection * model * vec4(position, 0.0, 1.0);
}