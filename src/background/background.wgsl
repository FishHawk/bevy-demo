#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings  view

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

struct BackgroundMaterial {
    range: vec4<f32>,
    palette_rows: vec4<i32>,
    palette_ratio: vec2<f32>,
};

@group(1) @binding(0)
var<uniform> material: BackgroundMaterial;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;
@group(1) @binding(3)
var palette: texture_2d<f32>;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    var uv = mesh.uv * material.range.zw  + material.range.xy;
    var output_color = vec4<f32>(1.0,1.0,1.0,1.0);
    if (material.palette_rows.x < 0) {
        output_color = textureSample(texture, texture_sampler, uv);
    } else {
        var indice = textureSample(texture, texture_sampler, uv);
        var palette_dim = textureDimensions(palette);
        var palette_y = i32(indice.r * 65535.0);

        var output_color1 = textureLoad(palette, vec2(material.palette_rows.x, palette_y), 0);
        var output_color2 = textureLoad(palette, vec2(material.palette_rows.y, palette_y), 0);
        output_color = output_color1 * material.palette_ratio.x + output_color2 * (1.0 - material.palette_ratio.x);
    }
#ifdef TONEMAP_IN_SHADER
    output_color = bevy_core_pipeline::tonemapping::tone_mapping(output_color, view.color_grading);
#endif
    return output_color;
}
