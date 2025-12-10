#version 460
#pragma shader_stage(fragment)

// Adds support for non-uniform indexing and variable sized descriptor arrays
#extension GL_EXT_nonuniform_qualifier : require

// textures bound to set 0
layout(set = 0, binding = 0) uniform sampler u_Sampler;
layout(set = 0, binding = 1) uniform texture2D u_Textures[];

// frame data
layout(set = 1, binding = 0) uniform ubo {
    float delta_time;
    float current_time;
} u_FrameConstants;

// Inputs
layout(location = 0) in vec4 in_VertexColor;
layout(location = 1) in vec2 in_UV;
layout(location = 2) flat in int in_TextureIndex;

// Outputs
layout(location = 0) out vec4 out_FragColor;

void main() {
    float t = u_FrameConstants.current_time;
    float pi = 3.1415;
    float tau = pi * 2;

    float frag_color = 0.0;

    {
        float line_h =
            0.5
                + 0.125 * cos(t * pi + in_UV.x * tau * 1.5)
                + 0.0125 * cos(0.5 * pi + t * pi + in_UV.x * tau * 3.2)
                + 0.006 * cos(0.31 * pi + t * pi * 1.2 + in_UV.x * tau * 7.8);
        float d_line = 1.0 - (abs(in_UV.y - line_h) / 0.5);
        frag_color += pow(d_line, 32.0);
    }

    out_FragColor = vec4(vec3(frag_color), 1.0);
}
