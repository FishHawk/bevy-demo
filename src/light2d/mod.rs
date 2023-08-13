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

fn setup(
    ref mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    ref mut images: ResMut<Assets<Image>>,
    mut overlay_materials: ResMut<Assets<Light2dOverlayMaterial>>,
) {
    let camera_base = commands
        .spawn((Camera2dBundle::default(), RENDER_LAYER_BASE))
        .id();

    let (camera_world, world) = spawn_camera_render_to_image(
        commands,
        images,
        ClearColorConfig::Custom(Color::rgba(0.0, 0.0, 0.0, 0.0)),
        RENDER_LAYER_WORLD,
    );
    let (camera_light, overlay) = spawn_camera_render_to_image(
        commands,
        images,
        ClearColorConfig::Custom(Color::BLACK),
        RENDER_LAYER_LIGHT,
    );

    let mesh = meshes.add(Mesh::from(shape::Quad::default()));
    commands.spawn((
        MaterialMesh2dBundle::<Light2dOverlayMaterial> {
            mesh: mesh.clone().into(),
            material: overlay_materials.add(Light2dOverlayMaterial { world, overlay }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(960.0, 540.0, 1.0),
                ..default()
            },
            ..default()
        },
        RENDER_LAYER_BASE,
    ));
}

fn spawn_camera_render_to_image(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    clear_color: ClearColorConfig,
    render_layers: RenderLayers,
) -> (Entity, Handle<Image>) {
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
    let overlay = images.add(overlay_image);

    let camera = commands
        .spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color,
                    ..default()
                },
                camera: Camera {
                    order: -1,
                    target: RenderTarget::Image(overlay.clone()),
                    ..default()
                },
                ..default()
            },
            render_layers,
        ))
        .id();
    (camera, overlay)
}
