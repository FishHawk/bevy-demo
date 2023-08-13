#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings  view

@group(1) @binding(0)
var world: texture_2d<f32>;
@group(1) @binding(1)
var world_sampler: sampler;

@group(1) @binding(2)
var overlay: texture_2d<f32>;
@group(1) @binding(3)
var overlay_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let world_color = textureSample(world, world_sampler, mesh.uv);
    let light_color = textureSample(overlay, overlay_sampler, mesh.uv);
    let color = vec4<f32>(light_color.rgb * world_color.rgb, world_color.a);
    return color;
}
