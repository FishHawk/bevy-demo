use bevy::prelude::*;
use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::*};

use crate::{tile_coor, MoveIntendHorizontal, MoveIntendVertical, Moveable};

#[derive(Resource)]
pub struct PathFinder {
    pub position: IVec2,
    pub size: IVec2,
    pub platforms_index: Vec<usize>,
    pub platforms: Vec<Platform>,
    pub platform_to_nodes: Vec<(usize, usize)>,
    pub node_position: Vec<IVec2>,
    pub node_nav_matrix: Vec<Vec<(MoveIntendHorizontal, MoveIntendVertical)>>,
}

pub struct Platform {
    pub id: usize,
    pub span_left: usize,
    pub span_right: usize,
    pub left: i32,
    pub right: i32,
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
            platform_to_nodes: Default::default(),
            node_position: Default::default(),
            node_nav_matrix: Default::default(),
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
                left: from.x,
                right: to.x,
            };
            span_left = span_right;
            pf.platforms.push(platform);
        }
        pf.platform_to_nodes = vec![(usize::MAX, usize::MAX); span_left];

        // Build graph
        let mut graph = UnGraph::<usize, f32>::default();
        for (src, dst) in stairs {
            let src = pf.relative_position(src);
            let dst = pf.relative_position(dst);

            let mut get_node = |pos: IVec2| match pf.node_position.iter().position(|&x| x == pos) {
                Some(index) => NodeIndex::new(index),
                None => {
                    pf.node_position.push(pos);
                    graph.add_node(graph.node_count())
                }
            };
            let node_src = get_node(src);
            let node_dst = get_node(dst);
            let weight = (dst.distance_squared(src) as f32).sqrt();
            graph.add_edge(node_src, node_dst, weight);
        }

        for platform in pf.platforms.iter() {
            let nodes = pf
                .node_position
                .iter()
                .map(|it| it.clone())
                .enumerate()
                .filter(|(_index, pos)| pf.get_platform(*pos).id == platform.id)
                .sorted_unstable_by_key(|(_index, pos)| (*pos).x)
                .collect::<Vec<(usize, IVec2)>>();

            for i in 1..nodes.len() {
                let (node_src, src) = nodes[i - 1];
                let (node_dst, dst) = nodes[i];
                let weight = (dst.distance_squared(src) as f32).sqrt();
                graph.add_edge(NodeIndex::new(node_src), NodeIndex::new(node_dst), weight);
            }

            for i in 0..nodes.len() + 1 {
                let (left, node_left) = if i == 0 {
                    (platform.left, usize::MAX)
                } else {
                    let (node, pos) = nodes[i - 1];
                    (pos.x, node)
                };
                let (right, node_right) = if i == nodes.len() {
                    (platform.right, usize::MAX)
                } else {
                    let (node, pos) = nodes[i];
                    (pos.x, node)
                };
                let left = platform.span_left + (left - platform.left) as usize;
                let right = platform.span_left + (right - platform.left) as usize;
                pf.platform_to_nodes[left..right]
                    .iter_mut()
                    .for_each(|it| *it = (node_left, node_right));
                if i != 0 {
                    pf.platform_to_nodes[left] = (node_left, node_left);
                }
            }
        }

        // Build next node matrix
        let shortest_distance_matrix = (0..graph.node_count())
            .map(|i| {
                dijkstra(&graph, NodeIndex::new(i), None, |edge| *edge.weight())
                    .into_iter()
                    .sorted_by_key(|(node_index, _)| node_index.index())
                    .map(|(_, distance)| distance)
                    .collect::<Vec<f32>>()
            })
            .collect::<Vec<Vec<f32>>>();

        let mut next_step_matrix = vec![];
        for src in 0..graph.node_count() {
            let neighbors = graph
                .edges(NodeIndex::new(src))
                .map(|it| (it.target().index(), *it.weight()))
                .collect::<Vec<(usize, f32)>>();

            let mut next_step_vec = vec![];
            for dst in 0..graph.node_count() {
                if src == dst {
                    next_step_vec.push(src);
                } else {
                    let (next_step, _) = neighbors
                        .iter()
                        .min_by(|(n1, w1), (n2, w2)| {
                            let n1_cost = shortest_distance_matrix[*n1][dst] + w1;
                            let n2_cost = shortest_distance_matrix[*n2][dst] + w2;
                            n1_cost.partial_cmp(&n2_cost).unwrap()
                        })
                        .unwrap();
                    next_step_vec.push(*next_step);
                }
            }
            next_step_matrix.push(next_step_vec);
        }

        pf.node_nav_matrix =
            vec![
                vec![(MoveIntendHorizontal::None, MoveIntendVertical::None); graph.node_count()];
                graph.node_count()
            ];
        for src in 0..graph.node_count() {
            for dst in 0..graph.node_count() {
                let src_pos = pf.node_position[graph.raw_nodes().get(src).unwrap().weight];
                let dst_pos = pf.node_position[graph.raw_nodes().get(dst).unwrap().weight];
                pf.node_nav_matrix[src][dst] =
                    if pf.get_platform(src_pos).id == pf.get_platform(dst_pos).id {
                        (
                            if src_pos.x < dst_pos.x {
                                MoveIntendHorizontal::Right
                            } else {
                                MoveIntendHorizontal::Left
                            },
                            MoveIntendVertical::None,
                        )
                    } else {
                        (
                            MoveIntendHorizontal::None,
                            if src_pos.y < dst_pos.y {
                                MoveIntendVertical::Up
                            } else {
                                MoveIntendVertical::Down
                            },
                        )
                    };
            }
        }

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

    fn get_neighbor_nodes(
        &self,
        relative_position: IVec2,
    ) -> (Option<(usize, f32)>, Option<(usize, f32)>) {
        let node_with_distance = |n: usize| {
            if n == usize::MAX {
                None
            } else {
                let d = (self.node_position[n].distance_squared(relative_position) as f32).sqrt();
                Some((n, d))
            }
        };
        let platform = self.get_platform(relative_position);
        let index = platform.span_left + (relative_position.x - platform.left) as usize;
        let (n1, n2) = self.platform_to_nodes[index];
        (node_with_distance(n1), node_with_distance(n2))
    }

    fn get_intends_p2p(&self, src: Vec2, dst: Vec2) {
        let src = tile_coor(src);
        let dst = tile_coor(dst);

        let src = self.relative_position(src);
        let dst = self.relative_position(dst);

        let (sp1, sp2) = self.get_neighbor_nodes(src);
        let (dp1, dp2) = self.get_neighbor_nodes(dst);
    }

    fn measure_distance(&self, src_pair: Option<(usize, f32)>, dst_pair: Option<(usize, f32)>) {
        if let (Some((sn, sd)), Some((dn, dd))) = (src_pair, dst_pair) {
            // sd + dd + self.node_nav_matrix[sn][dn];
        }
    }
}

pub fn update_move_intend(
    mut commands: Commands,
    pf: Res<PathFinder>,
    mut moveable_query: Query<(Entity, &mut Moveable, &Transform, &MoveTo)>,
) {
    for (entity, mut moveable, transform, move_to) in moveable_query.iter_mut() {
        let src = tile_coor(transform.translation.truncate());
        let dst = tile_coor(move_to.0);

        let r_src = pf.relative_position(src);
        let r_dst = pf.relative_position(dst);

        // let (n_src_1, n_src_2) = pf.get_neighbor_nodes(r_src);
        // let (n_src_1, n_dst_2) = pf.get_neighbor_nodes(r_dst);
        // println!("{}, {}", n1, n2);

        let layer_from = pf.get_platform(r_src);
        let layer_to = pf.get_platform(r_dst);

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
