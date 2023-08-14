use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey},
};

use super::{create_light2d_fragment_target, LIGHT2D_SPRITE_MATERIAL_SHADER_HANDLE};

#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[reflect(Debug)]
#[uuid = "26804bff-0161-4142-8f73-f810150f1f9c"]
pub struct Light2dSpriteMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub intensity: f32,
    #[texture(1)]
    #[sampler(2)]
    pub sprite: Handle<Image>,
}

impl Material2d for Light2dSpriteMaterial {
    fn fragment_shader() -> ShaderRef {
        LIGHT2D_SPRITE_MATERIAL_SHADER_HANDLE.typed().into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _: &MeshVertexBufferLayout,
        _: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(ref mut fragment) = descriptor.fragment {
            fragment.targets = vec![Option::Some(create_light2d_fragment_target())];
        }
        Ok(())
    }
}
