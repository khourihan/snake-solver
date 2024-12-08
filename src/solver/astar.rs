use std::collections::BinaryHeap;

use bevy::prelude::*;
use indexmap::map::Entry;

use crate::{arena::{Arena, Direction}, snake::Snake};

use super::SolveMethod;

#[derive(Debug, Clone, Default)]
pub struct AstarSolver {
    shortest_path: Option<Vec<Direction>>,
    start: UVec2,
}

impl SolveMethod for AstarSolver {
    fn get_direction(&mut self, snake: &Snake, arena: &Arena) -> Direction {
        let shortest = shortest_path(arena.head, arena.food.unwrap(), snake.direction, arena);

        if let Some(shortest) = shortest {
            let dir = shortest[0];
            self.shortest_path = Some(shortest);
            self.start = arena.head;
            dir
        } else {
            warn!("no shortest path found");
            snake.direction
        }
    }

    fn debug_paths(&self, _arena: &Arena) -> Vec<(UVec2, &[Direction])> {
        if let Some(path) = &self.shortest_path {
            vec![(self.start, path)]
        } else {
            Vec::new()
        }
    }
}

/// Computes the shortest path from `start` to `goal`, given a starting `direction`.
///
/// The result is returned as a sequence of directions to the next node, with the first element
/// being the direction from the starting node to the next node.
pub(super) fn shortest_path(
    start: UVec2,
    goal: UVec2,
    direction: Direction,
    arena: &Arena,
) -> Option<Vec<Direction>> {
    astar(start, goal, direction, arena).map(|v| v.0)
}

pub(super) fn astar(
    start: UVec2,
    goal: UVec2,
    direction: Direction,
    arena: &Arena,
) -> Option<(Vec<Direction>, i32)> {
    let mut open_set = BinaryHeap::new();
    open_set.push(CostHolder {
        estimated_cost: 0,
        cost: 0,
        index: 0,
    });

    let mut parents: FxIndexMap<(UVec2, Direction), (usize, i32)> = FxIndexMap::default();
    parents.insert((start, direction), (usize::MAX, 0));

    while let Some(CostHolder { cost, index, .. }) = open_set.pop() {
        let successors = {
            let (node, &(_, c)) = parents.get_index(index).unwrap();
            if node.0 == goal {
                let path = reconstruct_path(&parents, index);
                return Some((path, cost));
            }
            
            if cost > c {
                continue;
            }

            arena.adjacencies.get_neighbors(node.0)
        };

        for successor in successors {
            let new_cost = cost + 1;
            let h;
            let n;

            match parents.entry(successor) {
                Entry::Vacant(e) => {
                    h = heuristic(e.key().0, goal);
                    n = e.index();
                    e.insert((index, new_cost));
                },
                Entry::Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        h = heuristic(e.key().0, goal);
                        n = e.index();
                        e.insert((index, new_cost));
                    } else {
                        continue;
                    }
                },
            }

            open_set.push(CostHolder {
                estimated_cost: new_cost + h,
                cost: new_cost,
                index: n,
            });
        }
    }

    None
}

fn reconstruct_path(parents: &FxIndexMap<(UVec2, Direction), (usize, i32)>, mut i: usize) -> Vec<Direction> {
    let path = std::iter::from_fn(|| {
        parents.get_index(i).map(|(node, value)| {
            i = value.0;
            node
        })
    })
        .map(|(_pos, dir)| *dir)
        .collect::<Vec<Direction>>();

    path.into_iter().rev().skip(1).collect()
}

struct CostHolder<C> {
    estimated_cost: C,
    cost: C,
    index: usize,
}

impl<C: PartialEq> PartialEq for CostHolder<C> {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
    }
}

impl<C: PartialEq> Eq for CostHolder<C> {}

impl<C: Ord> Ord for CostHolder<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.estimated_cost.cmp(&self.estimated_cost) {
            std::cmp::Ordering::Equal => self.cost.cmp(&other.cost),
            s => s,
        }
    }
}

impl<C: Ord> PartialOrd for CostHolder<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type FxIndexMap<K, V> = indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

/// A* heuristic function. In this case, using manhattan distance over euclidean distance.
fn heuristic(pos: UVec2, goal: UVec2) -> i32 {
    (goal.x as i32 - pos.x as i32).abs() + (goal.y as i32 - pos.y as i32).abs()
}
