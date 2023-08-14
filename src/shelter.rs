use bevy::prelude::*;

use crate::{solid_bundle, stair_bundle, SolidBundle, StairBundle, CameraBoundary, CameraMode};

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

pub fn spwan_shelter(commands: &mut Commands, room_texture: Handle<Image>) {
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
    for y in 0..room_number.y {
        let position_y = -(y as f32 + 1.0) * (LAYER_HEIGHT + INTERVAL);
        // rooms
        for x in 0..room_number.x {
            commands.spawn(sprite_bundle(
                Vec2::new(-width + (x as f32) * ROOM_WIDTH, position_y),
                Vec2::new(ROOM_WIDTH, LAYER_HEIGHT),
                9.0,
                room_texture.clone(),
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
