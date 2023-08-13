use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::texture::{CompressedImageFormats, ImageType},
    sprite::MaterialMesh2dBundle,
    window::{close_on_esc, PrimaryWindow},
};
use bevy_demo::*;
use bevy_rapier2d::prelude::*;

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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                close_on_esc,
                control_selected_moveable,
                update_moveable,
                day_cycle,
                update_camera_mode,
                update_camera.before(BackgroundSystems),
                time_change,
            )
                .chain(),
        )
        .insert_resource(GameDateTime {
            time_ratio: 0.1,
            ..default()
        })
        .run();
}

fn setup(
    mut commands: Commands,
    asset: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut background_materials: ResMut<Assets<BackgroundMaterial>>,
    mut sprite_light2d_materials: ResMut<Assets<Light2dSpriteMaterial>>,
) {
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
                position: Vec2::new(0.0, -324.0 / 2.0),
                offset: Vec2::new(0.0, 1.5),
                speed,
                z,
                scale: 0.5,
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
    let wall_image = load_texture("demo/wall.png");
    spwan_shelter(&mut commands, images.add(wall_image));

    let person_image = load_texture("demo/person.png");
    let person_size = person_image.texture_descriptor.size;
    let person_size = Vec2::new(person_size.width as f32, person_size.height as f32);
    commands.spawn((
        sprite_bundle(
            shelter_position(IVec2::new(3, 1), Vec2::ZERO),
            person_size,
            100.0,
            images.add(person_image),
        ),
        moveable_bundle(
            Collider::compound(vec![(
                Vec2::new(0.0, -0.5 + 0.5 * 4.0 / person_size.y),
                0.0,
                Collider::cuboid(0.5, 0.5 * 4.0 / person_size.y),
            )]),
            80.0,
        ),
    ));

    let mesh = meshes.add(Mesh::from(shape::Quad::default()));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material: sprite_light2d_materials.add(Light2dSpriteMaterial {
                color: Color::hex("ce61767f").unwrap(),
                intensity: 0.5,
                sprite: asset.load("demo/light.png"),
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(900.0, 900.0, 1.0),
                ..default()
            },
            ..default()
        },
        RENDER_LAYER_LIGHT,
    ));

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

// hacky
fn load_texture(texture_path: &str) -> Image {
    let real_path = "assets/".to_owned() + texture_path;
    let ext = std::path::Path::new(&real_path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();
    let img_bytes = std::fs::read(&real_path).unwrap();
    Image::from_buffer(
        &img_bytes,
        ImageType::Extension(ext),
        CompressedImageFormats::all(),
        true,
    )
    .unwrap()
}
