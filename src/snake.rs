use std::time::{SystemTime, Duration};

use macroquad::{prelude::{is_key_pressed, KeyCode}};
use rand::distributions::{Uniform, Distribution};


pub enum Directions {
    Up,
    Down,
    Left,
    Right
}



pub struct Snake {
    pub x: isize,
    pub y: isize,
    pub segments: Vec<(isize, isize)>,
    pub length: isize,
    pub score: usize,
    grid_size: usize,
    dir: (i8, i8),

    time: SystemTime,
    time_step: Duration,

    movable_dirs: [bool; 4],
    hit_body: bool,
}

impl Snake {
    pub fn new(grid_size: usize) -> Self {
        let range = Uniform::from(0..grid_size as isize);
        let mut rng = rand::thread_rng();
        let xpos = range.sample(&mut rng);
        let ypos = range.sample(&mut rng);
        Self {
            x: xpos,
            y: ypos,
            grid_size: grid_size,
            segments: Vec::from([(xpos, ypos)]),
            length: 1,
            dir: (0, 0),

            time: SystemTime::now(),
            time_step: Duration::from_millis(110),
            movable_dirs: [true; 4], // Up, Down, Left, Right
            hit_body: false,
            score: 0
        }
    }
    pub fn input(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.change_dir(Directions::Up);
        }
        else if is_key_pressed(KeyCode::Down) {
            self.change_dir(Directions::Down);
        }
        if is_key_pressed(KeyCode::Left) {
            self.change_dir(Directions::Left);
        }
        else if is_key_pressed(KeyCode::Right) {
            self.change_dir(Directions::Right);
        }
    }
    pub fn change_dir(&mut self, dir: Directions) {
        match dir {
            Directions::Up => {
                if self.movable_dirs[0] {
                    self.dir = (0, -1);
                    self.movable_dirs = [true, false, true, true];
                }
            }
            Directions::Down => {
                if self.movable_dirs[1] {
                    self.dir = (0, 1);
                    self.movable_dirs = [false, true, true, true];
                }
            }
            Directions::Left => {
                if self.movable_dirs[2] {
                    self.dir = (-1, 0);
                    self.movable_dirs = [true, true, true, false];
                }
            }
            Directions::Right => {
                if self.movable_dirs[3] {
                    self.dir = (1, 0);
                    self.movable_dirs = [true, true, false, true];
                }
            }
        }
    }
    pub fn reset(&mut self) {
        let range = Uniform::from(0..self.grid_size as isize);
        let mut rng = rand::thread_rng();
        let xpos = range.sample(&mut rng);
        let ypos = range.sample(&mut rng);

        self.x = xpos;
        self.y = ypos;
        self.score = 0;
        self.length = 1;
        self.dir = (0, 0);
        self.segments = Vec::from([(xpos, ypos)]);
        self.movable_dirs = [true, true, true, true];
    }
    fn respawn_food(&mut self, food: &mut (isize, isize)) {
        let range = Uniform::from(0..self.grid_size as isize);
        let mut rng = rand::thread_rng();
        let x = range.sample(&mut rng);
        let y = range.sample(&mut rng);

        if self.segments.contains(&(x, y)) {
            return self.respawn_food(food);
        }

        food.0 = x;
        food.1 = y;
    }
    pub fn update(&mut self, food: &mut (isize, isize)) {
        let time_now = SystemTime::now();

        if time_now.duration_since(self.time).unwrap() > self.time_step {
            self.time = time_now;

            self.x += self.dir.0 as isize;
            self.y += self.dir.1 as isize;
            self.segments.push((self.x, self.y));
            if (self.x == food.0) && (self.y == food.1) {
                self.length += 1;
                self.respawn_food(food);
                self.score += 1;
            }
            self.segments = self.segments[((self.segments.len() - self.length as usize) as usize)..].to_vec();

        }

        self.hit_body = self.segments[..(self.segments.len() - 1)].contains(&(self.x, self.y));

        if (self.x < 0) || (self.x >= self.grid_size as isize) || (self.y < 0) || (self.y >= self.grid_size as isize) || self.hit_body {
            self.reset();
        }
    }
}