use bevy::{
    prelude::*,
    window::{close_on_esc, PrimaryWindow},
};
use bevy_demo::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Demo".into(),
                        resolution: Vec2::new(960.0, 540.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(30.0),
            RapierDebugRenderPlugin::default(),
            BackgroundPlugin,
            Light2dPlugin,
        ))
        .add_systems(Startup, (setup_cameras, setup_shelter))
        .add_systems(
            Update,
            (
                day_cycle,
                debug_control_day_cycle,
                (
                    update_background_color.before(BackgroundSystems),
                    update_ambient_light,
                    debug_toggle_global_light,
                ),
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                close_on_esc,
                control_selected_moveable,
                update_moveable,
                update_camera_mode,
                update_camera.before(BackgroundSystems),
            )
                .chain(),
        )
        .insert_resource(GameDateTime {
            time_ratio: 0.1,
            ..default()
        })
        .run();
}

fn update_camera_mode(
    rapier_context: Res<RapierContext>,
    mut boundary: ResMut<CameraBoundary>,
    buttons: Res<Input<MouseButton>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let window = windows_query.single();

        boundary.mode = CameraMode::Free;
        if let Some(click_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            rapier_context.intersections_with_point(
                click_position,
                QueryFilter::from(CollisionGroups::new(GROUP_SOLID, GROUP_MOVEABLE)),
                |entity| {
                    boundary.mode = CameraMode::Follow(entity);
                    false
                },
            );
        }
    }
}

fn control_selected_moveable(
    boundary: ResMut<CameraBoundary>,
    keyboard_input: Res<Input<KeyCode>>,
    mut moveable_query: Query<(Entity, &mut Moveable)>,
) {
    let selected_entity = match boundary.mode {
        CameraMode::Free => Entity::PLACEHOLDER,
        CameraMode::Follow(entity) => entity,
    };
    for (entity, mut moveable) in moveable_query.iter_mut() {
        moveable.intend_horizontal = MoveIntendHorizontal::None;
        moveable.intend_vertical = MoveIntendVertical::None;
        if entity == selected_entity {
            let mut direction_x = 0;
            let mut direction_y = 0;
            if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                direction_x -= 1;
            }
            if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                direction_x += 1;
            }
            if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                direction_y += 1;
            }
            if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                direction_y -= 1;
            }
            moveable.intend_horizontal = match direction_x {
                x if x < 0 => MoveIntendHorizontal::Left,
                x if x > 0 => MoveIntendHorizontal::Right,
                _ => MoveIntendHorizontal::None,
            };
            moveable.intend_vertical = match direction_y {
                y if y < 0 => MoveIntendVertical::Down,
                y if y > 0 => MoveIntendVertical::Up,
                _ => MoveIntendVertical::None,
            };
        }
    }
}
