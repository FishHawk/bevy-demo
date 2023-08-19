use bevy::prelude::*;

pub const TILE_SIZE: f32 = 10.0;

pub fn tile_coor(vector: Vec2) -> IVec2 {
    (vector / TILE_SIZE).as_ivec2()
}

pub fn world_coor(vector: IVec2) -> Vec2 {
    (vector * (TILE_SIZE as i32)).as_vec2()
}

pub fn transform_2d_m(position: Vec2, size: Vec2, z: f32) -> Transform {
    Transform {
        translation: position.extend(z),
        scale: size.extend(1.),
        ..default()
    }
}

pub fn transform_2d(position: Vec2, size: Vec2, z: f32) -> Transform {
    Transform {
        translation: (position + size / 2.0).extend(z),
        scale: size.extend(1.),
        ..default()
    }
}

pub fn transform_2d_tile_m(position: IVec2, size: IVec2, z: f32) -> Transform {
    transform_2d_m(world_coor(position), world_coor(size), z)
}

pub fn transform_2d_tile(position: IVec2, size: IVec2, z: f32) -> Transform {
    transform_2d(world_coor(position), world_coor(size), z)
}

pub fn transform_bundle(position: Vec2, size: Vec2, z: f32) -> TransformBundle {
    TransformBundle {
        local: transform_2d(position, size, z),
        ..default()
    }
}

pub fn transform_bundle_tile(position: IVec2, size: IVec2, z: f32) -> TransformBundle {
    transform_bundle(world_coor(position), world_coor(size), z)
}

pub fn spatial_bundle(position: Vec2, size: Vec2, z: f32) -> SpatialBundle {
    SpatialBundle {
        transform: transform_2d(position, size, z),
        ..default()
    }
}

pub fn spatial_bundle_tile(position: IVec2, size: IVec2, z: f32) -> SpatialBundle {
    spatial_bundle(world_coor(position), world_coor(size), z)
}

pub fn pure_color_bundle(position: Vec2, size: Vec2, z: f32, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite { color, ..default() },
        transform: transform_2d(position, size, z),
        ..default()
    }
}

pub fn pure_color_bundle_tile(position: IVec2, size: IVec2, z: f32, color: Color) -> SpriteBundle {
    pure_color_bundle(world_coor(position), world_coor(size), z, color)
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

pub fn sprite_bundle_tile(
    position: IVec2,
    size: IVec2,
    z: f32,
    texture: Handle<Image>,
) -> SpriteBundle {
    sprite_bundle(world_coor(position), world_coor(size), z, texture)
}
