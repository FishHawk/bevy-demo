use bevy::{
    prelude::*,
    render::{
        render_resource::{AddressMode, FilterMode, SamplerDescriptor},
        texture::{CompressedImageFormats, ImageSampler, ImageType},
    },
    sprite::MaterialMesh2dBundle,
};

use crate::{
    moveable_bundle, selectable_bundle, spatial_bundle_tile, transform_2d, MoveTo, OutlineMaterial,
    WorldCursor, OUTLINE_MATERIAL_MESH_HANDLE, TILE_SIZE,
};

#[derive(Resource)]
pub struct SelectedPerson(pub Entity);
#[derive(Component)]
pub struct Person;

pub fn spawn_person(
    commands: &mut Commands,
    images: &mut Assets<Image>,
    outline_materials: &mut Assets<OutlineMaterial>,
    position: IVec2,
) -> Entity {
    let person_image = load_texture("demo/person.png");
    let person_size = person_image.texture_descriptor.size;
    let person_size = Vec2::new(person_size.width as f32, person_size.height as f32);
    commands
        .spawn((
            spatial_bundle_tile(position, IVec2::ONE, 100.0),
            moveable_bundle(80.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                MaterialMesh2dBundle {
                    mesh: OUTLINE_MATERIAL_MESH_HANDLE.typed().into(),
                    material: outline_materials.add(OutlineMaterial {
                        color: Color::WHITE,
                        line_width: 0,
                        texture: images.add(person_image),
                    }),
                    transform: transform_2d(-Vec2::new(0.5, 0.5), person_size / TILE_SIZE, 100.0),
                    ..default()
                },
                selectable_bundle(),
                Person,
            ));
        })
        .id()
}

pub fn select_person(
    mut commands: Commands,
    world_cursor: Res<WorldCursor>,
    buttons: Res<Input<MouseButton>>,
    mut selected_person: ResMut<SelectedPerson>,
    mut materials: ResMut<Assets<OutlineMaterial>>,
    person_query: Query<(Entity, &Parent, &Handle<OutlineMaterial>), With<Person>>,
) {
    let mut person_under_cursor: Option<Entity> = None;
    for (person, parent, handle) in person_query.iter() {
        let under_cursor = world_cursor.entities_below.contains(&person);
        if let Some(material) = materials.get_mut(handle) {
            material.line_width = if under_cursor { 1 } else { 0 };
        }
        if under_cursor {
            person_under_cursor = Some(parent.get());
        }
    }

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(person) = person_under_cursor {
            selected_person.0 = person;
        } else {
            commands
                .entity(selected_person.0)
                .insert(MoveTo(world_cursor.position));
        }
    }
}

// pub fn control_selected_person(
//     selected_person: Res<SelectedPerson>,
//     // keyboard_input: Res<Input<KeyCode>>,
//     mut moveable_query: Query<(Entity, &mut Moveable)>,
// ) {
//     for (entity, mut moveable) in moveable_query.iter_mut() {
//         moveable.intend_horizontal = MoveIntendHorizontal::None;
//         moveable.intend_vertical = MoveIntendVertical::None;
//         if entity == selected_person.0 {
//             let mut direction_x = 0;
//             let mut direction_y = 0;
//             if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
//                 direction_x -= 1;
//             }
//             if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
//                 direction_x += 1;
//             }
//             if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
//                 direction_y += 1;
//             }
//             if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
//                 direction_y -= 1;
//             }
//             moveable.intend_horizontal = match direction_x {
//                 x if x < 0 => MoveIntendHorizontal::Left,
//                 x if x > 0 => MoveIntendHorizontal::Right,
//                 _ => MoveIntendHorizontal::None,
//             };
//             moveable.intend_vertical = match direction_y {
//                 y if y < 0 => MoveIntendVertical::Down,
//                 y if y > 0 => MoveIntendVertical::Up,
//                 _ => MoveIntendVertical::None,
//             };
//         }
//     }
// }

// hacky
fn load_texture(texture_path: &str) -> Image {
    let real_path = "assets/".to_owned() + texture_path;
    let ext = std::path::Path::new(&real_path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();
    let img_bytes = std::fs::read(&real_path).unwrap();
    let mut image = Image::from_buffer(
        &img_bytes,
        ImageType::Extension(ext),
        CompressedImageFormats::all(),
        true,
    )
    .unwrap();
    image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
        address_mode_u: AddressMode::ClampToBorder,
        address_mode_v: AddressMode::ClampToBorder,
        mag_filter: FilterMode::Nearest,
        min_filter: FilterMode::Nearest,
        mipmap_filter: FilterMode::Nearest,
        ..default()
    });
    image
}
