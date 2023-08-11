use bevy::{input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

pub const GROUP_PLAYER: Group = Group::GROUP_1;
pub const GROUP_WALL: Group = Group::GROUP_2;

#[derive(Resource, Default)]
pub struct CameraBoundary {
    pub negative: Vec2,
    pub positive: Vec2,
    pub scale_level: u32,
}

fn dirt_bundle(position: Vec2, size: Vec2) -> (SpriteBundle, Collider, CollisionGroups) {
    (
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                ..default()
            },
            transform: Transform {
                translation: (position - size / 2.0).extend(10.0),
                scale: size.extend(1.),
                ..default()
            },
            ..default()
        },
        Collider::cuboid(0.5, 0.5),
        CollisionGroups::new(GROUP_WALL, GROUP_PLAYER),
    )
}

fn room_bundle(position: Vec2, size: Vec2) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            ..default()
        },
        transform: Transform {
            translation: (position - size / 2.0).extend(9.0),
            scale: size.extend(1.),
            ..default()
        },
        ..default()
    }
}

pub fn spwan_shelter(commands: &mut Commands) {
    const BLOCK_SIZE: f32 = 5.0;
    const HORIZON_PADDING: f32 = 5.0;
    const VERTICAL_PADDING: f32 = 5.0;
    const VERTICAL_INTERVAL: f32 = 1.0;

    let room_size: Vec2 = Vec2::new(40.0, 40.0);
    let room_number: UVec2 = UVec2::new(7, 5);

    let width = room_size.x * room_number.x as f32 / 2.0;
    let height = (room_number.y as f32) * (room_size.y + VERTICAL_INTERVAL) + VERTICAL_PADDING;

    commands.insert_resource(CameraBoundary {
        negative: Vec2::new(
            -width - HORIZON_PADDING / 2.0,
            -height + VERTICAL_PADDING / 2.0,
        ) * BLOCK_SIZE,
        positive: Vec2::new(width + HORIZON_PADDING / 2.0, 100.0) * BLOCK_SIZE,
        scale_level: 1,
    });

    // left dirt
    commands.spawn(dirt_bundle(
        Vec2::new(-width, 0.0) * BLOCK_SIZE,
        Vec2::new(HORIZON_PADDING, height) * BLOCK_SIZE,
    ));

    // right dirt
    commands.spawn(dirt_bundle(
        Vec2::new(width + HORIZON_PADDING, 0.0) * BLOCK_SIZE,
        Vec2::new(HORIZON_PADDING, height) * BLOCK_SIZE,
    ));

    // bottom dirt
    commands.spawn(dirt_bundle(
        Vec2::new(width, -height + VERTICAL_PADDING) * BLOCK_SIZE,
        Vec2::new(2.0 * width, VERTICAL_PADDING) * BLOCK_SIZE,
    ));

    // layers
    for y in 0..room_number.y {
        for x in 0..room_number.x {
            room_bundle(
                Vec2::new(
                    width - (x as f32) * room_size.x,
                    -(y as f32) * (room_size.y + VERTICAL_INTERVAL) + VERTICAL_INTERVAL,
                ) * BLOCK_SIZE,
                room_size,
            );
        }

        commands.spawn(dirt_bundle(
            Vec2::new(width, -(y as f32) * (room_size.y + VERTICAL_INTERVAL)) * BLOCK_SIZE,
            Vec2::new(2.0 * width, VERTICAL_INTERVAL) * BLOCK_SIZE,
        ));
    }
}

pub fn move_camera_free(
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut wheel_events: EventReader<MouseWheel>,
    mut boundary: ResMut<CameraBoundary>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
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

        let scale = (boundary.positive.x - boundary.negative.x) / window.width();
        let scale = scale / boundary.scale_level as f32;
        camera_transform.scale = Vec2::new(scale, scale).extend(1.0);
    }

    {
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
        let camera_center = camera_transform.translation.truncate()
            + Vec2::new(x_direction, y_direction) * CAMERA_SPEED * time.delta_seconds();
        let camera_size =
            Vec2::new(window.width(), window.height()) * camera_transform.scale.truncate();

        let negative = camera_center - camera_size / 2.0;
        let positive = camera_center + camera_size / 2.0;
        let camera_center = camera_center
            + (boundary.negative - negative).max(Vec2::ZERO)
            + (boundary.positive - positive).min(Vec2::ZERO);
        camera_transform.translation = camera_center.extend(camera_transform.translation.z);
    }
}
