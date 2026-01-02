#version 460
#pragma shader_stage(compute)

// process square patches, size specified when pipeline is created
layout(local_size_x_id = 0, local_size_y_id = 0, local_size_z_id = 1) in;

layout(set = 0, binding = 0, rgba16f) writeonly uniform image2D out_tex;

layout(push_constant) uniform PushConstants {
    uvec2 resolution;
}
pc;

void main() {
    if (any(greaterThanEqual(gl_GlobalInvocationID.xy, pc.resolution))) {
        // If we're outside the available resolution, then just quit
        return;
    }

    imageStore(
        out_tex,
        ivec2(gl_GlobalInvocationID.xy),
        vec4(1.0, 0.0, 0.0, 1.0)
    );
}



