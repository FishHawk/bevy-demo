use std::f32::consts::PI;

use bevy::{prelude::*, window::close_on_esc};
use bevy_demo::*;

#[derive(Resource, Default)]
struct GameDateTime {
    pub paused: bool,
    pub days: i32,
    pub time: f32,
    pub time_ratio: f32,
}

#[derive(Component)]
struct GameDateTimeText;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Demo".into(),
                    resolution: (1980., 1080.).into(),
                    ..default()
                }),
                ..default()
            }),
            BackgroundPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                //
                close_on_esc,
                day_cycle,
                move_camera_free.before(BackgroundSystems),
                time_change,
            ),
        )
        .insert_resource(GameDateTime {
            time_ratio: 0.1,
            ..default()
        })
        .run();
}

fn setup(
    mut commands: Commands,
    mut background_materials: ResMut<Assets<BackgroundMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // Spawn camera
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.y = 0.0;
    commands.spawn(camera);

    // Spawn background
    let mut spawn_background = |texture_path: &str, speed: Vec2, z: f32| {
        let background_images = BackgroundMaterialImages::palette(
            &mut images,
            BackgroundRepeat::X,
            texture_path,
            "demo/lut.png",
        );
        commands.spawn(BackgroundBundle {
            material_bundle: BackgroundMaterial::bundle(
                &mut background_materials,
                background_images,
            ),
            background: Background {
                position: Vec2::new(0.0, -324.0),
                offset: Vec2::new(0.0, 1.5),
                speed,
                z,
                scale: 1.0,
                ..default()
            },
        });
    };

    spawn_background("demo/1.png", Vec2::new(0.0, 0.5), 0.1);
    spawn_background("demo/2.png", Vec2::new(0.0, 0.2), 0.2);
    spawn_background("demo/3.png", Vec2::new(0.0, 0.1), 0.3);
    spawn_background("demo/4.png", Vec2::new(0.0, 0.0), 0.4);
    spawn_background("demo/5.png", Vec2::new(0.0, 0.0), 0.5);
    spawn_background("demo/6.png", Vec2::new(0.0, 0.0), 0.6);

    // Spawn building
    spwan_shelter(&mut commands);

    // Spawn UI
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        GameDateTimeText,
    ));
}

fn day_cycle(
    time: Res<Time>,
    mut game_date_time: ResMut<GameDateTime>,
    mut game_date_time_text_query: Query<&mut Text, With<GameDateTimeText>>,
    //
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    mut background_query: Query<&Handle<BackgroundMaterial>>,
) {
    if !game_date_time.paused {
        game_date_time.time += game_date_time.time_ratio * time.delta_seconds();
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

    let ratio = (game_date_time.time * 2.0 * PI).cos() / 2.0 + 0.5;
    for material_handle in &mut background_query {
        let material = materials.get_mut(material_handle).unwrap();
        material.palette_ratio.x = ratio;
    }
}

fn time_change(mut game_date_time: ResMut<GameDateTime>, keyboard_input: Res<Input<KeyCode>>) {
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
