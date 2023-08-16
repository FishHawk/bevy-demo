use bevy::{
    prelude::*,
    render::{
        render_resource::{AddressMode, FilterMode, SamplerDescriptor},
        texture::{CompressedImageFormats, ImageSampler, ImageType},
    },
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::Collider;

use crate::{
    freeform_polygon_mesh, moveable_bundle, solid_bundle, stair_bundle, Background,
    BackgroundBundle, BackgroundMaterial, BackgroundMaterialImages, BackgroundRepeat,
    CameraBoundary, CameraMode, GameDateTimeText, Light2dFreeformMaterial, LightIntensity,
    OutlineMaterial, SolidBundle, StairBundle, OUTLINE_MATERIAL_MESH_HANDLE, RENDER_LAYER_LIGHT1,
    RENDER_LAYER_MAIN2,
};

fn transform(position: Vec2, size: Vec2, z: f32) -> Transform {
    Transform {
        translation: (position + size / 2.0).extend(z),
        scale: size.extend(1.),
        ..default()
    }
}

fn sprite_placeholder(position: Vec2, size: Vec2, z: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite { color, ..default() },
        transform: Transform {
            translation: (position + size / 2.0).extend(z),
            scale: size.extend(1.),
            ..default()
        },
        ..default()
    }
}

pub fn sprite_bundle(position: Vec2, size: Vec2, z: f32, texture: Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        texture: texture,
        transform: Transform {
            translation: (position + size / 2.0).extend(z),
            scale: size.extend(1.),
            ..default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        ..default()
    }
}

fn debug_stair_bundle_pair(position1: Vec2, position2: Vec2) -> Vec<(SpriteBundle, StairBundle)> {
    let size = Vec2::new(20.0, 2.0);
    vec![
        (
            sprite_placeholder(position1, size, 9.4, Color::BLUE),
            stair_bundle(position2 - position1),
        ),
        (
            sprite_placeholder(position2, size, 9.4, Color::BLUE),
            stair_bundle(position1 - position2),
        ),
    ]
}

fn debug_solid_bundle(position: Vec2, size: Vec2) -> (SpriteBundle, SolidBundle) {
    (
        sprite_placeholder(position, size, 10.0, Color::BLACK),
        solid_bundle(),
    )
}

const BORDER: f32 = 60.0;
const INTERVAL: f32 = 10.0;

const STAIR_WIDTH: f32 = 90.0;
const ROOM_WIDTH: f32 = 120.0;
const OUTSIDE_HEIGHT: f32 = 250.0;
const LAYER_HEIGHT: f32 = 120.0;

pub fn shelter_position(room: IVec2, offset: Vec2) -> Vec2 {
    // temp
    let width = (ROOM_WIDTH * 7.0 + STAIR_WIDTH) / 2.0;
    Vec2::new(
        (room.x as f32) * ROOM_WIDTH - width,
        -(room.y as f32) * (LAYER_HEIGHT + INTERVAL),
    ) + offset
}

