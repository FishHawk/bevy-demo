use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod moveable;
mod selectable;

pub use moveable::*;
pub use selectable::*;

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct CollisionSystems;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(30.0),
            RapierDebugRenderPlugin::default(),
        ))
        .init_resource::<WorldCursor>()
        .add_systems(
            Update,
            (update_moveable, update_world_cursor).in_set(CollisionSystems),
        );
    }
}

// Groups
pub const GROUP_MOVEABLE: Group = Group::GROUP_1;
pub const GROUP_MOVEABLE_IN_STAIR: Group = Group::GROUP_2;
pub const GROUP_SOLID: Group = Group::GROUP_3;
pub const GROUP_STAIR: Group = Group::GROUP_4;

pub const GROUP_SELECTABLE: Group = Group::GROUP_5;
pub const GROUP_SELECTABLE_SENSOR: Group = Group::GROUP_5;
