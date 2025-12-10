#version 460
#pragma shader_stage(vertex)

// Adds support for buffer references, used for vertex data
#extension GL_EXT_buffer_reference : enable

// Adds support for non-uniform indexing and variable sized descriptor arrays
#extension GL_EXT_nonuniform_qualifier : require

#extension GL_EXT_shader_explicit_arithmetic_types_int32: enable

struct Vertex {
    vec3 pos;
    float uv_x;
    vec4 color;
    int texture_index;
    float uv_y;
};

struct MeshTransform {
    mat4 transform;
};

// textures bound to set 0
layout(set = 0, binding = 0) uniform sampler u_Sampler;
layout(set = 0, binding = 1) uniform texture2D u_Textures[];

// frame data
// layout(set = 1, binding = 0) uniform ubo {
//     float delta_time;
// } u_FrameConstants;

// Push Constants
layout(buffer_reference, std430) readonly buffer VertexBuffer {
    Vertex data[];
};
layout(buffer_reference, std430) readonly buffer TransformBuffer {
    MeshTransform data[];
};
layout(push_constant) uniform constants {
    VertexBuffer vertices;
    TransformBuffer mesh_transforms;
    uint32_t transform_index;
} pc_Constants;

// Per-Vertex outputs
layout(location = 0) out vec4 out_VertexColor;
layout(location = 1) out vec2 out_UV;
layout(location = 2) flat out int out_TextureIndex;

void main() {
    Vertex vert = pc_Constants.vertices.data[gl_VertexIndex];
    out_VertexColor = vert.color;
    out_TextureIndex = vert.texture_index;
    out_UV = vec2(vert.uv_x, vert.uv_y);

    mat4 transform =
        pc_Constants.mesh_transforms.data[pc_Constants.transform_index].transform;

    gl_Position = transform * vec4(vert.pos.x, vert.pos.y, vert.pos.z, 1.0);
}
