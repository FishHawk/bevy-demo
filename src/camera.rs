use bevy::{input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};

use crate::MainCamera;

#[derive(Default)]
pub enum CameraMode {
    #[default]
    Free,
    Follow(Entity),
}

#[derive(Resource, Default)]
pub struct CameraBoundary {
    pub real_resolution: Vec2,
    pub negative: Vec2,
    pub positive: Vec2,
    pub scale_level: i32,
    pub mode: CameraMode,
}

pub fn update_camera(
    time: Res<Time>,
    mut boundary: ResMut<CameraBoundary>,
    keyboard_input: Res<Input<KeyCode>>,
    mut wheel_events: EventReader<MouseWheel>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    target_query: Query<&Transform, Without<MainCamera>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let window = window_query.single();

    let target_transform = if let CameraMode::Follow(target) = boundary.mode {
        target_query.get(target).ok()
    } else {
        None
    };

    {
        let mut scale_level_new = boundary.scale_level;
        for ev in wheel_events.iter() {
            if ev.y > 0.0 {
                scale_level_new += 1;
            } else {
                scale_level_new -= 1;
            }
        }
        scale_level_new = scale_level_new.clamp(1, 3);

        if scale_level_new != boundary.scale_level {
            boundary.scale_level = scale_level_new;
            let scale = boundary.real_resolution.x / window.width();
            let scale = scale / boundary.scale_level as f32;
            camera_transform.scale.x = scale;
            camera_transform.scale.y = scale;
            if let Some(target_transform) = target_transform {
                camera_transform.translation.x = target_transform.translation.x;
                camera_transform.translation.y = target_transform.translation.y;
            }
        }
    }

    {
        let camera_center = match boundary.mode {
            CameraMode::Free => {
                let mut x_direction = 0.0;
                let mut y_direction = 0.0;
                if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                    x_direction -= 1.;
                }
                if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                    x_direction += 1.;
                }
                if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                    y_direction += 1.;
                }
                if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                    y_direction -= 1.;
                }

                const CAMERA_SPEED: f32 = 300.0;
                camera_transform.translation.truncate()
                    + Vec2::new(x_direction, y_direction) * CAMERA_SPEED * time.delta_seconds()
            }
            CameraMode::Follow(_) => match target_transform {
                Some(target_transform) => {
                    const SMOOTHING: f32 = 0.9;
                    camera_transform.translation.truncate() * SMOOTHING
                        + target_transform.translation.truncate() * (1.0 - SMOOTHING)
                }
                None => {
                    println!("Can not get target transform. Set camera to free mode.");
                    boundary.mode = CameraMode::Free;
                    camera_transform.translation.truncate()
                }
            },
        };

        let camera_size =
            Vec2::new(window.width(), window.height()) * camera_transform.scale.truncate();

        let negative = camera_center - camera_size / 2.0;
        let positive = camera_center + camera_size / 2.0;
        let camera_center = camera_center
            + (boundary.negative - negative).max(Vec2::ZERO)
            + (boundary.positive - positive).min(Vec2::ZERO);
        camera_transform.translation.x = camera_center.x;
        camera_transform.translation.y = camera_center.y;
    }
}
