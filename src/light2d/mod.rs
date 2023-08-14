use std::f32::consts::E;

use bevy::{
    asset::load_internal_asset,
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            BlendComponent, BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrites,
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{BevyDefault, ImageSampler},
        view::RenderLayers,
    },
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
    window::PrimaryWindow,
};

pub mod freeform;
pub mod overlay;
pub mod point;
pub mod sprite;

pub use freeform::*;
pub use overlay::*;
pub use point::*;
pub use sprite::*;

pub const RENDER_LAYER_WORLD: RenderLayers = RenderLayers::layer(0);
pub const RENDER_LAYER_LIGHT: RenderLayers = RenderLayers::layer(1);
pub const RENDER_LAYER_BASE: RenderLayers = RenderLayers::layer(2);

pub const LIGHT2D_POINT_DEFAULT_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 268956803042264025);

const LIGHT2D_FALLOFF_LOOKUP_IMAGE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 17108410941913908125);

const LIGHT2D_CIRCLE_LOOKUP_IMAGE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 7954851330280344899);

const LIGHT2D_OVERLAY_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3333834159522335299);

const LIGHT2D_FREEFORM_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 4344113866858121641);

const LIGHT2D_SPRITE_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 14482162647506175261);

const LIGHT2D_POINT_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2858849434766952494);

pub struct Light2dPlugin;

impl Plugin for Light2dPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            LIGHT2D_OVERLAY_MATERIAL_SHADER_HANDLE,
            "overlay.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            LIGHT2D_FREEFORM_MATERIAL_SHADER_HANDLE,
            "freeform.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            LIGHT2D_SPRITE_MATERIAL_SHADER_HANDLE,
            "sprite.wgsl",
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            LIGHT2D_POINT_MATERIAL_SHADER_HANDLE,
            "point.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<Light2dOverlayMaterial>::default())
            .add_plugins(Material2dPlugin::<Light2dSpriteMaterial>::default())
            .add_plugins(Material2dPlugin::<Light2dPointMaterial>::default())
            .add_plugins(Material2dPlugin::<Light2dFreeformMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, resize_render_targets);
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct LightCamera;

fn setup(
    ref mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    ref mut images: ResMut<Assets<Image>>,
    mut overlay_materials: ResMut<Assets<Light2dOverlayMaterial>>,
) {
    // Spawn default mesh
    meshes.set_untracked(
        LIGHT2D_POINT_DEFAULT_MESH_HANDLE,
        Mesh::from(shape::Quad::default()),
    );

    // Spawn lookup images
    spawn_falloff_lookup_image(images);
    spawn_circle_lookup_image(images);

    // Spawn cameras
    commands.spawn((Camera2dBundle::default(), RENDER_LAYER_BASE));

    let main_texture = spawn_render_target_image(images);
    let camera_main = commands
        .spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                    ..default()
                },
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(main_texture.clone()),
                    ..default()
                },
                ..default()
            },
            RENDER_LAYER_WORLD,
            MainCamera,
        ))
        .id();

    let light_texture = spawn_render_target_image(images);
    let camera_light = commands
        .spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                    ..default()
                },
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(light_texture.clone()),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            RENDER_LAYER_LIGHT,
            LightCamera,
        ))
        .id();

    commands.entity(camera_main).push_children(&[camera_light]);

    let mesh = meshes.add(Mesh::from(shape::Quad::default()));
    commands.spawn((
        MaterialMesh2dBundle::<Light2dOverlayMaterial> {
            mesh: mesh.clone().into(),
            material: overlay_materials.add(Light2dOverlayMaterial {
                main: main_texture,
                light: light_texture,
            }),
            transform: Transform {
                scale: Vec3::new(960.0, 540.0, 1.0),
                ..default()
            },
            ..default()
        },
        RENDER_LAYER_BASE,
    ));
}

fn resize_render_targets(
    materials: ResMut<Assets<Light2dOverlayMaterial>>,
    mut images: ResMut<Assets<Image>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut overlays: Query<&mut Transform, With<Handle<Light2dOverlayMaterial>>>,
) {
    let primary_window = window_query.single();
    let window_size = Vec2::new(primary_window.width(), primary_window.height());

    fn resize_render_target(render_target_size: UVec2, render_target: &mut Image) {
        if render_target_size.x != render_target.texture_descriptor.size.width
            && render_target_size.y != render_target.texture_descriptor.size.height
        {
            render_target.resize(Extent3d {
                width: render_target_size.x,
                height: render_target_size.y,
                ..default()
            });
        }
    }

    return; // bug, see https://github.com/bevyengine/bevy/issues/6480

    let render_target_size = UVec2::new(window_size.x as u32, window_size.y as u32);
    for (_, material) in materials.iter() {
        if let Some(render_target) = images.get_mut(&material.main) {
            resize_render_target(render_target_size, render_target);
        }
        if let Some(render_target) = images.get_mut(&material.light) {
            resize_render_target(render_target_size, render_target);
        }
    }

    for mut transform in overlays.iter_mut() {
        transform.scale = window_size.extend(transform.scale.z);
    }
}

// Util
fn spawn_render_target_image(images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
    let size = Extent3d {
        width: 960,
        height: 540,
        ..default()
    };
    let mut overlay_image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler_descriptor: ImageSampler::Descriptor(ImageSampler::linear_descriptor()),
        ..default()
    };
    overlay_image.resize(size);
    images.add(overlay_image)
}

pub fn spawn_falloff_lookup_image(images: &mut Assets<Image>) {
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
    let mut image = Image::new_fill(
        Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &data[..],
        TextureFormat::R32Float,
    );
    image.sampler_descriptor = ImageSampler::Descriptor(ImageSampler::linear_descriptor());
    images.set_untracked(LIGHT2D_FALLOFF_LOOKUP_IMAGE_HANDLE, image);
}

fn spawn_circle_lookup_image(images: &mut Assets<Image>) {
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
    let mut image = Image::new_fill(
        Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &data[..],
        TextureFormat::Rgba32Float,
    );
    image.sampler_descriptor = ImageSampler::Descriptor(ImageSampler::linear_descriptor());
    images.set_untracked(LIGHT2D_CIRCLE_LOOKUP_IMAGE_HANDLE, image);
}

fn create_light2d_fragment_target() -> ColorTargetState {
    ColorTargetState {
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
    }
}
