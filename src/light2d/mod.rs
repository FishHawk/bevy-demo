use bevy::{
    asset::load_internal_asset,
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

pub mod overlay;
pub mod sprite;

pub use overlay::*;
pub use sprite::*;

pub const RENDER_LAYER_WORLD: RenderLayers = RenderLayers::layer(0);
pub const RENDER_LAYER_LIGHT: RenderLayers = RenderLayers::layer(1);
pub const RENDER_LAYER_BASE: RenderLayers = RenderLayers::layer(2);

const LIGHT2D_OVERLAY_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8841543261533787000);

const LIGHT2D_SPRITE_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8841543261533783000);

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
            LIGHT2D_SPRITE_MATERIAL_SHADER_HANDLE,
            "sprite.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<Light2dOverlayMaterial>::default())
            .add_plugins(Material2dPlugin::<Light2dSpriteMaterial>::default())
            .add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup(
    ref mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    ref mut images: ResMut<Assets<Image>>,
    mut overlay_materials: ResMut<Assets<Light2dOverlayMaterial>>,
) {
    commands
        .spawn((Camera2dBundle::default(), RENDER_LAYER_BASE))
        .id();

    let world = spawn_camera_render_to_image(images);
    let camera_world = commands
        .spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                    ..default()
                },
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(world.clone()),
                    ..default()
                },
                ..default()
            },
            RENDER_LAYER_WORLD,
            MainCamera,
        ))
        .id();

    let overlay = spawn_camera_render_to_image(images);
    let camera_light = commands
        .spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::WHITE),
                    ..default()
                },
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(overlay.clone()),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            RENDER_LAYER_LIGHT,
        ))
        .id();

    commands.entity(camera_world).push_children(&[camera_light]);

    let mesh = meshes.add(Mesh::from(shape::Quad::default()));
    commands.spawn((
        MaterialMesh2dBundle::<Light2dOverlayMaterial> {
            mesh: mesh.clone().into(),
            material: overlay_materials.add(Light2dOverlayMaterial { world, overlay }),
            transform: Transform {
                scale: Vec3::new(960.0, 540.0, 1.0),
                ..default()
            },
            ..default()
        },
        RENDER_LAYER_BASE,
    ));
}

fn spawn_camera_render_to_image(images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
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
        ..default()
    };
    overlay_image.resize(size);
    images.add(overlay_image)
}
