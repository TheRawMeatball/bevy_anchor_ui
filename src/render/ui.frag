#version 450

// Taken from bevy_ui

layout(set = 2, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

# ifdef COLORMATERIAL_TEXTURE 
layout(set = 2, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 2, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

void main() {
    vec4 color = Color;
    # ifdef COLORMATERIAL_TEXTURE
        color *= texture(
            sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),
            v_Uv);
    # endif
    o_Target = color;//vec4(1.0, 0.0, 0.0, 0.3);
}