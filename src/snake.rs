use bevy::prelude::*;

use crate::{arena::{Arena, Cell, Direction, Directions}, game::LastSolverInput, solver::Solver};

#[derive(Resource)]
pub struct Snake {
    pub direction: Direction,
    pub length: usize,
    possible_directions: Directions,
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            direction: Direction::Left,
            length: 2,
            possible_directions: !Directions::RIGHT,
        }
    }
}

pub fn setup_snake(
    mut commands: Commands,
    mut arena: ResMut<Arena>,
    mut solver: ResMut<Solver>,
) {
    arena.adjacencies.reset();
    let head = arena.size / 2 - UVec2::new(1, 0);
    let mid = head + UVec2::new(1, 0);
    let tail = head + UVec2::new(2, 0);
    arena.head = head;
    arena.tail = tail;
    arena.behind = tail + UVec2::new(1, 0);

    for pos in arena.positions() {
        let cell = arena.get_cell_unchecked_mut(pos);

        if pos == head {
            *cell = Cell::SnakeHead;
            arena.adjacencies.insert(pos);
        } else if pos == mid {
            *cell = Cell::SnakeTail { distance: 1 };
            arena.adjacencies.insert_snake_segment(pos, Direction::Left);
        } else if pos == tail {
            *cell = Cell::SnakeTail { distance: 2 };
            arena.adjacencies.insert_snake_segment(pos, Direction::Left);
        } else {
            *cell = Cell::None;
            arena.adjacencies.insert(pos);
        }
    }

    let snake = Snake::default();
    solver.initialize(&snake, &arena);
    commands.insert_resource(snake);
}

pub fn setup_solver(
    mut commands: Commands,
) {
    commands.insert_resource(Solver::default());
}

pub fn update_snake_direction_human(
    keys: Res<LastSolverInput>,
    mut snake: ResMut<Snake>,
) {
    if keys.just_pressed(KeyCode::ArrowUp) && snake.possible_directions.up() {
        snake.direction = Direction::Up;
        snake.possible_directions = !Directions::DOWN;
    }

    if keys.just_pressed(KeyCode::ArrowDown) && snake.possible_directions.down() {
        snake.direction = Direction::Down;
        snake.possible_directions = !Directions::UP;
    }

    if keys.just_pressed(KeyCode::ArrowLeft) && snake.possible_directions.left() {
        snake.direction = Direction::Left;
        snake.possible_directions = !Directions::RIGHT;
    }

    if keys.just_pressed(KeyCode::ArrowRight) && snake.possible_directions.right() {
        snake.direction = Direction::Right;
        snake.possible_directions = !Directions::LEFT;
    }
}

pub fn compute_snake_direction(
    arena: Res<Arena>,
    mut snake: ResMut<Snake>,
    mut solver: ResMut<Solver>,
) {
    let direction = solver.get_direction(&snake, &arena);

    if !snake.possible_directions.contains(direction.into()) {
        warn!("snake tried to travel in illegal direction");
    }

    snake.possible_directions = !Directions::from(direction.flip());
    snake.direction = direction;
}
