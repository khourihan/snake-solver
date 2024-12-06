use bevy::prelude::*;

use crate::{arena::{Arena, Cell, Direction, Directions}, game::LastSolverInput};

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
            length: 1,
            possible_directions: !Directions::RIGHT,
        }
    }
}

pub fn setup_snake(
    mut commands: Commands,
    mut arena: ResMut<Arena>,
) {
    commands.insert_resource(Snake::default());

    let head = arena.size / 2;
    let tail = head + UVec2::new(1, 0);
    for (pos, cell) in arena.cells_mut() {
        if pos == head {
            *cell = Cell::SnakeHead;
        } else if pos == tail {
            *cell = Cell::SnakeTail { distance: 1 };
        } else {
            *cell = Cell::None;
        }
    }
}

pub fn update_snake_direction(
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
