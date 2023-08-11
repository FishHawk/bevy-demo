#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings  view

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

struct Light {
    light_color: vec4<f32>,
    falloff_intensity: f32,
    outer_angle: f32,
    inner_radius_mult: f32,
    inner_angle_mult: f32,
    is_full_angle: f32,
}

@group(1) @binding(0)
var<uniform> light: Light;
@group(1) @binding(1)
var falloff_lookup_texture: texture_2d<f32>;
@group(1) @binding(2)
var falloff_lookup_sampler: sampler;
@group(1) @binding(3)
var light_lookup_texture: texture_2d<f32>;
@group(1) @binding(4)
var light_lookup_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    // r = distance, g = angle, b = x direction, a = y direction
    let lookup = textureSample(light_lookup_texture, light_lookup_sampler, mesh.uv);

    let distance = lookup.r;
    let radius_attenuation = saturate(light.inner_radius_mult * distance);

    let angle = lookup.g;
    let angle_attenuation = saturate((light.outer_angle - angle) * light.inner_angle_mult);

    var attenuation = radius_attenuation * angle_attenuation;
    attenuation = textureSample(
        falloff_lookup_texture,
        falloff_lookup_sampler,
        vec2<f32>(attenuation, light.falloff_intensity)
    ).r;

    var output_color = light.light_color;
    output_color.a *= attenuation;

    return output_color;
}
