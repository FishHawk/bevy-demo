use bevy::{input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};

use crate::{moveable_bundle, solid_bundle, MoveableBundle, SolidBundle};

#[derive(Default)]
pub enum CameraMode {
    #[default]
    Free,
    Follow(Entity),
}

#[derive(Resource, Default)]
pub struct CameraBoundary {
    pub real_resolution: Vec2,
    pub negative: Vec2,
    pub positive: Vec2,
    pub scale_level: i32,
    pub mode: CameraMode,
}

fn sprite_placeholder(position: Vec2, size: Vec2, z: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite { color, ..default() },
        transform: Transform {
            translation: (position - size / 2.0).extend(z),
            scale: size.extend(1.),
            ..default()
        },
        ..default()
    }
}

fn person_bundle(position: Vec2, size: Vec2) -> (SpriteBundle, MoveableBundle) {
    (
        sprite_placeholder(position, size, 9.5, Color::RED),
        moveable_bundle(30.0),
    )
}

fn dirt_bundle(position: Vec2, size: Vec2) -> (SpriteBundle, SolidBundle) {
    (
        sprite_placeholder(position, size, 10.0, Color::BLACK),
        solid_bundle(),
    )
}

fn room_bundle(position: Vec2, size: Vec2) -> SpriteBundle {
    sprite_placeholder(position, size, 9.0, Color::GRAY)
}

pub fn spwan_shelter(commands: &mut Commands) {
    const BORDER: f32 = 40.0;
    const INTERVAL: f32 = 5.0;

    const STAIR_WIDTH: f32 = 60.0;
    const ROOM_WIDTH: f32 = 80.0;
    const LAYER_HEIGHT: f32 = 80.0;

    let room_number: UVec2 = UVec2::new(7, 5);

    let width = (ROOM_WIDTH * room_number.x as f32 + STAIR_WIDTH) / 2.0;
    let height = (room_number.y as f32) * (LAYER_HEIGHT + INTERVAL) + BORDER;

    commands.insert_resource(CameraBoundary {
        real_resolution: Vec2::new(640.0, 360.0),
        negative: Vec2::new(-640.0 / 2.0, -height + 0.5 * BORDER),
        positive: Vec2::new(640.0 / 2.0, 250.0),
        scale_level: 1,
        mode: CameraMode::Free,
    });

    commands.spawn(person_bundle(Vec2::new(0.0, 20.0), Vec2::new(10.0, 10.0)));

    // left border
    commands.spawn(dirt_bundle(
        Vec2::new(-width, 0.0),
        Vec2::new(BORDER, height),
    ));

    // right border
    commands.spawn(dirt_bundle(
        Vec2::new(width + BORDER, 0.0),
        Vec2::new(BORDER, height),
    ));

    // bottom border
    commands.spawn(dirt_bundle(
        Vec2::new(width, -height + BORDER),
        Vec2::new(2.0 * width, BORDER),
    ));

    // layers
    for y in 0..room_number.y {
        for x in 0..room_number.x {
            commands.spawn(room_bundle(
                Vec2::new(
                    width - (x as f32) * ROOM_WIDTH,
                    -(y as f32) * (LAYER_HEIGHT + INTERVAL) - INTERVAL,
                ),
                Vec2::new(ROOM_WIDTH, LAYER_HEIGHT),
            ));
        }

        commands.spawn(dirt_bundle(
            Vec2::new(width, -(y as f32) * (LAYER_HEIGHT + INTERVAL)),
            Vec2::new(2.0 * width, INTERVAL),
        ));
    }
}

pub fn update_camera(
    time: Res<Time>,
    mut boundary: ResMut<CameraBoundary>,
    keyboard_input: Res<Input<KeyCode>>,
    mut wheel_events: EventReader<MouseWheel>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    moveable_query: Query<&Transform, Without<Camera2d>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let window = window_query.single();

    {
        for ev in wheel_events.iter() {
            if ev.y > 0.0 {
                boundary.scale_level += 1;
            } else {
                boundary.scale_level -= 1;
            }
        }
        boundary.scale_level = boundary.scale_level.clamp(1, 3);

        let scale = boundary.real_resolution.x / window.width();
        let scale = scale / boundary.scale_level as f32;
        camera_transform.scale.x = scale;
        camera_transform.scale.y = scale;
    }

    {
        let camera_center = match boundary.mode {
            CameraMode::Free => {
                let mut x_direction = 0.0;
                let mut y_direction = 0.0;
                if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                    x_direction -= 1.;
                }
                if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                    x_direction += 1.;
                }
                if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                    y_direction += 1.;
                }
                if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                    y_direction -= 1.;
                }

                const CAMERA_SPEED: f32 = 300.0;
                camera_transform.translation.truncate()
                    + Vec2::new(x_direction, y_direction) * CAMERA_SPEED * time.delta_seconds()
            }
            CameraMode::Follow(entity) => match moveable_query.get(entity) {
                Ok(moveable_transform) => moveable_transform.translation.truncate(),
                Err(_) => camera_transform.translation.truncate(),
            },
        };

        let camera_size =
            Vec2::new(window.width(), window.height()) * camera_transform.scale.truncate();

        let negative = camera_center - camera_size / 2.0;
        let positive = camera_center + camera_size / 2.0;
        let camera_center = camera_center
            + (boundary.negative - negative).max(Vec2::ZERO)
            + (boundary.positive - positive).min(Vec2::ZERO);
        camera_transform.translation.x = camera_center.x;
        camera_transform.translation.y = camera_center.y;
    }
}
