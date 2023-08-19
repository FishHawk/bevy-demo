use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    freeform_polygon_mesh, pure_color_bundle_tile, solid_bundle, spawn_person, sprite_bundle_tile,
    stair_bundle, transform_2d_tile_m, transform_bundle_tile, world_coor, Background,
    BackgroundBundle, BackgroundMaterial, BackgroundMaterialImages, BackgroundRepeat,
    CameraBoundary, CameraMode, GameDateTimeText, Light2dFreeformMaterial, LightIntensity,
    OutlineMaterial, PathFind, SelectedPerson, SolidBundle, StairBundle, RENDER_LAYER_LIGHT1,
    RENDER_LAYER_MAIN2,
};

fn stair_bundle_pair(position1: IVec2, position2: IVec2) -> Vec<(TransformBundle, StairBundle)> {
    let size = IVec2::new(2, 1);
    vec![
        (
            transform_bundle_tile(position1, size, 9.4),
            stair_bundle(world_coor(position2 - position1)),
        ),
        (
            transform_bundle_tile(position2, size, 9.4),
            stair_bundle(world_coor(position1 - position2)),
        ),
    ]
}

fn solid_bundle_with_color(position: IVec2, size: IVec2) -> (SpriteBundle, SolidBundle) {
    (
        pure_color_bundle_tile(position, size, 10.0, Color::BLACK),
        solid_bundle(),
    )
}

const BORDER: i32 = 6;
const INTERVAL: i32 = 1;

const STAIR_WIDTH: i32 = 10;
const ROOM_WIDTH: i32 = 12;
const OUTSIDE_HEIGHT: i32 = 25;
const LAYER_HEIGHT: i32 = 11;

pub fn shelter_position(room: IVec2, offset: IVec2) -> IVec2 {
    // temp
    let width = (ROOM_WIDTH * 7 + STAIR_WIDTH) / 2;
    IVec2::new(
        room.x * ROOM_WIDTH - width,
        -room.y * (LAYER_HEIGHT + INTERVAL),
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
        shelter_position(IVec2::new(3, 1), IVec2::ZERO),
    );
    spawn_person(
        &mut commands,
        images,
        &mut outline_materials,
        shelter_position(IVec2::new(2, 1), IVec2::ZERO),
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
    let room_number = IVec2::new(7, 5);

    let width = (ROOM_WIDTH * room_number.x + STAIR_WIDTH) / 2;
    let height = room_number.y * (LAYER_HEIGHT + INTERVAL) + BORDER;

    commands.insert_resource(CameraBoundary {
        max_width: 960.0,
        negative: Vec2::new(-960.0 / 2.0, -height as f32 + 0.5 * BORDER as f32) * 2.0,
        positive: Vec2::new(960.0 / 2.0, OUTSIDE_HEIGHT as f32) * 2.0,
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
            transform: transform_2d_tile_m(
                IVec2::new(-width - 5, 0),
                IVec2::new(width * 2 + 10, 50),
                1.0,
            ),
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
            transform: transform_2d_tile_m(
                IVec2::new(-width - 5, -height - 50),
                IVec2::new(width * 2 + 10, height + 50),
                1.0,
            ),
            ..default()
        },
        RENDER_LAYER_LIGHT1,
    ));

    // left border
    commands.spawn(solid_bundle_with_color(
        IVec2::new(-width - BORDER, -height),
        IVec2::new(BORDER, height),
    ));

    // right border
    commands.spawn(solid_bundle_with_color(
        IVec2::new(width, -height),
        IVec2::new(BORDER, height),
    ));

    // bottom border
    commands.spawn(solid_bundle_with_color(
        IVec2::new(-width, -height),
        IVec2::new(2 * width, BORDER),
    ));

    let mut path_find = PathFind::from(IVec2::new(-50, -height), IVec2::new(100, height + 30));

    for y in 0..room_number.y {
        let from_y = -(y + 1) * (LAYER_HEIGHT + INTERVAL) + 90;
        let to_y = from_y + LAYER_HEIGHT + INTERVAL;

        path_find.add_layer(IVec2::new(0, from_y), IVec2::new(100, to_y), 0, 100);
        path_find.add_layer(IVec2::new(95, from_y + 6), IVec2::new(100, to_y), 95, 100);
    }
    commands.insert_resource(path_find);

    // layers
    let wall_image = asset.load("demo/wall.png");
    for y in 0..room_number.y {
        let position_y = -(y + 1) * (LAYER_HEIGHT + INTERVAL);
        // rooms
        for x in 0..room_number.x {
            commands.spawn(sprite_bundle_tile(
                IVec2::new(-width + x * ROOM_WIDTH, position_y),
                IVec2::new(ROOM_WIDTH, LAYER_HEIGHT),
                9.0,
                wall_image.clone(),
            ));
        }

        // stair
        commands.spawn_batch(stair_bundle_pair(
            IVec2::new(width - STAIR_WIDTH, position_y),
            IVec2::new(width - 2, position_y + (LAYER_HEIGHT + INTERVAL) / 2),
        ));
        commands.spawn_batch(stair_bundle_pair(
            IVec2::new(width - 2, position_y + (LAYER_HEIGHT + INTERVAL) / 2),
            IVec2::new(width - STAIR_WIDTH, position_y + (LAYER_HEIGHT + INTERVAL)),
        ));
        commands.spawn(solid_bundle_with_color(
            IVec2::new(
                width - 1,
                position_y + (LAYER_HEIGHT + INTERVAL) / 2 - INTERVAL,
            ),
            IVec2::new(1, INTERVAL),
        ));

        // ceil
        commands.spawn(solid_bundle_with_color(
            IVec2::new(-width, position_y + LAYER_HEIGHT),
            IVec2::new(2 * width, INTERVAL),
        ));
    }
}
