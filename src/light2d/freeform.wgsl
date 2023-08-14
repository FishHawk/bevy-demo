#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

struct Light {
    color: vec4<f32>,
    intensity: f32,
    falloff: f32,
}

@group(1) @binding(0)
var<uniform> light: Light;
@group(1) @binding(1)
var falloff_lookup: texture_2d<f32>;
@group(1) @binding(2)
var falloff_lookup_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let attenuation = textureSample(
        falloff_lookup,
        falloff_lookup_sampler,
        vec2<f32>(mesh.color.a, light.falloff)
    ).r;
    let output_color = attenuation * light.intensity * light.color;
    return output_color;
}
