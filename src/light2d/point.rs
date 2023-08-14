use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, RenderPipelineDescriptor, ShaderRef, ShaderType,
            SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey},
};

use super::{
    create_light2d_fragment_target, LIGHT2D_CIRCLE_LOOKUP_IMAGE_HANDLE,
    LIGHT2D_FALLOFF_LOOKUP_IMAGE_HANDLE, LIGHT2D_POINT_MATERIAL_SHADER_HANDLE,
};

#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[reflect(Debug)]
#[uuid = "44e7385a-1ea0-4785-b6f9-191b99dd2cca"]
#[uniform(0, Light2dPointMaterialUniform)]
pub struct Light2dPointMaterial {
    pub color: Color,
    pub intensity: f32,
    pub falloff: f32,
    pub inner_angle: f32,
    pub outer_angle: f32,
    pub inner_radius: f32,
    #[texture(1)]
    #[sampler(2)]
    pub falloff_lookup: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub circle_lookup: Handle<Image>,
}

impl Default for Light2dPointMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 1.0,
            falloff: 0.5,
            inner_angle: 1.0,
            outer_angle: 1.0,
            inner_radius: 0.0,
            falloff_lookup: LIGHT2D_FALLOFF_LOOKUP_IMAGE_HANDLE.clone().typed(),
            circle_lookup: LIGHT2D_CIRCLE_LOOKUP_IMAGE_HANDLE.clone().typed(),
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct Light2dPointMaterialUniform {
    pub color: Vec4,
    pub intensity: f32,
    pub falloff: f32,
    pub outer_angle: f32,
    pub inner_radius_mult: f32,
    pub inner_angle_mult: f32,
    pub is_full_angle: f32,
}

impl AsBindGroupShaderType<Light2dPointMaterialUniform> for Light2dPointMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &RenderAssets<Image>,
    ) -> Light2dPointMaterialUniform {
        Light2dPointMaterialUniform {
            color: self.color.as_linear_rgba_f32().into(),
            intensity: self.intensity,
            falloff: self.falloff,
            outer_angle: self.outer_angle,
            inner_radius_mult: 1.0 / (1.0 - self.inner_radius),
            inner_angle_mult: 1.0 / (self.outer_angle - self.inner_angle),
            is_full_angle: if self.inner_angle == 1.0 { 1.0 } else { 0.0 },
        }
    }
}

impl Material2d for Light2dPointMaterial {
    fn fragment_shader() -> ShaderRef {
        LIGHT2D_POINT_MATERIAL_SHADER_HANDLE.typed().into()
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
