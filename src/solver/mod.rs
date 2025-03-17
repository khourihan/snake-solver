use astar::AstarSolver;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use greedy::GreedySolver;
use hamilton::{CycleCell, HamiltonSolver};

use crate::{
    arena::{Arena, Direction},
    snake::Snake,
};

pub mod astar;
pub mod greedy;
pub mod hamilton;
mod pathfinding;

pub trait SolveMethod {
    fn initialize(&mut self, _snake: &Snake, _arena: &Arena) {
        // Do nothing
    }

    fn get_direction(&mut self, snake: &Snake, arena: &Arena) -> Direction;

    fn debug_paths(&self, _arena: &Arena) -> Vec<(UVec2, Option<&[Direction]>)> {
        Vec::new()
    }

    fn debug_tables(&self, _arena: &Arena) -> Vec<Option<&[CycleCell]>> {
        Vec::new()
    }

    fn debug_points(&self, _arena: &Arena) -> Vec<Option<UVec2>> {
        Vec::new()
    }
}

#[derive(Reflect, Resource, Debug, Clone, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub enum Solver {
    Astar(AstarSolver),
    Greedy(GreedySolver),
    Hamilton(HamiltonSolver),
}

impl Solver {
    pub fn initialize(&mut self, snake: &Snake, arena: &Arena) {
        match self {
            Solver::Astar(s) => s.initialize(snake, arena),
            Solver::Greedy(s) => s.initialize(snake, arena),
            Solver::Hamilton(s) => s.initialize(snake, arena),
        }
    }

    pub fn get_direction(&mut self, snake: &Snake, arena: &Arena) -> Direction {
        match self {
            Solver::Astar(s) => s.get_direction(snake, arena),
            Solver::Greedy(s) => s.get_direction(snake, arena),
            Solver::Hamilton(s) => s.get_direction(snake, arena),
        }
    }

    pub fn debug_paths(&self, arena: &Arena) -> Vec<(UVec2, Option<&[Direction]>)> {
        match self {
            Solver::Astar(s) => s.debug_paths(arena),
            Solver::Greedy(s) => s.debug_paths(arena),
            Solver::Hamilton(s) => s.debug_paths(arena),
        }
    }

    pub fn debug_tables(&self, arena: &Arena) -> Vec<Option<&[CycleCell]>> {
        match self {
            Solver::Astar(s) => s.debug_tables(arena),
            Solver::Greedy(s) => s.debug_tables(arena),
            Solver::Hamilton(s) => s.debug_tables(arena),
        }
    }

    pub fn debug_points(&self, arena: &Arena) -> Vec<Option<UVec2>> {
        match self {
            Solver::Astar(s) => s.debug_points(arena),
            Solver::Greedy(s) => s.debug_points(arena),
            Solver::Hamilton(s) => s.debug_points(arena),
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Solver::Hamilton(HamiltonSolver::default())
    }
}
