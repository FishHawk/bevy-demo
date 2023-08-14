use std::f32::consts::PI;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use crate::{BackgroundMaterial, Light2dFreeformMaterial, LightCamera};

#[derive(Resource, Default)]
pub struct GameDateTime {
    pub paused: bool,
    pub days: i32,
    pub time: f32,
    pub time_ratio: f32,
}
impl GameDateTime {
    pub fn cos(&self) -> f32 {
        let cos = (self.time * 2.0 * PI).cos();
        -cos / 2.0 + 0.5
    }
}

#[derive(Component)]
pub struct LightIntensity {
    pub max: f32,
    pub min: f32,
    pub addition: f32,
}

#[derive(Component)]
pub struct GameDateTimeText;

pub fn day_cycle(
    time: Res<Time>,
    mut game_date_time: ResMut<GameDateTime>,
    mut game_date_time_text_query: Query<&mut Text, With<GameDateTimeText>>,
) {
    if !game_date_time.paused {
        game_date_time.time += game_date_time.time_ratio * 0.1 * time.delta_seconds();
        if game_date_time.time >= 1.0 {
            game_date_time.days += 1;
            game_date_time.time = game_date_time.time.fract();
        }
    }

    for mut text in &mut game_date_time_text_query {
        text.sections[0].value = format!(
            "Day {0} Hour {1:02} {2}",
            game_date_time.days,
            game_date_time.time * 24.0,
            if game_date_time.paused { "paused" } else { "" }
        );
    }
}

pub fn update_background_color(
    game_date_time: ResMut<GameDateTime>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    mut background_query: Query<&Handle<BackgroundMaterial>>,
) {
    let ratio = game_date_time.cos();
    for material_handle in &mut background_query {
        let material = materials.get_mut(material_handle).unwrap();
        material.palette_ratio.x = ratio;
    }
}

pub fn update_ambient_light(
    game_date_time: ResMut<GameDateTime>,
    mut materials: ResMut<Assets<Light2dFreeformMaterial>>,
    light_query: Query<(&LightIntensity, &Handle<Light2dFreeformMaterial>)>,
) {
    let ratio = game_date_time.cos();
    for (intensity, material_handle) in light_query.iter() {
        let material = materials.get_mut(&material_handle).unwrap();
        material.intensity = intensity.min + ratio * intensity.max + intensity.addition;
    }
}

pub fn debug_control_day_cycle(
    mut game_date_time: ResMut<GameDateTime>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        game_date_time.paused = !game_date_time.paused;
    }
    if keyboard_input.just_pressed(KeyCode::Q) {
        game_date_time.time = ((game_date_time.time * 24.0 + 23.0).floor() / 24.0).fract();
    }
    if keyboard_input.just_pressed(KeyCode::E) {
        game_date_time.time = ((game_date_time.time * 24.0 + 1.0).floor() / 24.0).fract();
    }
}

pub fn debug_toggle_global_light(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Camera2d, With<LightCamera>>,
) {
    if keyboard_input.just_pressed(KeyCode::L) {
        let mut camera2d = camera_query.single_mut();
        if let ClearColorConfig::Custom(color) = camera2d.clear_color {
            if color == Color::WHITE {
                camera2d.clear_color = ClearColorConfig::Custom(Color::BLACK);
            } else {
                camera2d.clear_color = ClearColorConfig::Custom(Color::WHITE);
            }
        }
    }
}
