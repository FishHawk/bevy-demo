#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput

@group(1) @binding(0)
var main: texture_2d<f32>;
@group(1) @binding(1)
var main_sampler: sampler;

@group(1) @binding(2)
var light: texture_2d<f32>;
@group(1) @binding(3)
var light_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let main_color = textureSample(main, main_sampler, mesh.uv);
    let light_color = textureSample(light, light_sampler, mesh.uv);
    let color = vec4<f32>(light_color.rgb * main_color.rgb, main_color.a);
    return color;
}
