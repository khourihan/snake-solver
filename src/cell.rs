use macroquad::prelude::{draw_circle, draw_rectangle, Color};



pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub contents: u8,
    pub snake_segment: isize
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