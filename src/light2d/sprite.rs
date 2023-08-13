use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
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
}
