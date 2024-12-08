use astar::AstarSolver;
use greedy::GreedySolver;
use bevy::prelude::*;

use crate::{arena::{Arena, Direction}, snake::Snake};

mod astar;
mod greedy;
mod cycle;

pub trait SolveMethod {
    fn get_direction(&mut self, snake: &Snake, arena: &Arena) -> Direction;

    fn debug_paths(&self, _arena: &Arena) -> Vec<(UVec2, Option<&[Direction]>)> {
        Vec::new()
    }

    fn debug_points(&self, _arena: &Arena) -> Vec<Option<UVec2>> {
        Vec::new()
    }
}

#[derive(Resource, Debug, Clone)]
pub enum Solver {
    Astar(AstarSolver),
    Greedy(GreedySolver),
}

impl Solver {
    pub fn get_direction(&mut self, snake: &Snake, arena: &Arena) -> Direction {
        match self {
            Solver::Astar(s) => s.get_direction(snake, arena),
            Solver::Greedy(s) => s.get_direction(snake, arena),
        }
    }

    pub fn debug_paths(&self, arena: &Arena) -> Vec<(UVec2, Option<&[Direction]>)> {
        match self {
            Solver::Astar(s) => s.debug_paths(arena),
            Solver::Greedy(s) => s.debug_paths(arena),
        }
    }

    pub fn debug_points(&self, arena: &Arena) -> Vec<Option<UVec2>> {
        match self {
            Solver::Astar(s) => s.debug_points(arena),
            Solver::Greedy(s) => s.debug_points(arena),
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Solver::Greedy(GreedySolver::default())
    }
}
