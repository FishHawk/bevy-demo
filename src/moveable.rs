use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const GROUP_MOVEABLE: Group = Group::GROUP_1;
pub const GROUP_MOVEABLE_IN_STAIR: Group = Group::GROUP_2;
pub const GROUP_SOLID: Group = Group::GROUP_3;
pub const GROUP_STAIR: Group = Group::GROUP_4;

// Component
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum MoveIntendHorizontal {
    #[default]
    None = 0,
    Left = -1,
    Right = 1,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum MoveIntendVertical {
    #[default]
    None,
    Up,
    Down,
}

#[derive(Debug, Default)]
pub enum MoveMode {
    #[default]
    Normal,
    InStair {
        start: Vec2,
        direction: Vec2,
    },
}

#[derive(Component, Default)]
pub struct Moveable {
    pub speed: f32,
    pub intend_horizontal: MoveIntendHorizontal,
    pub intend_vertical: MoveIntendVertical,
    pub mode: MoveMode,
}

pub type MoveableBundle = (
    Moveable,
    GravityScale,
    RigidBody,
    Collider,
    LockedAxes,
    CollisionGroups,
);
pub fn moveable_bundle(collider: Collider, speed: f32) -> MoveableBundle {
    (
        Moveable { speed, ..default() },
        GravityScale(1.0),
        RigidBody::Dynamic,
        collider,
        LockedAxes::ROTATION_LOCKED,
        CollisionGroups::new(GROUP_MOVEABLE, GROUP_SOLID | GROUP_STAIR),
    )
}

pub type SolidBundle = (Collider, CollisionGroups);
pub fn solid_bundle() -> SolidBundle {
    (
        Collider::compound(vec![(Vec2::new(0.5, 0.5), 0.0, Collider::cuboid(0.5, 0.5))]),
        CollisionGroups::new(GROUP_SOLID, GROUP_MOVEABLE),
    )
}

#[derive(Component, Default)]
pub struct Stair(Vec2);

pub type StairBundle = (Stair, Sensor, Collider, CollisionGroups);
pub fn stair_bundle(stair: Vec2) -> StairBundle {
    (
        Stair(stair),
        Sensor,
        Collider::compound(vec![(Vec2::new(0.5, 0.5), 0.0, Collider::cuboid(0.5, 0.5))]),
        CollisionGroups::new(GROUP_STAIR, GROUP_MOVEABLE),
    )
}

#[derive(Debug, PartialEq)]
pub enum MoveIntendStair {
    None,
    Near,
    Far,
}

pub fn update_moveable(
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
    mut moveable_query: Query<(
        Entity,
        &mut Moveable,
        &mut GravityScale,
        &mut CollisionGroups,
        &mut Transform,
    )>,
    stair_query: Query<(Entity, &Stair, &Transform), Without<Moveable>>,
) {
    fn intend_horizontal_to_direction_x(intend_horizontal: MoveIntendHorizontal) -> f32 {
        match intend_horizontal {
            MoveIntendHorizontal::None => 0.0,
            MoveIntendHorizontal::Left => -1.0,
            MoveIntendHorizontal::Right => 1.0,
        }
    }

    for (
        moveable_entity,
        mut moveable,
        mut moveable_gravity,
        mut moveable_groups,
        mut moveable_transform,
    ) in moveable_query.iter_mut()
    {
        match moveable.mode {
            MoveMode::Normal => {
                let mut stair_bundle: Option<(Vec2, f32)> = None;
                for (stair_entity, stair, stair_transform) in stair_query.iter() {
                    if rapier_context.intersection_pair(moveable_entity, stair_entity) == Some(true)
                    {
                        let stair_direction = stair.0;
                        let stair_enable = match moveable.intend_vertical {
                            MoveIntendVertical::None => false,
                            MoveIntendVertical::Up => stair_direction.y > 0.0,
                            MoveIntendVertical::Down => stair_direction.y < 0.0,
                        };
                        if stair_enable {
                            stair_bundle = Some((stair_direction, stair_transform.translation.x));
                            break;
                        }
                    }
                }
                match stair_bundle {
                    Some((stair_direction, stair_x)) => {
                        let mut direction_x =
                            intend_horizontal_to_direction_x(moveable.intend_horizontal);

                        let moveable_x = moveable_transform.translation.x;
                        let direction_to_stair = if moveable_x < stair_x { 1.0 } else { -1.0 };
                        direction_x += direction_to_stair;
                        direction_x = direction_x.clamp(-1.0, 1.0);

                        let movement = direction_x * moveable.speed * time.delta_seconds();
                        moveable_transform.translation.x += movement;

                        if false
                            || moveable_x < stair_x && moveable_transform.translation.x >= stair_x
                            || moveable_x > stair_x && moveable_transform.translation.x <= stair_x
                        {
                            let start = Vec2::new(stair_x, moveable_transform.translation.y);
                            moveable_transform.translation.x = start.x;
                            moveable_transform.translation.y = start.y;
                            moveable.mode = MoveMode::InStair {
                                start,
                                direction: stair_direction,
                            };
                            moveable_gravity.0 = 0.0;
                            moveable_groups.memberships = GROUP_MOVEABLE_IN_STAIR;
                        }
                    }
                    None => {
                        let direction_x =
                            intend_horizontal_to_direction_x(moveable.intend_horizontal);
                        let movement = direction_x * moveable.speed * time.delta_seconds();
                        moveable_transform.translation.x += movement;
                    }
                }
            }
            MoveMode::InStair { start, direction } => {
                let follow_1 = intend_horizontal_to_direction_x(moveable.intend_horizontal)
                    * if direction.x > 0.0 { 1.0 } else { -1.0 };
                let follow_2 = match moveable.intend_vertical {
                    MoveIntendVertical::None => 0.0,
                    MoveIntendVertical::Up => 1.0,
                    MoveIntendVertical::Down => -1.0,
                } * if direction.y > 0.0 { 1.0 } else { -1.0 };
                let movement = (follow_1 + follow_2).clamp(-1.0, 1.0)
                    * direction.normalize()
                    * moveable.speed
                    * time.delta_seconds();
                let position = moveable_transform.translation.truncate();
                let position = position + movement;

                let position_relative = (position - start) / direction;
                moveable_transform.translation.x += movement.x;
                let mut out_stair = false;
                if position_relative.x < 0.0 || position_relative.y < 0.0 {
                    moveable_transform.translation.y = start.y;
                    out_stair = true;
                } else if position_relative.x > 1.0 || position_relative.y > 1.0 {
                    moveable_transform.translation.y = start.y + direction.y;
                    out_stair = true;
                } else {
                    moveable_transform.translation.y += movement.y;
                }
                if out_stair {
                    moveable.mode = MoveMode::Normal;
                    moveable_gravity.0 = 1.0;
                    moveable_groups.memberships = GROUP_MOVEABLE;
                }
            }
        }
    }
}
