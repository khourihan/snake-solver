use bevy::{prelude::*, utils::{HashMap, HashSet}};
use priority_queue::PriorityQueue;

use crate::{arena::{Arena, Direction}, snake::Snake};

use super::SolveMethod;

#[derive(Debug, Clone, Default)]
pub struct AstarSolver {
    pub shortest_path: Option<Vec<Direction>>,
}

impl SolveMethod for AstarSolver {
    fn get_direction(&mut self, _snake: &Snake, arena: &Arena) -> Direction {
        let shortest = shortest_path(arena.head, arena.food.unwrap(), arena);
        let dir = *shortest.last().unwrap();
        self.shortest_path = Some(shortest);

        dir
    }

    fn debug_paths(&self, arena: &Arena) -> Vec<(UVec2, &[Direction])> {
        if let Some(path) = &self.shortest_path {
            vec![(arena.head, path)]
        } else {
            Vec::new()
        }
    }
}

pub(super) fn shortest_path(
    start: UVec2,
    goal: UVec2,
    arena: &Arena,
) -> Vec<Direction> {
    let mut route = Vec::new();
    let visited = astar(start, goal, arena);

    let mut path_segment = goal;
    while let Some(&parent) = visited.get(&path_segment) {
        if parent == UVec2::MAX {
            break;
        }

        let dir = Direction::from_offset(path_segment.as_ivec2() - parent.as_ivec2()).unwrap();
        route.push(dir);
        path_segment = parent;
    }

    route
}

pub(super) fn astar(
    start: UVec2,
    goal: UVec2,
    arena: &Arena,
) -> HashMap<UVec2, UVec2> {
    let mut count = 0;
    let mut open_set = PriorityQueue::new();
    let mut visited = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    let mut open_set_hash = HashSet::new();

    open_set.push((count, start), 0);
    open_set_hash.insert(start);
    visited.insert(start, UVec2::MAX);

    for pos in arena.positions() {
        g_score.insert(pos, i32::MAX);
        f_score.insert(pos, i32::MAX);
    }

    g_score.insert(start, 0);
    f_score.insert(start, heuristic(start, goal));

    while let Some(((_, current), _)) = open_set.pop() {
        open_set_hash.remove(&current);

        if current == goal {
            break;
        }

        let neighbors = arena.adjacencies.get_neighbors(current);

        for neighbor in neighbors {
            let temp_g_score = g_score.get(&current).unwrap() + 1;

            if temp_g_score < *g_score.get(&neighbor).unwrap() {
                let f_score_neighbor = temp_g_score + heuristic(neighbor, goal);
                visited.insert(neighbor, current);
                g_score.insert(neighbor, temp_g_score);
                f_score.insert(neighbor, f_score_neighbor);

                if !open_set_hash.contains(&neighbor) {
                    count += 1;
                    open_set.push((count, neighbor), -f_score_neighbor);
                    open_set_hash.insert(neighbor);
                }
            }
        }
    }

    visited
}

/// A* heuristic function. In this case, using manhattan distance over euclidean distance.
fn heuristic(pos: UVec2, goal: UVec2) -> i32 {
    (goal.x as i32 - pos.x as i32).abs() + (goal.y as i32 - pos.y as i32).abs()
}
