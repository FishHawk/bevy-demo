use bevy::{prelude::*, window::close_on_esc};
use bevy_demo::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

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
            DebugPlugin,
        ))
        .add_plugins(EguiPlugin)
        .add_systems(Update, ui_example_system)
        .add_systems(Startup, (startup, setup_cameras, setup_shelter))
        .add_systems(
            Update,
            (
                day_cycle,
                debug_control_day_cycle,
                (
                    update_background_color,
                    // update_ambient_light,
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

#[derive(Default, Resource)]
struct UiState {
    intensity: f32,
    color: [f32; 3],
}

fn startup(mut commands: Commands) {
    commands.insert_resource(UiState {
        intensity: 0.5,
        color: [1.0, 1.0, 1.0],
    });
}

fn ui_example_system(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut materials: ResMut<Assets<Light2dFreeformMaterial>>,
    light_query: Query<&Handle<Light2dFreeformMaterial>>,
) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut ui_state.intensity, 0.0..=1.0).text("age"));
        egui::color_picker::color_edit_button_rgb(ui, &mut ui_state.color);
        ui.label("world");
    });

    for material_handle in light_query.iter() {
        let material = materials.get_mut(&material_handle).unwrap();
        material.intensity = ui_state.intensity;
        material.color = Color::rgb(ui_state.color[0], ui_state.color[1], ui_state.color[2]);
    }
}
