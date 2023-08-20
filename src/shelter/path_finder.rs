use bevy::{prelude::*, utils::HashMap};
use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*};

use crate::{tile_coor, MoveIntendHorizontal, MoveIntendVertical, Moveable};

#[derive(Resource)]
pub struct PathFinder {
    pub position: IVec2,
    pub size: IVec2,
    pub platforms_index: Vec<usize>,
    pub platforms: Vec<Platform>,
}

pub struct Platform {
    pub id: usize,
    pub span_left: usize,
    pub span_right: usize,
    pub offset: i32,
}

pub struct Node {
    pub layer_id: usize,
    pub x: u32,
}

#[derive(Component)]
pub struct MoveTo(pub Vec2);

impl PathFinder {
    pub fn new(
        position: IVec2,
        size: IVec2,
        platforms: Vec<(IVec2, IVec2)>,
        stairs: Vec<(IVec2, IVec2)>,
    ) -> PathFinder {
        let mut pf = PathFinder {
            position,
            size,
            platforms_index: vec![0; (size.x * size.y) as usize],
            platforms: Default::default(),
        };

        // Add platforms
        let mut span_left = 0;
        for (position, size) in platforms {
            let from = pf.relative_border(position);
            let to = pf.relative_border(position + size);
            for x in from.x..to.x {
                for y in from.y..to.y {
                    let index = x * pf.size.y + y;
                    pf.platforms_index[index as usize] = pf.platforms.len();
                }
            }
            let span_right = span_left + (to.x - from.x) as usize;
            let platform = Platform {
                id: pf.platforms.len(),
                span_left,
                span_right,
                offset: from.x,
            };
            span_left = span_right;
            pf.platforms.push(platform);
        }

        // Build graph
        let mut graph = UnGraph::<(), f32>::default();
        let mut tile_to_node = HashMap::<IVec2, NodeIndex>::default();
        for (src, dst) in stairs {
            let src = pf.relative_position(src);
            let dst = pf.relative_position(dst);

            let mut get_node = |pos: IVec2| {
                *tile_to_node
                    .entry(pos)
                    .or_insert_with(|| graph.add_node(()))
            };
            let node_src = get_node(src);
            let node_dst = get_node(dst);
            let weight = (dst.distance_squared(src) as f32).sqrt();
            graph.add_edge(node_src, node_dst, weight);
        }

        let mut nodes_on_each_platforms = vec![];
        for platform in pf.platforms.iter() {
            let nodes_on_this_platforms = tile_to_node
                .iter()
                .filter_map(|(key, value)| {
                    if pf.get_platform(*key).id == platform.id {
                        Some((key, value))
                    } else {
                        None
                    }
                })
                .sorted_unstable_by_key(|it| -it.0.x)
                .collect::<Vec<(&IVec2, &NodeIndex)>>();

            for i in 1..nodes_on_this_platforms.len() {
                let (src, node_src) = nodes_on_this_platforms[i - 1];
                let (dst, node_dst) = nodes_on_this_platforms[i];
                let weight = (dst.distance_squared(*src) as f32).sqrt();
                graph.add_edge(*node_src, *node_dst, weight);
            }
            nodes_on_each_platforms.push(nodes_on_this_platforms);
        }

        // Build next node matrix
        let shortest_distance_matrix = (0..graph.node_count())
            .map(|i| {
                dijkstra(&graph, NodeIndex::new(i), None, |edge| *edge.weight())
                    .into_iter()
                    .sorted_by_key(|it| it.0)
                    .collect::<Vec<(NodeIndex, f32)>>()
            })
            .collect::<Vec<Vec<(NodeIndex, f32)>>>();

        // Build intent index
        let total_tile = span_left;
        let tile_to_node = vec![(usize::MAX, usize::MAX); total_tile];
        let tile_to_intent =
            vec![(MoveIntendHorizontal::None, MoveIntendVertical::None); total_tile];

        pf
    }

    fn relative_position(&self, position: IVec2) -> IVec2 {
        (position - self.position).clamp(IVec2::ZERO, self.size - 1)
    }

    fn relative_border(&self, position: IVec2) -> IVec2 {
        (position - self.position).clamp(IVec2::ZERO, self.size)
    }

    fn absoult_position(&self, position: IVec2) -> IVec2 {
        position + self.position
    }

    fn get_platform(&self, relative_position: IVec2) -> &Platform {
        let index = relative_position.x * self.size.y + relative_position.y;
        let index = self.platforms_index[index as usize];
        &self.platforms[index]
    }
}

pub fn update_move_intend(
    mut commands: Commands,
    path_find: Res<PathFinder>,
    mut moveable_query: Query<(Entity, &mut Moveable, &Transform, &MoveTo)>,
) {
    for (entity, mut moveable, transform, move_to) in moveable_query.iter_mut() {
        let src = tile_coor(transform.translation.truncate());
        let dst = tile_coor(move_to.0);

        let r_src = path_find.relative_position(src);
        let r_dst = path_find.relative_position(dst);

        let layer_from = path_find.get_platform(r_src);
        let layer_to = path_find.get_platform(r_dst);

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
