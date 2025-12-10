#version 460
#pragma shader_stage(fragment)

// Adds support for non-uniform indexing and variable sized descriptor arrays
#extension GL_EXT_nonuniform_qualifier : require

// textures bound to set 0
layout(set = 0, binding = 0) uniform sampler u_Sampler;
layout(set = 0, binding = 1) uniform texture2D u_Textures[];

// Inputs
layout(location = 0) in vec4 in_VertexColor;
layout(location = 1) in vec2 in_UV;
layout(location = 2) flat in int in_TextureIndex;

// Outputs
layout(location = 0) out vec4 out_FragColor;

void main() {
    vec4 tex_color = vec4(1.0);

    // Only perform texture fetch if a texture is specified.
    if (in_TextureIndex >= 0) {
        tex_color *= texture(
                nonuniformEXT(sampler2D(u_Textures[in_TextureIndex], u_Sampler)),
                in_UV
            );
    }

    out_FragColor = in_VertexColor * tex_color;
}
