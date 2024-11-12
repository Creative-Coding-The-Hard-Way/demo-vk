struct FrameData {
    float2 screen_size;
    float t;
}

struct Layer {
    float4x4 projection;
    FrameData frame;
}

// Texture atlas bindings
[vk_binding(0, 0)] SamplerState samplers[3];
[vk_binding(1, 0)] Texture2D textures[];

/// Layer data
[vk_binding(0, 1)] ConstantBuffer<Layer> layer;

[shader("fragment")]
float4 main(
    float4 tint: Sprite_Tint,
    float2 uv: Sprite_UV,
    int32_t texture: Sprite_Texture,
    uint32_t sampler_index: Sprite_Sampler
) : SV_Target {
    // Default sprite shading
    // ----------------------
    float4 out = tint;
    if (texture >= 0) {
        let tex = textures[NonUniformResourceIndex(texture)];
        let sampler = samplers[NonUniformResourceIndex(sampler_index)];
        out *= tex.Sample(sampler, uv);
    }

    // The special effect
    // ------------------

    let frame = layer.frame;
    let aspect = frame.screen_size.x / frame.screen_size.y;
    var pos = float2(uv.x*2.0 - 1.0, uv.y*2.0 - 1.0);
    pos = float2(pos.x * aspect, pos.y);

    let center = float2(0.0, 0.0);
    let d = distance(pos, center);

    if (d > 0.5) {
        out.a *= 0;
    }

    return out;
}
