#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings  view

struct Light {
    color: vec4<f32>,
    intensity: f32,
}

@group(1) @binding(0)
var<uniform> light: Light;
@group(1) @binding(1)
var sprite: texture_2d<f32>;
@group(1) @binding(2)
var sprite_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let sprite_color = textureSample(sprite, sprite_sampler, mesh.uv);
    let color = light.color.a * light.intensity * light.color * sprite_color;
    return color;
}