pub fn setup_shelter(
    mut commands: Commands,
    asset: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    ref mut images: ResMut<Assets<Image>>,
    mut background_materials: ResMut<Assets<BackgroundMaterial>>,
    mut light2d_freeform_materials: ResMut<Assets<Light2dFreeformMaterial>>,
    mut outline_materials: ResMut<Assets<OutlineMaterial>>,
) {
    let person_image = load_texture("demo/person.png");
    let person_size = person_image.texture_descriptor.size;
    let person_size = Vec2::new(person_size.width as f32, person_size.height as f32);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: OUTLINE_MATERIAL_MESH_HANDLE.typed().into(),
            material: outline_materials.add(OutlineMaterial {
                color: Color::CYAN,
                line_width: 1,
                texture: images.add(person_image),
            }),
            transform: transform(
                shelter_position(IVec2::new(3, 1), Vec2::ZERO),
                person_size,
                100.0,
            ),
            ..Default::default()
        },
        moveable_bundle(
            Collider::compound(vec![(
                Vec2::new(0.0, -0.5 + 0.5 * 4.0 / person_size.y),
                0.0,
                Collider::cuboid(0.5, 0.5 * 4.0 / person_size.y),
            )]),
            80.0,
        ),
    ));

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        GameDateTimeText,
    ));

    // Spawn background
    let mut spawn_background = |texture_path: &str, speed: Vec2, z: f32| {
        let background_images = BackgroundMaterialImages::palette(
            images,
            BackgroundRepeat::X,
            texture_path,
            "demo/lut.png",
        );
        commands.spawn((
            BackgroundBundle {
                material_bundle: BackgroundMaterial::bundle(
                    &mut background_materials,
                    background_images,
                ),
                background: Background {
                    position: Vec2::new(0.0, -324.0 / 2.0),
                    offset: Vec2::new(0.0, 1.5),
                    speed,
                    z,
                    scale: 0.5,
                    ..default()
                },
            },
            RENDER_LAYER_MAIN2,
        ));
    };

    spawn_background("demo/1.png", Vec2::new(0.0, 0.5), 0.1);
    spawn_background("demo/2.png", Vec2::new(0.0, 0.2), 0.2);
    spawn_background("demo/3.png", Vec2::new(0.0, 0.1), 0.3);
    spawn_background("demo/4.png", Vec2::new(0.0, 0.0), 0.4);
    spawn_background("demo/5.png", Vec2::new(0.0, 0.0), 0.5);
    spawn_background("demo/6.png", Vec2::new(0.0, 0.0), 0.6);

    // Spawn shelter
    let real_resolution = Vec2::new(960.0, 540.0);
    let room_number: UVec2 = UVec2::new(7, 5);

    let width = (ROOM_WIDTH * room_number.x as f32 + STAIR_WIDTH) / 2.0;
    let height = (room_number.y as f32) * (LAYER_HEIGHT + INTERVAL) + BORDER;

    commands.insert_resource(CameraBoundary {
        real_resolution,
        negative: Vec2::new(-real_resolution.x / 2.0, -height + 0.5 * BORDER),
        positive: Vec2::new(real_resolution.x / 2.0, OUTSIDE_HEIGHT),
        scale_level: 1,
        mode: CameraMode::Free,
    });

    let mesh = meshes.add(freeform_polygon_mesh(
        vec![
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 0.0),
        ],
        0.0,
    ));

    // background light
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.clone().into(),
            material: light2d_freeform_materials.add(Light2dFreeformMaterial { ..default() }),
            transform: Transform {
                translation: Vec3::new(-width - 50.0, 0.0, 1.0),
                scale: Vec3::new(width * 2.0 + 100.0, 500.0, 0.0),
                ..default()
            },
            ..default()
        },
        RENDER_LAYER_LIGHT1,
        LightIntensity {
            max: 1.0,
            min: 0.1,
            addition: 0.0,
        },
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.clone().into(),
            material: light2d_freeform_materials.add(Light2dFreeformMaterial {
                color: Color::rgb(0.4, 0.4, 0.4),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(-width - 50.0, -height - 500.0, 1.0),
                scale: Vec3::new(width * 2.0 + 100.0, height + 500.0, 0.0),
                ..default()
            },
            ..default()
        },
        RENDER_LAYER_LIGHT1,
    ));

    // left border
    commands.spawn(debug_solid_bundle(
        Vec2::new(-width - BORDER, -height),
        Vec2::new(BORDER, height),
    ));

    // right border
    commands.spawn(debug_solid_bundle(
        Vec2::new(width, -height),
        Vec2::new(BORDER, height),
    ));

    // bottom border
    commands.spawn(debug_solid_bundle(
        Vec2::new(-width, -height),
        Vec2::new(2.0 * width, BORDER),
    ));

    // layers
    let wall_image = asset.load("demo/wall.png");
    for y in 0..room_number.y {
        let position_y = -(y as f32 + 1.0) * (LAYER_HEIGHT + INTERVAL);
        // rooms
        for x in 0..room_number.x {
            commands.spawn(sprite_bundle(
                Vec2::new(-width + (x as f32) * ROOM_WIDTH, position_y),
                Vec2::new(ROOM_WIDTH, LAYER_HEIGHT),
                9.0,
                wall_image.clone(),
            ));
        }

        // stair
        commands.spawn_batch(debug_stair_bundle_pair(
            Vec2::new(width - STAIR_WIDTH, position_y),
            Vec2::new(width - 20.0, position_y + (LAYER_HEIGHT + INTERVAL) / 2.0),
        ));
        commands.spawn_batch(debug_stair_bundle_pair(
            Vec2::new(width - 20.0, position_y + (LAYER_HEIGHT + INTERVAL) / 2.0),
            Vec2::new(width - STAIR_WIDTH, position_y + (LAYER_HEIGHT + INTERVAL)),
        ));
        commands.spawn(debug_solid_bundle(
            Vec2::new(
                width - 10.0,
                position_y + (LAYER_HEIGHT + INTERVAL) / 2.0 - INTERVAL,
            ),
            Vec2::new(10.0, INTERVAL),
        ));

        // ceil
        commands.spawn(debug_solid_bundle(
            Vec2::new(-width, position_y + LAYER_HEIGHT),
            Vec2::new(2.0 * width, INTERVAL),
        ));
    }
}

// hacky
fn load_texture(texture_path: &str) -> Image {
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
    image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
        address_mode_u: AddressMode::ClampToBorder,
        address_mode_v: AddressMode::ClampToBorder,
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        mipmap_filter: FilterMode::Nearest,
        ..default()
    });
    image
}
