use bevy::math::UVec2;

use crate::{arena::{Arena, Direction}, snake::Snake};

use super::{pathfinding::{longest_path, shortest_path}, SolveMethod};

#[derive(Debug, Clone, Default)]
pub struct HamiltonSolver {
    size: UVec2,
    cycle: Vec<CycleCell>,
    head: UVec2,
    shortest_path: Option<Vec<Direction>>,
}

impl SolveMethod for HamiltonSolver {
    fn initialize(&mut self, snake: &Snake, arena: &Arena) {
        self.size = arena.size;
        self.cycle = vec![CycleCell::default(); (arena.size.x * arena.size.y) as usize];
        let width = arena.size.x;

        let path = shortest_path(arena.head, arena.behind, snake.direction, &arena.adjacencies)
            .and_then(|path| longest_path(arena.head, &arena.adjacencies, path))
            .unwrap(); // Cannot fail; the initial state always has a valid path to tail.

        let mut current = arena.head;
        let mut count = 0;

        for direction in path {
            self.cycle[(current.y * width + current.x) as usize].index = count;
            self.cycle[(current.y * width + current.x) as usize].direction = direction;
            current = current + direction;
            count += 1;
        }

        // Process snake segments
        current = arena.behind;
        for _ in 0..snake.length + 1 {
            self.cycle[(current.y * width + current.x) as usize].index = count;
            self.cycle[(current.y * width + current.x) as usize].direction = snake.direction;
            current = current + snake.direction;
            count += 1;
        }
    }

    fn get_direction(&mut self, snake: &Snake, arena: &Arena) -> Direction {
        let mut dir = self.cycle[(arena.head.y * self.size.x + arena.head.x) as usize].direction;
        let food_pos = arena.food.unwrap();
        self.shortest_path = None;
        self.head = arena.head;

        let dist = |p1: usize, mut p2: usize| {
            if p1 > p2 {
                p2 += (self.size.x * self.size.y) as usize;
            }
            p2 - p1
        };

        if (snake.length as u32) < arena.size.x * arena.size.y / 2 {
            if let Some(shortest) = shortest_path(arena.head, arena.food.unwrap(), snake.direction, &arena.adjacencies) {
                let next_pos = arena.head + shortest[0];
                let tail = self.cycle[(arena.tail.y * self.size.x + arena.tail.x) as usize].index;
                let head = self.cycle[(arena.head.y * self.size.x + arena.head.x) as usize].index;
                let next = self.cycle[(next_pos.y * self.size.x + next_pos.x) as usize].index;
                let food = self.cycle[(food_pos.y * self.size.x + food_pos.x) as usize].index;

                if !((shortest.len() == 1) && ((food as i32 - tail as i32).abs() == 1)) {
                    let head_dist = dist(tail, head);
                    let next_dist = dist(tail, next);
                    let food_dist = dist(tail, food);

                    if (next_dist > head_dist) && (next_dist <= food_dist) {
                        dir = shortest[0];
                        self.shortest_path = Some(shortest);
                    }
                }
            }
        }

        dir
    }

    fn debug_paths(&self, _arena: &Arena) -> Vec<(UVec2, Option<&[Direction]>)> {
        vec![(self.head, self.shortest_path.as_deref())]
    }

    fn debug_tables(&self, _arena: &Arena) -> Vec<Option<&[CycleCell]>> {
        vec![None, Some(&self.cycle)]
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CycleCell {
    pub index: usize,
    pub direction: Direction,
}
