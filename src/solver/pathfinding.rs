use std::collections::BinaryHeap;

use bevy::{math::UVec2, utils::HashSet};
use indexmap::map::Entry;

use crate::{adjacencies::AdjacencyGraph, arena::Direction};

/// Computes the shortest path from `start` to `goal`, given a starting `direction`.
///
/// The result is returned as a sequence of directions to the next node, with the first element
/// being the direction from the starting node to the next node.
pub(super) fn shortest_path(
    start: UVec2,
    goal: UVec2,
    direction: Direction,
    adjacencies: &AdjacencyGraph,
) -> Option<Vec<Direction>> {
    astar(start, goal, direction, adjacencies).map(|v| v.0)
}

/// Computes the longest path from `start` by modifying the given shortest path.
pub(super) fn longest_path(
    start: UVec2,
    adjacencies: &AdjacencyGraph,
    mut path: Vec<Direction>,
) -> Option<Vec<Direction>> {
    if path.is_empty() {
        return None;
    }

    let mut current = start;
    let mut visited = HashSet::new();

    visited.insert(current);
    
    for dir in path.iter() {
        current = (current.as_ivec2() + dir.offset()).as_uvec2();
        visited.insert(current);
    }

    let mut index = 0;
    current = start;

    loop {
        let cur_dir = path[index];
        let next = (current.as_ivec2() + cur_dir.offset()).as_uvec2();

        let test_dirs = if cur_dir.is_horizontal() {
            Some([Direction::Up, Direction::Down])
        } else if cur_dir.is_vertical() {
            Some([Direction::Right, Direction::Left])
        } else {
            None
        };

        let mut extended = false;
        for (dir, offset) in test_dirs.into_iter().flat_map(|v| v.into_iter()).map(|dir| (dir, dir.offset())) {
            let cur_test = current.as_ivec2() + offset;
            let next_test = next.as_ivec2() + offset;

            if cur_test.x < 0 || cur_test.y < 0 || next_test.x < 0 || next_test.y < 0 {
                continue;
            }

            let cur_test = cur_test.as_uvec2();
            let next_test = next_test.as_uvec2();

            if adjacencies.contains(cur_test) && !visited.contains(&cur_test)
                && adjacencies.contains(next_test) && !visited.contains(&next_test)
            {
                visited.insert(cur_test);
                visited.insert(next_test);
                path.insert(index, dir);
                path.insert(index + 2, dir.flip());
                extended = true;
                break;
            }
        }

        if !extended {
            current = next;
            index += 1;
            if index >= path.len() {
                break;
            }
        }
    }

    Some(path)
}

pub(super) fn astar(
    start: UVec2,
    goal: UVec2,
    direction: Direction,
    adjacencies: &AdjacencyGraph,
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

            adjacencies.get_neighbors(node.0)
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
