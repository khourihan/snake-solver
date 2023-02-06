use std::time::{SystemTime, Duration};
use std::collections::{HashMap, HashSet};
use priority_queue::PriorityQueue;

use macroquad::prelude::{is_key_pressed, KeyCode};
use rand::distributions::{Uniform, Distribution};

#[path = "cell.rs"] mod cell;
pub use cell::Cell;


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
    optimal_path: Vec<(isize, isize)>,

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
            optimal_path: Vec::new(),

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


    fn get_neighbor_cells(&self, grid: &Vec<Vec<Cell>>, cell: &Cell) -> Vec<(isize, isize)> {
        let mut neighbors = Vec::new();
        for row in grid.iter() {
            for other in row.iter() {
                if (other.x as isize - cell.x as isize).abs() + (other.y as isize - cell.y as isize).abs() != 1 {
                    continue;
                }
                if !other.is_snake() {
                    neighbors.push((other.x as isize, other.y as isize));
                }
            };
        };
        return neighbors;
    }
    fn create_graph(&self, grid: &Vec<Vec<Cell>>) -> HashMap<(isize, isize), Vec<(isize, isize)>> {
        let mut graph = HashMap::<(isize, isize), Vec<(isize, isize)>>::new();
        for row in grid.iter() {
            for cell in row.iter() {
                let neighbors = self.get_neighbor_cells(grid, cell);
                if neighbors.len() > 0 {
                    graph.insert((cell.x as isize, cell.y as isize), neighbors);
                }
            };
        };
        return graph;
    }
    fn heuristic(&self, p1: (isize, isize), p2: (isize, isize)) -> isize {
        return (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
    }
    fn astar(&self, food: &mut (isize, isize), grid: &Vec<Vec<Cell>>, graph: HashMap<(isize, isize), Vec<(isize, isize)>>) -> HashMap<(isize, isize), (isize, isize)> {
        let mut count = 0;
        let mut open_set = PriorityQueue::new();
        open_set.push((count, (self.x, self.y)), 0);

        let mut visited = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();
        let mut open_set_hash = HashSet::new();
        open_set_hash.insert((self.x, self.y));
        visited.insert((self.x, self.y), (-1, -1));

        for row in grid.iter() {
            for cell in row.iter() {
                g_score.insert((cell.x as isize, cell.y as isize), isize::MAX);
                f_score.insert((cell.x as isize, cell.y as isize), isize::MAX);
            }
        }
        g_score.insert((self.x, self.y), 0);
        f_score.insert((self.x, self.y), self.heuristic((self.x, self.y), food.clone()));

        while !open_set.is_empty() {
            let cur_value = open_set.pop().unwrap().0;
            let current = cur_value.1;
            open_set_hash.remove(&current);

            if current == *food {
                break
            }
            let v: Vec<(isize, isize)> = Vec::new();
            let neighbors = match graph.get(&current) {
                Some(value) => value,
                None => &v,
            };

            for neigh in neighbors.iter() {
                let temp_g_score = g_score.get(&current).unwrap() + 1;

                if temp_g_score < *g_score.get(neigh).unwrap() {
                    visited.insert(*neigh, current);
                    g_score.insert(*neigh, temp_g_score);
                    f_score.insert(*neigh, temp_g_score + self.heuristic(*neigh, *food));
                    if !open_set_hash.contains(neigh) {
                        count += 1;
                        open_set.push((count, *neigh), -*f_score.get(neigh).unwrap());
                        open_set_hash.insert(*neigh);
                    };
                };
            };
        };
        return visited;
    }

    fn reconstruct_path(&self, route: &mut Vec<(isize, isize)>, visited: HashMap<(isize, isize), (isize, isize)>, food: &mut (isize, isize)) {
        let mut path_segment = *food;
        while (path_segment != (-1, -1)) && visited.contains_key(&path_segment) {
            route.push(path_segment);
            path_segment = *visited.get(&path_segment).unwrap();
        }
    }
    fn create_path(&self, food: &mut (isize, isize), grid: &Vec<Vec<Cell>>) -> Vec<(isize, isize)> {
        let mut route = Vec::new();
        let graph = self.create_graph(grid);
        let visited = self.astar(food, grid, graph);
        if visited.len() > 0 {
            self.reconstruct_path(&mut route, visited, food);
        }
        return route;
    }
    pub fn update_compute(&mut self, food: &mut (isize, isize), grid: &Vec<Vec<Cell>>) {
        self.optimal_path = self.create_path(food, grid);
        if self.optimal_path.len() > 0 {
            self.optimal_path.pop().unwrap();
            (self.x, self.y) = self.optimal_path.pop().unwrap();
        }
    }

    pub fn update(&mut self, food: &mut (isize, isize), grid: &Vec<Vec<Cell>>) {
        let time_now = SystemTime::now();

        if time_now.duration_since(self.time).unwrap() > self.time_step {
            self.time = time_now;

            self.update_compute(food, grid);
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