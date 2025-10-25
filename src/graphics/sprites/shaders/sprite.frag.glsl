// [require(SPV_EXT_descriptor_indexing)]
//
// struct Layer {
//     float4x4 projection;
// }
//
// [vk_binding(0, 0)] // binding = 0, set = 0
// SamplerState samplers[3];
//
// [vk_binding(1, 0)] // binding = 1, set = 0
// Texture2D textures[];
//
// [vk_binding(0, 1)] // binding = 0, set = 1
// ConstantBuffer<Layer> layer;
//
// [shader("fragment")]
// float4 main(
//     float4 tint: Sprite_Tint,
//     float2 uv: Sprite_UV,
//     int32_t texture: Sprite_Texture,
//     uint32_t sampler_index: Sprite_Sampler
// ) : SV_Target {
//     printf("%i\n", texture);
//     if (texture < 0) {
//         return tint;
//     } else {
//         let tex = textures[texture];
//         let sampler = samplers[sampler_index];
//         return tex.Sample(sampler, uv) * tint;
//     }
// }

#version 460
#pragma shader_stage(fragment)
#extension GL_EXT_nonuniform_qualifier : enable

// uniform inputs
layout(set = 0, binding = 0) uniform sampler samplers[3];
layout(set = 0, binding = 1) uniform texture2D textures[];

// Not used for the fragment shader
// layout(set = 1, binding = 0) uniform Layer {
//     mat4 projection;
// };

// Per-fragment input, supplied by the vertex shader
layout(location = 0) in vec4 tint;
layout(location = 1) in vec2 uv;
layout(location = 2) flat in int texture_index;
layout(location = 3) flat in uint sampler_index;

// color attachment output
layout(location = 0) out vec4 frag_color;

void main() {
    frag_color = tint;
    if (texture_index < 0) {
        return;
    }

    vec4 tex_color = texture(
        sampler2D(
            textures[nonuniformEXT(texture_index)],
            samplers[nonuniformEXT(sampler_index)]
        ),
        uv
    );
    frag_color = tint * tex_color;
}
