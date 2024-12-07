use bevy::prelude::*;
use random::RandomWalkSolver;

use crate::{arena::{Arena, Direction}, snake::Snake};

mod random;
mod astar;

pub trait Solver {
    fn get_direction(&mut self, snake: &mut Snake, arena: &Arena) -> Direction;
}

#[derive(Resource, Debug, Clone)]
pub enum SnakeSolver {
    RandomWalk(RandomWalkSolver),
    // Astar(AstarSolver),
}

impl Solver for SnakeSolver {
    fn get_direction(&mut self, snake: &mut Snake, arena: &Arena) -> Direction {
        match self {
            SnakeSolver::RandomWalk(s) => s.get_direction(snake, arena),
            // SnakeSolver::Astar(s) => s.get_direction(snake, arena),
        }
    }
}

impl Default for SnakeSolver {
    fn default() -> Self {
        Self::RandomWalk(RandomWalkSolver)
    }
}
