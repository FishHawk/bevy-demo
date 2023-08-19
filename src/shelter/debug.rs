use bevy::prelude::*;
use bevy_rapier2d::render::{DebugRenderContext, RapierDebugRenderPlugin};

use crate::{world_coor, Moveable, PathFind, Stair};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierDebugRenderPlugin::default().disabled())
            .insert_resource(DebugContext {
                should_render_components: false,
                should_render_path_find: false,
            })
            .add_systems(
                Update,
                (
                    debug_render_components,
                    debug_render_path_find,
                    toggle_debug_context,
                ),
            );
    }
}

#[derive(Resource)]
struct DebugContext {
    should_render_components: bool,
    should_render_path_find: bool,
}

fn debug_render_components(
    mut gizmos: Gizmos,
    debug_context: Res<DebugContext>,
    stair_query: Query<&Transform, With<Stair>>,
    moveable_query: Query<&Transform, With<Moveable>>,
) {
    if !debug_context.should_render_components {
        return;
    }

    fn draw_gizmos<T: Component>(
        color: Color,
        gizmos: &mut Gizmos,
        query: Query<&Transform, With<T>>,
    ) {
        for transform in query.iter() {
            gizmos.rect_2d(
                transform.translation.truncate(),
                0.0,
                transform.scale.truncate(),
                color,
            );
        }
    }
    draw_gizmos(Color::BLUE, &mut gizmos, stair_query);
    draw_gizmos(Color::RED, &mut gizmos, moveable_query);
}

fn debug_render_path_find(
    mut gizmos: Gizmos,
    debug_context: Res<DebugContext>,
    path_find: Res<PathFind>,
) {
    if !debug_context.should_render_path_find {
        return;
    }

    let colors = vec![
        Color::RED,
        Color::BLUE,
        Color::GREEN,
        Color::CYAN,
        Color::YELLOW,
        Color::GOLD,
    ];
    for x in 0..path_find.size.x {
        for y in 0..path_find.size.y {
            let index = x * path_find.size.y + y;
            let index = path_find.layers_index[index as usize];
            gizmos.rect_2d(
                world_coor(IVec2::new(x, y) + path_find.position) + world_coor(IVec2::ONE) / 2.0,
                0.0,
                world_coor(IVec2::ONE),
                colors[index % colors.len()],
            );
        }
    }
}

fn toggle_debug_context(
    mut rapier_context: ResMut<DebugRenderContext>,
    mut debug_context: ResMut<DebugContext>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Key1) {
        rapier_context.enabled = !rapier_context.enabled;
    }
    if keyboard_input.just_pressed(KeyCode::Key2) {
        debug_context.should_render_components = !debug_context.should_render_components;
    }
    if keyboard_input.just_pressed(KeyCode::Key3) {
        debug_context.should_render_path_find = !debug_context.should_render_path_find;
    }
}
