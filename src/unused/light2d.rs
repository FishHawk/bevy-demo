use std::f32::consts::E;

use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, Extent3d, ShaderRef, ShaderType, TextureDimension,
            TextureFormat,
        },
    },
    sprite::{Material2d, Material2dPlugin},
};

// For Example:
//
// fn spawn_light(
//     mut commands: Commands,
//     mut background_materials: ResMut<Assets<BackgroundMaterial>>,
//     mut images: ResMut<Assets<Image>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut light_materials: ResMut<Assets<PointLight2dMaterial>>,
// ) {
//     let mesh = Mesh::from(shape::Quad::default());
//     let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

//     let image_fl = create_falloff_lookup_image(&mut images);
//     let image_pll = create_point_light_lookup_image(&mut images);

//     commands.spawn((MaterialMesh2dBundle {
//         mesh: mesh_handle.clone(),
//         material: light_materials.add(PointLight2dMaterial {
//             light_color: Color::hex("ce61767f").unwrap(),
//             falloff_intensity: 0.5,
//             inner_angle: 1.0,
//             outer_angle: 1.0,
//             inner_radius: 0.4,
//             falloff_lookup_texture: image_fl.clone(),
//             light_lookup_texture: image_pll.clone(),
//         }),
//         transform: Transform {
//             translation: Vec3::new(0.0, 0.0, 1.0),
//             scale: Vec3::new(300.0, 300.0, 0.0),
//             ..default()
//         },
//         ..default()
//     },));
// }

pub struct Light2dPlugin;

impl Plugin for Light2dPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            POINT_LIGHT_MATERIAL_SHADER_HANDLE,
            "point_light.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<PointLight2dMaterial>::default());
    }
}

// Material
#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[reflect(Debug)]
#[uuid = "44e7385a-1ea0-4785-b6f9-191b99dd2cca"]
#[uniform(0, PointLight2dMaterialUniform)]
pub struct PointLight2dMaterial {
    pub light_color: Color,
    pub falloff_intensity: f32,
    pub inner_angle: f32,
    pub outer_angle: f32,
    pub inner_radius: f32,
    #[texture(1)]
    #[sampler(2)]
    pub falloff_lookup_texture: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub light_lookup_texture: Handle<Image>,
}

#[derive(Clone, Default, ShaderType)]
pub struct PointLight2dMaterialUniform {
    pub light_color: Vec4,
    pub falloff_intensity: f32,
    pub outer_angle: f32,
    pub inner_radius_mult: f32,
    pub inner_angle_mult: f32,
    pub is_full_angle: f32,
}

impl AsBindGroupShaderType<PointLight2dMaterialUniform> for PointLight2dMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &RenderAssets<Image>,
    ) -> PointLight2dMaterialUniform {
        PointLight2dMaterialUniform {
            light_color: self.light_color.as_linear_rgba_f32().into(),
            falloff_intensity: self.falloff_intensity,
            outer_angle: self.outer_angle,
            inner_radius_mult: 1.0 / (1.0 - self.inner_radius),
            inner_angle_mult: 1.0 / (self.outer_angle - self.inner_angle),
            is_full_angle: if self.inner_angle == 1.0 { 1.0 } else { 0.0 },
        }
    }
}

pub const POINT_LIGHT_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8841543261533783000);

impl Material2d for PointLight2dMaterial {
    fn fragment_shader() -> ShaderRef {
        POINT_LIGHT_MATERIAL_SHADER_HANDLE.typed().into()
    }
}

pub fn create_falloff_lookup_image(images: &mut Assets<Image>) -> Handle<Image> {
    const WIDTH: usize = 2048;
    const HEIGHT: usize = 128;
    let mut data = Vec::with_capacity(WIDTH * HEIGHT * 4);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let alpha: f32 = x as f32 / WIDTH as f32;
            let intensity: f32 = y as f32 / HEIGHT as f32;
            let falloff = alpha.powf(E.powf(1.5 - 3.0 * intensity));
            for u in falloff.to_bits().to_le_bytes() {
                data.push(u);
            }
        }
    }
    let image = Image::new_fill(
        Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &data[..],
        TextureFormat::R32Float,
    );
    images.add(image)
}

pub fn create_point_light_lookup_image(images: &mut Assets<Image>) -> Handle<Image> {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;
    let mut data = Vec::with_capacity(WIDTH * HEIGHT * 4 * 4);
    let center = Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pos = Vec2::new(x as f32, y as f32);
            let distance = Vec2::distance(pos, center);
            let red = if x == WIDTH - 1 || y == HEIGHT - 1 {
                0.0
            } else {
                (1.0 - (2.0 * distance / (WIDTH as f32))).clamp(0.0, 1.0)
            };

            let angle_cos = (pos - center).normalize().y;
            let angle_cos = if angle_cos.is_nan() { 1.0 } else { angle_cos };
            let angle = angle_cos.acos().abs() / std::f32::consts::PI;
            let green = (1.0 - angle).clamp(0.0, 1.0);

            let direction = (center - pos).normalize();
            let blue = direction.x;
            let alpha = direction.y;

            for f in vec![red, green, blue, alpha] {
                for u in f.to_bits().to_le_bytes() {
                    data.push(u);
                }
            }
        }
    }
    let image = Image::new_fill(
        Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &data[..],
        TextureFormat::Rgba32Float,
    );
    images.add(image)
}
