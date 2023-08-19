use bevy::prelude::*;
use petgraph::prelude::UnGraph;

use crate::{tile_coor, MoveIntendHorizontal, Moveable};

#[derive(Resource)]
pub struct PathFind {
    pub position: IVec2,
    pub size: IVec2,
    pub layers_index: Vec<usize>,
    pub layers: Vec<Layer>,
}

pub struct Layer {
    pub id: usize,
    pub x_left: u32,
    pub x_right: u32,
}

pub struct Node {
    pub layer_id: usize,
    pub x: u32,
}

#[derive(Component)]
pub struct MoveTo(pub Vec2);

impl PathFind {
    pub fn from(position: IVec2, size: IVec2) -> PathFind {
        PathFind {
            position,
            size,
            layers_index: vec![0; (size.x * size.y) as usize],
            layers: vec![],
        }
    }

    fn relative_position(&self, position: IVec2) -> IVec2 {
        (position - self.position).clamp(IVec2::ZERO, self.size - 1)
    }

    fn absoult_position(&self, position: IVec2) -> IVec2 {
        position + self.position
    }

    fn get_layer(&self, relative_position: IVec2) -> &Layer {
        let index = relative_position.x * self.size.y + relative_position.y;
        let index = self.layers_index[index as usize];
        &self.layers[index]
    }

    pub fn add_layer(&mut self, from: IVec2, to: IVec2, x_left: u32, x_right: u32) {
        for x in from.x..to.x {
            for y in from.y..to.y {
                let index = x * self.size.y + y;
                self.layers_index[index as usize] = self.layers.len();
            }
        }
        let layer = Layer {
            id: self.layers.len(),
            x_left,
            x_right,
        };
        self.layers.push(layer);
    }
}

pub fn update_move_intend(
    mut commands: Commands,
    path_find: Res<PathFind>,
    mut moveable_query: Query<(Entity, &mut Moveable, &Transform, &MoveTo)>,
) {
    for (entity, mut moveable, transform, move_to) in moveable_query.iter_mut() {
        let src = tile_coor(transform.translation.truncate());
        let dst = tile_coor(move_to.0);

        let r_src = path_find.relative_position(src);
        let r_dst = path_find.relative_position(dst);

        let layer_from = path_find.get_layer(r_src);
        let layer_to = path_find.get_layer(r_dst);

        moveable.intend_horizontal = if layer_from.id == layer_to.id {
            if src.x < dst.x {
                MoveIntendHorizontal::Right
            } else if src.x > dst.x {
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
