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

float2 offset(float2 polar_point, float angle) {
    if (angle < 0) {
        angle = radians(360) - (abs(angle) % radians(360));
    }
    return float2(polar_point.x, polar_point.y + angle);
}


float4 circle(
    float2 p,
    float radius,
    float thickness,
    float angle_thickness
) {
    p.y = p.y % radians(360);

    let half_angle = (angle_thickness % radians(360)) * 0.5;
    let d = abs(p.x - radius);
    let a = abs(p.y - half_angle);

    if (d < thickness && a < half_angle) {
        let edge = smoothstep(thickness, thickness * 0.9, d);
        let angle = smoothstep(half_angle, half_angle * 0.98, a);
        return float4(edge) * float4(angle);
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
    let t = frame.t;

    // calculate the point's position in polar coordinates
    let aspect = frame.screen_size.x / frame.screen_size.y;
    var p = float2(uv.x*2.0 - 1.0, uv.y*2.0 - 1.0);
    p = float2(p.x * aspect, p.y);
    var pos = polar(p);

    // some constants for setting up circle offsets
    let half = radians(180);
    let third = radians(120);
    let quad = radians(90);

    out += circle(offset(pos, t * 1.0), 0.1, 0.1, radians(12));
    out += circle(offset(pos, third + t * 1.0), 0.1, 0.1, radians(12));
    out += circle(offset(pos, 2.0*third + t * 1.0), 0.1, 0.1, radians(12));

    out += circle(offset(pos, -t * 1.0),            0.2, 0.05, radians(12));
    out += circle(offset(pos, third - t * 1.0),     0.2, 0.05, radians(12));
    out += circle(offset(pos, 2.0*third - t * 1.0), 0.2, 0.05, radians(12));

    out += circle(offset(pos, 0.0    + t * 1.1354), 0.3, 0.05, radians(30));
    out += circle(offset(pos, quad   + t * 1.1354), 0.3, 0.05, radians(30));
    out += circle(offset(pos, half   + t * 1.1354), 0.3, 0.05, radians(30));
    out += circle(offset(pos, quad*3 + t * 1.1354), 0.3, 0.05, radians(30));

    out += circle(offset(pos, 0.0     - t * 0.73),     0.4, 0.025, radians(100));
    out += circle(offset(pos, third   - t * 0.73),   0.4, 0.025, radians(100));
    out += circle(offset(pos, 2*third - t * 0.73), 0.4, 0.025, radians(100));

    out += circle(
        offset(pos, radians(90 - t * 120)),
        0.5, 0.04,
        radians(180 + sin(t*1.1)*180)
    );
    out += circle(
        offset(pos, radians(180 + 90 + t*90)),
        0.6, 0.04,
        radians(180 + sin(t*0.9)*180)
    );
    out += circle(
        offset(pos, radians(270 + 90 - t*60)),
        0.7, 0.04,
        radians(180 + sin(t*0.7)*180)
    );
    out += circle(
        offset(pos, radians(90 + 90 + t*45)),
        0.8, 0.04,
        radians(180 + sin(t*0.5)*180)
    );

    return out;
}
