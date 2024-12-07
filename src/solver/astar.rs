use crate::{arena::{Arena, Direction}, snake::Snake};

use super::Solver;

#[derive(Debug, Clone)]
pub struct Astar;

impl Solver for Astar {
    fn get_direction(&mut self, snake: &mut Snake, arena: &Arena) -> Direction {
        todo!()
    }
}
