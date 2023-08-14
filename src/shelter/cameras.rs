use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
    sprite::MaterialMesh2dBundle,
};

use crate::{spawn_render_target_image, Light2dOverlayMaterial};

pub const RENDER_LAYER_MAIN1: RenderLayers = RenderLayers::layer(0);
pub const RENDER_LAYER_MAIN2: RenderLayers = RenderLayers::layer(1);
pub const RENDER_LAYER_LIGHT1: RenderLayers = RenderLayers::layer(2);
pub const RENDER_LAYER_MERGE1: RenderLayers = RenderLayers::layer(3);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct LightCamera;

pub fn setup_cameras(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut overlay_materials: ResMut<Assets<Light2dOverlayMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        RENDER_LAYER_MERGE1,
    ));

    let main_texture = spawn_render_target_image(&mut images);
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
            RENDER_LAYER_MAIN1,
            MainCamera,
        ))
        .id();

    let light_texture = spawn_render_target_image(&mut images);
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
            RENDER_LAYER_LIGHT1,
            LightCamera,
        ))
        .id();

    let camera_background = commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    order: -1,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            RENDER_LAYER_MAIN2,
        ))
        .id();

    commands
        .entity(camera_main)
        .push_children(&[camera_light, camera_background]);

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
        RENDER_LAYER_MERGE1,
    ));
}
