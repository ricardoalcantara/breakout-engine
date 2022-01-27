#version 450

layout(location=0) in vec3 v_color;
layout(location=1) in vec2 v_texture_coord;

layout(location=0) out vec4 o_color;

layout(set = 0, binding = 0) uniform sampler s_diffuse;
layout(set = 0, binding = 1) uniform texture2D t_diffuse_1;
layout(set = 0, binding = 2) uniform texture2D t_diffuse_2;
// layout(set = 0, binding = 3) uniform texture2D t_diffuse_3;
// layout(set = 0, binding = 4) uniform texture2D t_diffuse_4;
// layout(set = 0, binding = 5) uniform texture2D t_diffuse_5;
// layout(set = 0, binding = 6) uniform texture2D t_diffuse_6;
// layout(set = 0, binding = 7) uniform texture2D t_diffuse_7;
// layout(set = 0, binding = 8) uniform texture2D t_diffuse_8;
// layout(set = 0, binding = 9) uniform texture2D t_diffuse_9;
// layout(set = 0, binding = 10) uniform texture2D t_diffuse_10;
// layout(set = 0, binding = 11) uniform texture2D t_diffuse_11;
// layout(set = 0, binding = 12) uniform texture2D t_diffuse_12;
// layout(set = 0, binding = 13) uniform texture2D t_diffuse_13;
// layout(set = 0, binding = 14) uniform texture2D t_diffuse_14;
// layout(set = 0, binding = 15) uniform texture2D t_diffuse_15;
// layout(set = 0, binding = 16) uniform texture2D t_diffuse_16;
// layout(set = 0, binding = 17) uniform texture2D t_diffuse_17;
// layout(set = 0, binding = 18) uniform texture2D t_diffuse_18;
// layout(set = 0, binding = 19) uniform texture2D t_diffuse_19;
// layout(set = 0, binding = 20) uniform texture2D t_diffuse_20;
// layout(set = 0, binding = 21) uniform texture2D t_diffuse_21;
// layout(set = 0, binding = 22) uniform texture2D t_diffuse_22;
// layout(set = 0, binding = 23) uniform texture2D t_diffuse_23;
// layout(set = 0, binding = 24) uniform texture2D t_diffuse_24;
// layout(set = 0, binding = 25) uniform texture2D t_diffuse_25;
// layout(set = 0, binding = 26) uniform texture2D t_diffuse_26;
// layout(set = 0, binding = 27) uniform texture2D t_diffuse_27;
// layout(set = 0, binding = 28) uniform texture2D t_diffuse_28;
// layout(set = 0, binding = 29) uniform texture2D t_diffuse_29;
// layout(set = 0, binding = 30) uniform texture2D t_diffuse_30;
// layout(set = 0, binding = 31) uniform texture2D t_diffuse_31;
// layout(set = 0, binding = 32) uniform texture2D t_diffuse_32;

void main() {
    vec4 texColor = vec4(v_color, 1.0);
	switch(1)
	{
        case 0: texColor *= texture(sampler2D(t_diffuse_1, s_diffuse), v_texture_coord); break;
        case 1: texColor *= texture(sampler2D(t_diffuse_2, s_diffuse), v_texture_coord); break;
        // case 2: texColor *= texture(sampler2D(t_diffuse_3, s_diffuse), v_texture_coord); break;
        // case 3: texColor *= texture(sampler2D(t_diffuse_4, s_diffuse), v_texture_coord); break;
        // case 4: texColor *= texture(sampler2D(t_diffuse_5, s_diffuse), v_texture_coord); break;
        // case 5: texColor *= texture(sampler2D(t_diffuse_6, s_diffuse), v_texture_coord); break;
        // case 6: texColor *= texture(sampler2D(t_diffuse_7, s_diffuse), v_texture_coord); break;
        // case 7: texColor *= texture(sampler2D(t_diffuse_8, s_diffuse), v_texture_coord); break;
        // case 8: texColor *= texture(sampler2D(t_diffuse_9, s_diffuse), v_texture_coord); break;
        // case 9: texColor *= texture(sampler2D(t_diffuse_10, s_diffuse), v_texture_coord); break;
        // case 10: texColor *= texture(sampler2D(t_diffuse_11, s_diffuse), v_texture_coord); break;
        // case 11: texColor *= texture(sampler2D(t_diffuse_12, s_diffuse), v_texture_coord); break;
        // case 12: texColor *= texture(sampler2D(t_diffuse_13, s_diffuse), v_texture_coord); break;
        // case 13: texColor *= texture(sampler2D(t_diffuse_14, s_diffuse), v_texture_coord); break;
        // case 14: texColor *= texture(sampler2D(t_diffuse_15, s_diffuse), v_texture_coord); break;
        // case 15: texColor *= texture(sampler2D(t_diffuse_16, s_diffuse), v_texture_coord); break;
        // case 16: texColor *= texture(sampler2D(t_diffuse_17, s_diffuse), v_texture_coord); break;
        // case 17: texColor *= texture(sampler2D(t_diffuse_18, s_diffuse), v_texture_coord); break;
        // case 18: texColor *= texture(sampler2D(t_diffuse_19, s_diffuse), v_texture_coord); break;
        // case 19: texColor *= texture(sampler2D(t_diffuse_20, s_diffuse), v_texture_coord); break;
        // case 20: texColor *= texture(sampler2D(t_diffuse_21, s_diffuse), v_texture_coord); break;
        // case 21: texColor *= texture(sampler2D(t_diffuse_22, s_diffuse), v_texture_coord); break;
        // case 22: texColor *= texture(sampler2D(t_diffuse_23, s_diffuse), v_texture_coord); break;
        // case 23: texColor *= texture(sampler2D(t_diffuse_24, s_diffuse), v_texture_coord); break;
        // case 24: texColor *= texture(sampler2D(t_diffuse_25, s_diffuse), v_texture_coord); break;
        // case 25: texColor *= texture(sampler2D(t_diffuse_26, s_diffuse), v_texture_coord); break;
        // case 26: texColor *= texture(sampler2D(t_diffuse_27, s_diffuse), v_texture_coord); break;
        // case 27: texColor *= texture(sampler2D(t_diffuse_28, s_diffuse), v_texture_coord); break;
        // case 28: texColor *= texture(sampler2D(t_diffuse_29, s_diffuse), v_texture_coord); break;
        // case 29: texColor *= texture(sampler2D(t_diffuse_30, s_diffuse), v_texture_coord); break;
        // case 30: texColor *= texture(sampler2D(t_diffuse_31, s_diffuse), v_texture_coord); break;
        // case 31: texColor *= texture(sampler2D(t_diffuse_32, s_diffuse), v_texture_coord); break;
	}
    
    o_color = texColor;
}