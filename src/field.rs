use macroquad::prelude::*;
#[path = "snake.rs"] mod snake;
use self::snake::{Snake, Cell};



pub struct Field {
    pub cells: Vec<Vec<Cell>>,
    pub grid_size: usize,
    pub snake: Snake,
    pub food: (isize, isize)
}


impl Field {
    pub fn new(grid_size: usize) -> Self {
        let mut cells = Vec::new();
        for i in 0..grid_size {
            let mut row = Vec::new();
            for j in 0..grid_size {
                row.push(Cell::new(i, j));
            }
            cells.push(row);
        }

        Self {
            cells: cells,
            grid_size: grid_size,
            snake: Snake::new(grid_size),
            food: (3, 3)
        }
    }
    pub fn update(&mut self) {
        self.snake.input();
        self.snake.update(&mut self.food, &self.cells);
    }
    fn get_neighbors(&self, cell: &Cell) -> Vec<bool> {
        let mut neighbors: Vec<bool> = Vec::new();

        let range = ((cell.snake_segment - 1).max(0) as usize)..=(((cell.snake_segment + 1) as usize).min(self.snake.segments.len() as usize - 1) as usize);
        if (cell.y > 0) && self.cells[cell.x][cell.y - 1].is_snake() &&
        self.snake.segments[range.clone()].contains(&self.cells[cell.x][cell.y - 1].pos()) {
            neighbors.push(true);
        }
        else {
            neighbors.push(false);
        }
        if (cell.y < self.grid_size - 1) && self.cells[cell.x][cell.y + 1].is_snake() &&
        self.snake.segments[range.clone()].contains(&self.cells[cell.x][cell.y + 1].pos()) {
            neighbors.push(true);
        }
        else {
            neighbors.push(false);
        }
        if (cell.x > 0) && self.cells[cell.x - 1][cell.y].is_snake() &&
        self.snake.segments[range.clone()].contains(&self.cells[cell.x - 1][cell.y].pos()) {
            neighbors.push(true);
        }
        else {
            neighbors.push(false);
        }
        if (cell.x < self.grid_size - 1) && self.cells[cell.x + 1][cell.y].is_snake() &&
        self.snake.segments[range.clone()].contains(&self.cells[cell.x + 1][cell.y].pos()) {
            neighbors.push(true);
        }
        else {
            neighbors.push(false);
        }
        
        return neighbors;
    }
    pub fn draw(&mut self, tilesize: f32) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                cell.set_empty();
                cell.snake_segment = 0;
            }
        }
        for (i, (x, y)) in self.snake.segments.iter().enumerate() {
            self.cells[*x as usize][*y as usize].set_segment(i);
        }
        self.cells[self.snake.x as usize][self.snake.y as usize].set_head();
        self.cells[self.food.0 as usize][self.food.1 as usize].set_food();

        for row in self.cells.iter() {
            for cell in row.iter() {
                cell.draw(tilesize, self.get_neighbors(cell));
            }
        }
    }
}
