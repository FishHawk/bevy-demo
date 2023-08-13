use bevy::{
    asset::load_internal_asset,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_resource::{
            AddressMode, AsBindGroup, FilterMode, SamplerDescriptor, ShaderRef, TextureFormat,
        },
        texture::{CompressedImageFormats, ImageSampler, ImageType},
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::PrimaryWindow,
};
use serde::Deserialize;

use crate::MainCamera;

pub const BACKGROUND_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 8841543261533782908);

pub const BACKGROUND_MATERIAL_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID, 9282188024679778823);

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct BackgroundSystems;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        let mesh = Mesh::from(shape::Quad::default());
        app.world
            .resource_mut::<Assets<Mesh>>()
            .set_untracked(BACKGROUND_MATERIAL_MESH_HANDLE, mesh);

        load_internal_asset!(
            app,
            BACKGROUND_MATERIAL_SHADER_HANDLE,
            "background.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
            .add_systems(
                Update,
                update_background_transform.in_set(BackgroundSystems),
            );
    }
}

// Material
pub struct BackgroundMaterialImages {
    pub size: Vec2,
    pub texture_handle: Handle<Image>,
    pub palette_handle: Option<Handle<Image>>,
}

impl BackgroundMaterialImages {
    pub fn simple(
        images: &mut Assets<Image>,
        repeat: BackgroundRepeat,
        texture_path: &str,
    ) -> BackgroundMaterialImages {
        let (mode_u, mode_v) = match repeat {
            BackgroundRepeat::None => (AddressMode::ClampToBorder, AddressMode::ClampToBorder),
            BackgroundRepeat::X => (AddressMode::Repeat, AddressMode::ClampToBorder),
            BackgroundRepeat::Y => (AddressMode::ClampToBorder, AddressMode::Repeat),
            BackgroundRepeat::XY => (AddressMode::Repeat, AddressMode::Repeat),
        };
        let texture = load_texture(texture_path, Option::None, mode_u, mode_v);
        let size = texture.size();
        let texture_handle = images.add(texture);

        BackgroundMaterialImages {
            size,
            texture_handle,
            palette_handle: None,
        }
    }

    pub fn palette(
        images: &mut Assets<Image>,
        repeat: BackgroundRepeat,
        texture_path: &str,
        palette_path: &str,
    ) -> BackgroundMaterialImages {
        let (mode_u, mode_v) = match repeat {
            BackgroundRepeat::None => (AddressMode::ClampToBorder, AddressMode::ClampToBorder),
            BackgroundRepeat::X => (AddressMode::Repeat, AddressMode::ClampToBorder),
            BackgroundRepeat::Y => (AddressMode::ClampToBorder, AddressMode::Repeat),
            BackgroundRepeat::XY => (AddressMode::Repeat, AddressMode::Repeat),
        };
        let texture = load_texture(texture_path, Some(TextureFormat::R16Unorm), mode_u, mode_v);
        let size = texture.size();
        let texture_handle = images.add(texture);

        let palette = load_texture(
            palette_path,
            Option::None,
            AddressMode::ClampToEdge,
            AddressMode::ClampToEdge,
        );
        let palette_handle = images.add(palette);

        BackgroundMaterialImages {
            size,
            texture_handle,
            palette_handle: Some(palette_handle),
        }
    }
}

#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[reflect(Debug)]
#[uuid = "dc760329-e28c-43c3-89c1-fd145fa35b37"]
pub struct BackgroundMaterial {
    pub size: Vec2,
    #[uniform(0)]
    pub range: Vec4,
    #[uniform(0)]
    pub palette_rows: IVec4,
    #[uniform(0)]
    pub palette_ratio: Vec2,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
    #[texture(3)]
    pub palette: Option<Handle<Image>>,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        BACKGROUND_MATERIAL_SHADER_HANDLE.typed().into()
    }
}

impl BackgroundMaterial {
    pub fn bundle(
        materials: &mut ResMut<Assets<Self>>,
        images: BackgroundMaterialImages,
    ) -> MaterialMesh2dBundle<Self> {
        MaterialMesh2dBundle {
            mesh: BACKGROUND_MATERIAL_MESH_HANDLE.typed().into(),
            material: materials.add(BackgroundMaterial {
                size: images.size,
                range: Vec4::new(0.0, 0.0, 1.0, 1.0),
                palette_rows: match images.palette_handle {
                    None => IVec4::NEG_ONE,
                    Some(_) => IVec4::new(0, 1, 2, 3),
                },
                palette_ratio: Vec2::ZERO,
                texture: images.texture_handle,
                palette: images.palette_handle,
            }),
            ..default()
        }
    }
}

// Component
#[derive(Debug, Deserialize)]
pub enum BackgroundRepeat {
    None,
    X,
    Y,
    XY,
}

#[derive(Component)]
pub struct Background {
    pub speed: Vec2,
    pub position: Vec2,
    pub scale: f32,
    pub z: f32,
    pub offset: Vec2,
}

impl Default for Background {
    fn default() -> Self {
        Self {
            speed: Vec2::ONE,
            position: Default::default(),
            scale: 1.0,
            z: Default::default(),
            offset: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct BackgroundBundle {
    pub material_bundle: MaterialMesh2dBundle<BackgroundMaterial>,
    pub background: Background,
}

pub fn update_background_transform(
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &mut Transform), With<MainCamera>>,
    mut background_query: Query<
        (&Background, &Handle<BackgroundMaterial>, &mut Transform),
        Without<MainCamera>,
    >,
) {
    let primary_window = window_query.single();
    let mut window_size = Vec2::new(primary_window.width(), primary_window.height());

    let (camera, camera_transform) = camera_query.single();
    if let Some(viewport) = &camera.viewport {
        window_size = viewport.physical_size.as_vec2();
    }

    for (background, material_handle, mut transform) in &mut background_query {
        transform.translation = camera_transform.translation.truncate().extend(background.z);
        transform.scale = window_size.extend(transform.scale.z);

        let material = materials.get_mut(material_handle).unwrap();

        let parallax_offset = Vec2::new(1.0, -1.0)
            * (1.0 - background.speed)
            * (camera_transform.translation.truncate() - background.position);

        let relative_size = window_size / material.size / background.scale;
        let relative_position =
            0.5 + (parallax_offset / material.size) / background.scale + background.offset
                - relative_size / 2.0;

        material.range = Vec4::new(
            relative_position.x,
            relative_position.y,
            relative_size.x,
            relative_size.y,
        );
    }
}

// Util
fn load_texture(
    texture_path: &str,
    texture_format: Option<TextureFormat>,
    address_mode_u: AddressMode,
    address_mode_v: AddressMode,
) -> Image {
    let real_path = "assets/".to_owned() + texture_path;
    let ext = std::path::Path::new(&real_path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();
    let img_bytes = std::fs::read(&real_path).unwrap();
    let mut image = Image::from_buffer(
        &img_bytes,
        ImageType::Extension(ext),
        CompressedImageFormats::all(),
        true,
    )
    .unwrap();

    if let Some(image_format) = texture_format {
        image.texture_descriptor.format = image_format;
    }

    image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
        address_mode_u: address_mode_u,
        address_mode_v: address_mode_v,
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        mipmap_filter: FilterMode::Nearest,
        ..default()
    });
    image
}
