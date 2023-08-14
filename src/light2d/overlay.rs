use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use super::LIGHT2D_OVERLAY_MATERIAL_SHADER_HANDLE;

#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[reflect(Debug)]
#[uuid = "509e4cd3-d94a-4e4d-8cc3-471a4f10da6d"]
pub struct Light2dOverlayMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub main: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub light: Handle<Image>,
}

impl Material2d for Light2dOverlayMaterial {
    fn fragment_shader() -> ShaderRef {
        LIGHT2D_OVERLAY_MATERIAL_SHADER_HANDLE.typed().into()
    }
}
