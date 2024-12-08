use bevy::math::UVec2;

use crate::{arena::{Arena, Direction}, snake::Snake};

use super::{astar::shortest_path, cycle::longest_path, SolveMethod};

#[derive(Debug, Clone, Default)]
pub struct HamiltonSolver {
    size: UVec2,
    cycle: Vec<CycleCell>,
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
        dir
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
