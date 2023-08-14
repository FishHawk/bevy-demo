use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::texture::{CompressedImageFormats, ImageType},
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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                day_cycle,
                (
                    update_background_color.before(BackgroundSystems),
                    update_ambient_light,
                ),
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                debug_toggle_global_light,
                close_on_esc,
                control_selected_moveable,
                update_moveable,
                day_cycle,
                update_camera_mode,
                update_camera.before(BackgroundSystems),
                debug_control_day_cycle,
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
    mut asset: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut background_materials: ResMut<Assets<BackgroundMaterial>>,
    mut light2d_freeform_materials: ResMut<Assets<Light2dFreeformMaterial>>,
) {
    // Spawn building
    spwan_shelter(
        &mut commands,
        &mut asset,
        &mut meshes,
        &mut images,
        &mut background_materials,
        &mut light2d_freeform_materials,
    );

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

fn debug_toggle_global_light(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Camera2d, With<LightCamera>>,
) {
    if keyboard_input.just_pressed(KeyCode::L) {
        let mut camera2d = camera_query.single_mut();
        if let ClearColorConfig::Custom(color) = camera2d.clear_color {
            if color == Color::WHITE {
                camera2d.clear_color = ClearColorConfig::Custom(Color::rgb(0.2, 0.2, 0.2));
            } else {
                camera2d.clear_color = ClearColorConfig::Custom(Color::WHITE);
            }
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
