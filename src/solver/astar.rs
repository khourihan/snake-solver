use bevy::prelude::*;

use crate::{arena::{Arena, Direction}, snake::Snake};

use super::{pathfinding::shortest_path, SolveMethod};

#[derive(Debug, Clone, Default)]
pub struct AstarSolver {
    shortest_path: Option<Vec<Direction>>,
    start: UVec2,
}

impl SolveMethod for AstarSolver {
    fn get_direction(&mut self, snake: &Snake, arena: &Arena) -> Direction {
        let shortest = shortest_path(arena.head, arena.food.unwrap(), snake.direction, &arena.adjacencies);

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

    fn debug_paths(&self, _arena: &Arena) -> Vec<(UVec2, Option<&[Direction]>)> {
        if let Some(path) = &self.shortest_path {
            vec![(self.start, Some(path))]
        } else {
            Vec::new()
        }
    }
}

