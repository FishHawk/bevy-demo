use bevy::prelude::*;
use petgraph::prelude::UnGraph;

use crate::{MoveIntendHorizontal, Moveable};

#[derive(Resource)]
pub struct PathFind {
    pub origin: Vec2,
    pub size: UVec2,
    pub layers_index: Vec<usize>,
    pub layers: Vec<Layer>,
}

pub struct Layer {
    pub id: u32,
    pub x_left: u32,
    pub x_right: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct MoveTo(pub Vec2);

const BLOCK_SIZE: f32 = 10.0;

impl PathFind {
    fn as_block_position(&self, position: Vec2) -> UVec2 {
        ((position - self.origin).max(Vec2::ZERO) / BLOCK_SIZE)
            .as_uvec2()
            .min(self.size - 1)
    }

    fn from_block_position(&self, position: UVec2) -> Vec2 {
        position.as_vec2() * BLOCK_SIZE + self.origin
    }

    fn get_layer(&self, block_position: UVec2) -> &Layer {
        let index = block_position.x * self.size.y + block_position.y;
        let index = self.layers_index[index as usize];
        &self.layers[index]
    }

    pub fn add_layer(&mut self, from: UVec2, to: UVec2, layer: Layer) {
        for x in from.x..to.x {
            for y in from.y..to.y {
                let index = x * self.size.y + y;
                self.layers_index[index as usize] = self.layers.len();
            }
        }
        self.layers.push(layer);
    }
}

pub fn update_move_intend(
    mut commands: Commands,
    path_find: Res<PathFind>,
    mut moveable_query: Query<(Entity, &mut Moveable, &Transform, &MoveTo)>,
    mut gizmos: Gizmos,
) {
    // let mut g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

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
                path_find.from_block_position(UVec2::new(x, y)),
                0.0,
                Vec2::splat(10.),
                colors[index % colors.len()],
            );
        }
    }

    for (entity, mut moveable, transform, move_to) in moveable_query.iter_mut() {
        let from = transform.translation.truncate();
        let to = move_to.0;

        let block_from = path_find.as_block_position(from);
        let block_to = path_find.as_block_position(to);

        let layer_from = path_find.get_layer(block_from);
        let layer_to = path_find.get_layer(block_to);

        let from = from.as_ivec2();
        let to = to.as_ivec2();

        moveable.intend_horizontal = if layer_from.id == layer_to.id {
            if from.x < to.x {
                MoveIntendHorizontal::Right
            } else if from.x > to.x {
                MoveIntendHorizontal::Left
            } else {
                commands.entity(entity).remove::<MoveTo>();
                MoveIntendHorizontal::None
            }
        } else {
            commands.entity(entity).remove::<MoveTo>();
            MoveIntendHorizontal::None
        }
    }
}
