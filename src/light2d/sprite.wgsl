#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

struct Light {
    color: vec4<f32>,
    intensity: f32,
    falloff: f32,
}

@group(1) @binding(0)
var<uniform> light: Light;
@group(1) @binding(1)
var sprite: texture_2d<f32>;
@group(1) @binding(2)
var sprite_sampler: sampler;
@group(1) @binding(3)
var falloff_lookup: texture_2d<f32>;
@group(1) @binding(4)
var falloff_lookup_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let sprite_color = textureSample(sprite, sprite_sampler, mesh.uv);

    let attenuation = textureSample(
        falloff_lookup,
        falloff_lookup_sampler,
        vec2<f32>(sprite_color.a, light.falloff)
    ).r;
    let output_color = light.intensity * attenuation * light.color * sprite_color;
    return output_color;
}
