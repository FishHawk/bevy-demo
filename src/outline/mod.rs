use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

const OUTLINE_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 16091888430740423850);

pub const OUTLINE_MATERIAL_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 12182488029354938342);

pub struct OutlinePlugin;

impl Plugin for OutlinePlugin {
    fn build(&self, app: &mut App) {
        let mesh = Mesh::from(shape::Quad::default());
        app.world
            .resource_mut::<Assets<Mesh>>()
            .set_untracked(OUTLINE_MATERIAL_MESH_HANDLE, mesh);

        load_internal_asset!(
            app,
            OUTLINE_MATERIAL_SHADER_HANDLE,
            "outline.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<OutlineMaterial>::default());
    }
}

// Material
#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[reflect(Debug)]
#[uuid = "c5a092d1-e79d-4ea7-92d4-43c0798e06bf"]
pub struct OutlineMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub line_width: u32,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for OutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        OUTLINE_MATERIAL_SHADER_HANDLE.typed().into()
    }
}
