#version 460
#pragma shader_stage(vertex)

vec2 vertices[3] = vec2[](
    vec2(-0.5, -0.5),
    vec2(0.0, 0.5),
    vec2(0.5, -0.5)
);

vec4 colors[3] = vec4[](
    vec4(0.0, 0.0, 1.0, 1.0),
    vec4(0.0, 1.0, 0.0, 1.0),
    vec4(1.0, 0.0, 0.0, 1.0)
);

layout (location = 0) out vec4 vertexColor;

void main() {
    vec2 pos = vertices[gl_VertexIndex];
    vertexColor = colors[gl_VertexIndex];
    gl_Position = vec4(pos.x, pos.y, 0.0, 1.0);
}
