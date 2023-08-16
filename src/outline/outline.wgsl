#import bevy_sprite::mesh2d_vertex_output  MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings  view

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

struct OutlineMaterial {
    color: vec4<f32>,
    line_width: u32,
};

@group(1) @binding(0)
var<uniform> outline: OutlineMaterial;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let dims = vec2f(textureDimensions(texture));

    let uv_up = mesh.uv + vec2f(0.0, 1.0) * f32(outline.line_width) / dims;
    let uv_down = mesh.uv + vec2f(0.0, -1.0) * f32(outline.line_width) / dims;
    let uv_left = mesh.uv + vec2f(-1.0, 0.0) * f32(outline.line_width) / dims;
    let uv_right = mesh.uv + vec2f(1.0, 0.0) * f32(outline.line_width) / dims;

    let a =
        textureSample(texture, texture_sampler, uv_up).a *
        textureSample(texture, texture_sampler, uv_down).a *
        textureSample(texture, texture_sampler, uv_left).a *
        textureSample(texture, texture_sampler, uv_right).a;

    var output_color = textureSample(texture, texture_sampler, mesh.uv);
    if (output_color.a != 0.0 && a == 0.0) {
        output_color = outline.color;
    }
#ifdef TONEMAP_IN_SHADER
    output_color = bevy_core_pipeline::tonemapping::tone_mapping(output_color, view.color_grading);
#endif
    return output_color;
    // return outline.color;
}

