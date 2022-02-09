#version 330 core
#define MAX_TEXTURE_COUNT $MAX_TEXTURE_COUNT
in vec2 v_texture_coords;
in vec4 v_color;
in float v_tex_intex;

out vec4 o_color;

uniform sampler2D u_textures[MAX_TEXTURE_COUNT];

void main()
{   
    vec4 texColor = v_color;
	switch(int(v_tex_intex))
	{        
		#if (MAX_TEXTURE_COUNT > 0)
        case 0: texColor *= texture(u_textures[0], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 1)
        case 1: texColor *= texture(u_textures[1], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 2)
        case 2: texColor *= texture(u_textures[2], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 3)
        case 3: texColor *= texture(u_textures[3], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 4)
        case 4: texColor *= texture(u_textures[4], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 5)
        case 5: texColor *= texture(u_textures[5], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 6)
        case 6: texColor *= texture(u_textures[6], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 7)
        case 7: texColor *= texture(u_textures[7], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 8)
        case 8: texColor *= texture(u_textures[8], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 9)
        case 9: texColor *= texture(u_textures[9], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 10)
        case 10: texColor *= texture(u_textures[10], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 11)
        case 11: texColor *= texture(u_textures[11], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 12)
        case 12: texColor *= texture(u_textures[12], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 13)
        case 13: texColor *= texture(u_textures[13], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 14)
        case 14: texColor *= texture(u_textures[14], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 15)
        case 15: texColor *= texture(u_textures[15], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 16)
        case 16: texColor *= texture(u_textures[16], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 17)
        case 17: texColor *= texture(u_textures[17], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 18)
        case 18: texColor *= texture(u_textures[18], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 19)
        case 19: texColor *= texture(u_textures[19], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 20)
        case 20: texColor *= texture(u_textures[20], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 21)
        case 21: texColor *= texture(u_textures[21], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 22)
        case 22: texColor *= texture(u_textures[22], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 23)
        case 23: texColor *= texture(u_textures[23], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 24)
        case 24: texColor *= texture(u_textures[24], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 25)
        case 25: texColor *= texture(u_textures[25], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 26)
        case 26: texColor *= texture(u_textures[26], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 27)
        case 27: texColor *= texture(u_textures[27], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 28)
        case 28: texColor *= texture(u_textures[28], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 29)
        case 29: texColor *= texture(u_textures[29], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 30)
        case 30: texColor *= texture(u_textures[30], v_texture_coords); break;
        #endif
		#if (MAX_TEXTURE_COUNT > 31)
        case 31: texColor *= texture(u_textures[31], v_texture_coords); break;
        #endif
	}
    
    o_color = texColor;
}