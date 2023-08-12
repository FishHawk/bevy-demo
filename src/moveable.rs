use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const GROUP_MOVEABLE: Group = Group::GROUP_1;
pub const GROUP_SOLID: Group = Group::GROUP_2;
pub const GROUP_LADDER: Group = Group::GROUP_3;

// Component
#[derive(Debug, Default)]
pub enum MoveIntendHorizontal {
    #[default]
    None,
    Left,
    Right,
}

#[derive(Debug, Default)]
pub enum MoveIntendVertical {
    #[default]
    None,
    Up,
    Down,
}

#[derive(Component, Default)]
pub struct Moveable {
    pub speed: f32,
    pub intend_horizontal: MoveIntendHorizontal,
    pub intend_vertical: MoveIntendVertical,
}

pub type MoveableBundle = (Moveable, RigidBody, Collider, LockedAxes, CollisionGroups);
pub fn moveable_bundle(speed: f32) -> MoveableBundle {
    (
        Moveable { speed, ..default() },
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5),
        LockedAxes::ROTATION_LOCKED,
        CollisionGroups::new(GROUP_MOVEABLE, Group::ALL),
    )
}

pub type SolidBundle = (Collider, CollisionGroups);
pub fn solid_bundle() -> SolidBundle {
    (
        Collider::cuboid(0.5, 0.5),
        CollisionGroups::new(GROUP_SOLID, GROUP_MOVEABLE),
    )
}

pub fn update_moveable(time: Res<Time>, mut moveable_query: Query<(&mut Transform, &Moveable)>) {
    for (mut transform, moveable) in moveable_query.iter_mut() {
        let direction_x = match moveable.intend_horizontal {
            MoveIntendHorizontal::None => 0.0,
            MoveIntendHorizontal::Left => -1.0,
            MoveIntendHorizontal::Right => 1.0,
        };
        let movement = direction_x * moveable.speed * time.delta_seconds();
        transform.translation.x += movement;
    }
}
