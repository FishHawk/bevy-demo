use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    freeform_polygon_mesh, solid_bundle, spawn_person, sprite_bundle_pure_color, stair_bundle,
    transform_bundle, Background, BackgroundBundle, BackgroundMaterial, BackgroundMaterialImages,
    BackgroundRepeat, CameraBoundary, CameraMode, GameDateTimeText, Layer, Light2dFreeformMaterial,
    LightIntensity, OutlineMaterial, PathFind, SelectedPerson, SolidBundle, StairBundle,
    RENDER_LAYER_LIGHT1, RENDER_LAYER_MAIN2, sprite_bundle,
};

fn stair_bundle_pair(position1: Vec2, position2: Vec2) -> Vec<(TransformBundle, StairBundle)> {
    let size = Vec2::new(20.0, 2.0);
    vec![
        (
            transform_bundle(position1, size, 9.4),
            stair_bundle(position2 - position1),
        ),
        (
            transform_bundle(position2, size, 9.4),
            stair_bundle(position1 - position2),
        ),
    ]
}

fn solid_bundle_with_color(position: Vec2, size: Vec2) -> (SpriteBundle, SolidBundle) {
    (
        sprite_bundle_pure_color(position, size, 10.0, Color::BLACK),
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
    let id = spawn_person(
        &mut commands,
        images,
        &mut outline_materials,
        shelter_position(IVec2::new(3, 1), Vec2::ZERO),
    );
    spawn_person(
        &mut commands,
        images,
        &mut outline_materials,
        shelter_position(IVec2::new(2, 1), Vec2::ZERO),
    );
    commands.insert_resource(SelectedPerson(id));

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
    commands.spawn(solid_bundle_with_color(
        Vec2::new(-width - BORDER, -height),
        Vec2::new(BORDER, height),
    ));

    // right border
    commands.spawn(solid_bundle_with_color(
        Vec2::new(width, -height),
        Vec2::new(BORDER, height),
    ));

    // bottom border
    commands.spawn(solid_bundle_with_color(
        Vec2::new(-width, -height),
        Vec2::new(2.0 * width, BORDER),
    ));

    let mut path_find = PathFind {
        origin: Vec2::new(-500.0, -500.0),
        size: UVec2::new(100, 50),
        layers_index: vec![0; 100 * 50],
        layers: vec![],
    };
    for y in 0..room_number.y {
        let from_y = (-(y as f32 + 1.0) * (LAYER_HEIGHT + INTERVAL) + 500.0) / 10.0;
        let to_y = from_y + (LAYER_HEIGHT + INTERVAL) / 10.0;

        let from = UVec2::new(0, from_y as u32);
        let to = UVec2::new(100, to_y as u32);
        path_find.add_layer(
            from,
            to,
            Layer {
                id: y,
                x_left: 0,
                x_right: 100,
                y: from_y as u32,
            },
        );
    }
    commands.insert_resource(path_find);

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
        commands.spawn_batch(stair_bundle_pair(
            Vec2::new(width - STAIR_WIDTH, position_y),
            Vec2::new(width - 20.0, position_y + (LAYER_HEIGHT + INTERVAL) / 2.0),
        ));
        commands.spawn_batch(stair_bundle_pair(
            Vec2::new(width - 20.0, position_y + (LAYER_HEIGHT + INTERVAL) / 2.0),
            Vec2::new(width - STAIR_WIDTH, position_y + (LAYER_HEIGHT + INTERVAL)),
        ));
        commands.spawn(solid_bundle_with_color(
            Vec2::new(
                width - 10.0,
                position_y + (LAYER_HEIGHT + INTERVAL) / 2.0 - INTERVAL,
            ),
            Vec2::new(10.0, INTERVAL),
        ));

        // ceil
        commands.spawn(solid_bundle_with_color(
            Vec2::new(-width, position_y + LAYER_HEIGHT),
            Vec2::new(2.0 * width, INTERVAL),
        ));
    }
}
