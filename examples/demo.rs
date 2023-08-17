use bevy::{prelude::*, window::close_on_esc};
use bevy_demo::*;

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
            CollisionPlugin,
            BackgroundPlugin,
            OutlinePlugin,
            Light2dPlugin,
        ))
        .add_systems(Startup, (setup_cameras, setup_shelter))
        .add_systems(
            Update,
            (
                day_cycle,
                debug_control_day_cycle,
                (
                    update_background_color,
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
                (select_person, update_move_intend, update_camera)
                    .chain()
                    .before(BackgroundSystems)
                    .after(CollisionSystems),
            ),
        )
        .insert_resource(GameDateTime {
            time_ratio: 0.1,
            ..default()
        })
        .run();
}
