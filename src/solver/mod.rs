use astar::AstarSolver;
use bevy::prelude::*;

use crate::{arena::{Arena, Direction}, snake::Snake};

mod astar;
mod cycle;

pub trait SolveMethod {
    fn get_direction(&mut self, snake: &Snake, arena: &Arena) -> Direction;

    fn debug_paths(&self, arena: &Arena) -> Vec<(UVec2, &[Direction])> {
        Vec::new()
    }
}

#[derive(Resource, Debug, Clone)]
pub enum Solver {
    Astar(AstarSolver),
}

impl Solver {
    pub fn get_direction(&mut self, snake: &Snake, arena: &Arena) -> Direction {
        match self {
            Solver::Astar(s) => s.get_direction(snake, arena),
        }
    }

    pub fn debug_paths(&self, arena: &Arena) -> Vec<(UVec2, &[Direction])> {
        match self {
            Solver::Astar(s) => s.debug_paths(arena),
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Solver::Astar(AstarSolver::default())
    }
}
