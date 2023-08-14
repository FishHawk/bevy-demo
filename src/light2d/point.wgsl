#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

struct Light {
    color: vec4<f32>,
    intensity: f32,
    falloff: f32,
    outer_angle: f32,
    inner_radius_mult: f32,
    inner_angle_mult: f32,
    is_full_angle: f32,
}

@group(1) @binding(0)
var<uniform> light: Light;
@group(1) @binding(1)
var falloff_lookup: texture_2d<f32>;
@group(1) @binding(2)
var falloff_lookup_sampler: sampler;
@group(1) @binding(3)
var circle_lookup: texture_2d<f32>;
@group(1) @binding(4)
var circle_lookup_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    // r = distance, g = angle, b = x direction, a = y direction
    let lookup = textureSample(circle_lookup, circle_lookup_sampler, mesh.uv);

    let distance = lookup.r;
    let radius_attenuation = saturate(light.inner_radius_mult * distance);

    let angle = lookup.g;
    let angle_attenuation = saturate((light.outer_angle - angle) * light.inner_angle_mult);

    var attenuation = radius_attenuation * angle_attenuation;
    attenuation = textureSample(
        falloff_lookup,
        falloff_lookup_sampler,
        vec2<f32>(attenuation, light.falloff)
    ).r;
    var output_color = light.intensity * attenuation * light.color;
    return output_color;
}
