use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, BlendComponent, BlendFactor, BlendState, ColorTargetState, ColorWrites,
            RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError, TextureFormat, BlendOperation,
        },
        texture::BevyDefault,
    },
    sprite::{Material2d, Material2dKey},
};

use super::LIGHT2D_SPRITE_MATERIAL_SHADER_HANDLE;

// Material
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
            fragment.targets = vec![Some(ColorTargetState {
                format: TextureFormat::bevy_default(),
                blend: Some(BlendState {
                    color: BlendComponent {
                        src_factor: BlendFactor::One,
                        dst_factor: BlendFactor::One,
                        operation: BlendOperation::Add,
                    },
                    alpha: BlendComponent::REPLACE,
                }),
                write_mask: ColorWrites::ALL,
            })]
        }
        Ok(())
    }
}
