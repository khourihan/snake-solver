use rand::seq::IteratorRandom;

use crate::{arena::{Arena, Direction}, snake::Snake};

use super::Solver;

#[derive(Debug, Clone)]
pub struct RandomWalkSolver;

impl Solver for RandomWalkSolver {
    fn get_direction(&mut self, snake: &mut Snake, _arena: &Arena) -> Direction {
        let mut rng = rand::thread_rng();

        [
            snake.direction,
            snake.direction.rotate_clockwise(),
            snake.direction.rotate_counterclockwise(),
        ]
            .into_iter().choose(&mut rng).unwrap()
    }
}
