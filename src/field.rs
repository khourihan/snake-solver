use macroquad::prelude::*;
#[path = "snake.rs"] mod snake;
use self::snake::{Snake};



pub struct Cell {
    x: usize,
    y: usize,
    contents: u8,
    snake_segment: isize
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y,
            contents: 0,
            snake_segment: -1
        }
    }
    pub fn is_empty(&self) -> bool {
        self.contents == 0
    }
    pub fn is_food(&self) -> bool {
        self.contents == 1
    }
    pub fn is_segment(&self) -> bool {
        self.contents == 2
    }
    pub fn is_head(&self) -> bool {
        self.contents == 3
    }
    pub fn is_snake(&self) -> bool {
        (self.contents == 2) || (self.contents == 3)
    }
    pub fn set_empty(&mut self) {
        self.contents = 0;
    }
    pub fn set_food(&mut self) {
        self.contents = 1;
    }
    pub fn set_segment(&mut self, index: usize) {
        self.contents = 2;
        self.snake_segment = index as isize;
    }
    pub fn set_head(&mut self) {
        self.contents = 3;
    }
    pub fn pos(&self) -> (isize, isize) {
        (self.x as isize, self.y as isize)
    }
    pub fn draw(&self, tilesize: f32, neighbors: Vec<bool>) {
        let x_pos = self.x as f32 * tilesize;
        let y_pos = self.y as f32 * tilesize;
        let padding = tilesize / 20.0;

        match self.contents {
            1 => {
                draw_circle(x_pos + tilesize / 2.0, y_pos + tilesize / 2.0, (tilesize - padding * 2.0) / 2.0, Color::from_rgba(224, 176, 31, 255));
            },
            2 => {
                draw_circle(x_pos + tilesize / 2.0, y_pos + tilesize / 2.0, (tilesize - padding * 2.0) / 2.0, Color::from_rgba(45, 148, 10, 255));
                for (dir, value) in neighbors.iter().enumerate() {
                    if *value {
                        let mut pos = (x_pos, y_pos);
                        let mut dims = (tilesize, tilesize);
                        match dir {
                            0 => {
                                pos.0 += padding;
                                dims.0 -= padding * 2.0;
                                dims.1 /= 2.0;
                            },
                            1 => {
                                pos.0 += padding;
                                pos.1 += tilesize / 2.0;
                                dims.0 -= padding * 2.0;
                                dims.1 /= 2.0;
                            },
                            2 => {
                                pos.1 += padding;
                                dims.1 -= padding * 2.0;
                                dims.0 /= 2.0;
                            },
                            3 => {
                                pos.1 += padding;
                                pos.0 += tilesize / 2.0;
                                dims.1 -= padding * 2.0;
                                dims.0 /= 2.0;
                            },
                            _ => {}
                        }
                        draw_rectangle(pos.0, pos.1, dims.0, dims.1, Color::from_rgba(45, 148, 10, 255));
                    }
                }
            },
            3 => {
                draw_circle(x_pos + tilesize / 2.0, y_pos + tilesize / 2.0, (tilesize - padding * 2.0) / 2.0, Color::from_rgba(150, 235, 89, 255));
                for (dir, value) in neighbors.iter().enumerate() {
                    if *value {
                        let mut pos = (x_pos, y_pos);
                        let mut dims = (tilesize, tilesize);
                        match dir {
                            0 => {
                                pos.0 += padding;
                                dims.0 -= padding * 2.0;
                                dims.1 /= 2.0;
                            },
                            1 => {
                                pos.0 += padding;
                                pos.1 += tilesize / 2.0;
                                dims.0 -= padding * 2.0;
                                dims.1 /= 2.0;
                            },
                            2 => {
                                pos.1 += padding;
                                dims.1 -= padding * 2.0;
                                dims.0 /= 2.0;
                            },
                            3 => {
                                pos.1 += padding;
                                pos.0 += tilesize / 2.0;
                                dims.1 -= padding * 2.0;
                                dims.0 /= 2.0;
                            },
                            _ => {}
                        }
                        draw_rectangle(pos.0, pos.1, dims.0, dims.1, Color::from_rgba(150, 235, 89, 255));
                    }
                }
            },
            _ => {}
        }
    }
}


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
        self.snake.update(&mut self.food);
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
