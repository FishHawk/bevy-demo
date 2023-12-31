use std::collections::HashSet;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{MainCamera, GROUP_SELECTABLE, GROUP_SELECTABLE_SENSOR};

#[derive(Resource, Default)]
pub struct WorldCursor {
    pub position: Vec2,
    pub entities_below: HashSet<Entity>,
}

pub type SelectableBundle = (Sensor, Collider, CollisionGroups);

pub fn selectable_bundle() -> SelectableBundle {
    (
        Sensor,
        Collider::cuboid(0.5, 0.5),
        CollisionGroups::new(GROUP_SELECTABLE, GROUP_SELECTABLE_SENSOR),
    )
}

pub fn update_world_cursor(
    mut world_cursor: ResMut<WorldCursor>,
    rapier_context: Res<RapierContext>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_position) = windows_query
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        world_cursor.position = cursor_position;
        world_cursor.entities_below.clear();
        rapier_context.intersections_with_point(
            cursor_position,
            QueryFilter::from(CollisionGroups::new(
                GROUP_SELECTABLE_SENSOR,
                GROUP_SELECTABLE,
            )),
            |entity| {
                world_cursor.entities_below.insert(entity);
                true
            },
        );
    }
}
