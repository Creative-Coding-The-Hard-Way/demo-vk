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

/// Calculates the polar representation of the position.
/// Returns a float2(distance, angle_in_radians)
float2 polar(float2 p) {
    float d = length(p);
    float a = 0;
    if (p.x > 0 && p.y >= 0) {
        // top right quadrant
        a = atan(p.y / p.x);
    } else if (p.x < 0 && p.y >= 0) {
        // top left
        a = radians(180) - atan(-p.y / p.x);
    } else if (p.x <0 && p.y < 0) {
        // bottom left
        a = radians(180) + atan(p.y / p.x);
    } else if (p.x > 0 && p.y < 0) {
        // bottom right
        a = radians(360) - atan(-p.y / p.x);
    }
    return float2(d, a);
}

float4 circle(
    float2 p,
    float radius,
    float thickness,
    float angle_thickness
) {
    let d = abs(p.x - radius);
    let a = abs((p.y % radians(360)) - (angle_thickness % radians(360)));

    if (d < thickness && a < angle_thickness) {
        let d1 = (thickness - d) / thickness;
        let d2 = (angle_thickness - a) / angle_thickness;

        let interior = 10.0 * min(d1, d2);
        return float4(smoothstep(0.0, 1.0, interior));
    }
    else {
        return float4(0.0);
    }
}

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
    var p = float2(uv.x*2.0 - 1.0, uv.y*2.0 - 1.0);
    p = float2(p.x * aspect, p.y);

    var pos = polar(p);
    pos.y = (pos.y + frame.t);

    out += circle(pos, 0.1, 0.05, (0.17 + 2.0  * frame.t) % radians(70));
    out += circle(pos, 0.4, 0.05, (0.00 + 1.0  * frame.t) % radians(150));
    out += circle(pos, 0.6, 0.05, (1.89 + 0.24 * frame.t) % radians(95));
    out += circle(pos, 0.8, 0.05, radians(frame.t % 350));

    return out;
}
