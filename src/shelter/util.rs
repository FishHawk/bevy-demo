use bevy::prelude::*;

pub fn transform_2d(position: Vec2, size: Vec2, z: f32) -> Transform {
    Transform {
        translation: (position + size / 2.0).extend(z),
        scale: size.extend(1.),
        ..default()
    }
}

pub fn transform_bundle(position: Vec2, size: Vec2, z: f32) -> TransformBundle {
    TransformBundle {
        local: transform_2d(position, size, z),
        ..default()
    }
}

pub fn spatial_bundle(position: Vec2, size: Vec2, z: f32) -> SpatialBundle {
    SpatialBundle {
        transform: transform_2d(position, size, z),
        ..default()
    }
}

pub fn sprite_bundle_pure_color(position: Vec2, size: Vec2, z: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite { color, ..default() },
        transform: transform_2d(position, size, z),
        ..default()
    }
}

pub fn sprite_bundle(position: Vec2, size: Vec2, z: f32, texture: Handle<Image>) -> SpriteBundle {
    SpriteBundle {
        texture: texture,
        transform: transform_2d(position, size, z),
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        ..default()
    }
}
