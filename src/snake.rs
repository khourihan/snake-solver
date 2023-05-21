use std::time::{SystemTime, Duration};
use std::collections::{HashMap, HashSet};
use macroquad::text::Font;
use priority_queue::PriorityQueue;

use macroquad::prelude::{is_key_pressed, KeyCode, Color, draw_circle};
use rand::distributions::{Uniform, Distribution};

#[path = "cell.rs"] mod cell;
pub use cell::Cell;


pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}



pub struct Snake {
    pub x: isize,
    pub y: isize,
    pub segments: Vec<(isize, isize)>,
    pub length: isize,
    pub score: usize,
    grid_size: usize,
    dir: (i8, i8),
    cycle_table: Vec<Vec<(i32, (i8, i8))>>,

    time: SystemTime,
    time_step: Duration,

    movable_dirs: [bool; 4],
    hit_body: bool,
    pub should_reset: bool,
    pub has_won: bool
}

impl Snake {
    pub fn new(grid_size: usize, time_step: u64) -> Self {
        let range = Uniform::from(1..(grid_size as isize) - 1);
        let mut rng = rand::thread_rng();
        let xpos = range.sample(&mut rng);
        let ypos = range.sample(&mut rng);
        Self {
            x: xpos,
            y: ypos,
            grid_size: grid_size,
            segments: Vec::from([(xpos - 1, ypos), (xpos, ypos)]),
            length: 2,
            dir: (1, 0),
            cycle_table: Vec::new(),

            time: SystemTime::now(),
            time_step: Duration::from_millis(time_step),
            movable_dirs: [true; 4], // Up, Down, Left, Right
            hit_body: false,
            score: 0,
            should_reset: false,
            has_won: false
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
    pub fn reset(&mut self, grid: &Vec<Vec<Cell>>) {
        let range = Uniform::from(1..(self.grid_size as isize) - 1);
        let mut rng = rand::thread_rng();
        let xpos = range.sample(&mut rng);
        let ypos = range.sample(&mut rng);

        self.x = xpos;
        self.y = ypos;
        self.score = 0;
        self.length = 2;
        self.dir = (1, 0);
        self.segments = Vec::from([(xpos - 1, ypos), (xpos, ypos)]);
        self.movable_dirs = [true; 4];
        self.build_cycle(grid);
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

    fn direction_to(&self, cell: &(isize, isize), adj_cell: &(isize, isize)) -> (i8, i8) {
        return ((adj_cell.0 - cell.0) as i8, (adj_cell.1 - cell.1) as i8);
    }
    fn reconstruct_path(&self, route: &mut Vec<(i8, i8)>, visited: HashMap<(isize, isize), (isize, isize)>, food: &mut (isize, isize)) {
        let mut path_segment = *food;
        while visited.contains_key(&path_segment) {
            let parent = visited.get(&path_segment).unwrap();
            if *parent == (-1isize, -1isize) {
                break;
            }

            route.push(self.direction_to(parent, &path_segment));
            path_segment = *parent;
        }
    }
    fn create_shortest_path(&self, food: &mut (isize, isize), grid: &Vec<Vec<Cell>>) -> Vec<(i8, i8)> {
        let mut route: Vec<(i8, i8)> = Vec::new();
        let graph = self.create_graph(grid);
        let visited = self.astar(food, grid, graph);
        if visited.len() > 0 {
            self.reconstruct_path(&mut route, visited, food);
        }
        route.reverse();
        return route;
    }
    // pub fn update_compute(&mut self, food: &mut (isize, isize), grid: &Vec<Vec<Cell>>) {
    //     self.optimal_path = self.create_shortest_path(food, grid);
    //     if self.optimal_path.len() > 0 {
    //         self.optimal_path.pop().unwrap();
    //         (self.x, self.y) = self.optimal_path.pop().unwrap();
    //     }
    // }

    fn is_safe(&self, pos: (isize, isize), grid: &Vec<Vec<Cell>>) -> bool {
        return ((pos.0 >= 0) && (pos.0 < self.grid_size as isize) && (pos.1 >= 0) && (pos.1 < self.grid_size as isize)) && (
            !grid[pos.0 as usize][pos.1 as usize].is_snake());
    }
    pub fn longest_path_to_tail(&self, grid: &Vec<Vec<Cell>>) -> Vec<(i8, i8)> {
        let mut destination = self.segments[0];
        let mut path = self.create_shortest_path(&mut destination, grid);
        if path.len() == 0 {
            return Vec::new();
        }
        let head = self.segments[self.segments.len() - 1];
        let mut current = head;
        
        let mut visited = HashSet::new();
        
        visited.insert(grid[current.0 as usize][current.1 as usize].pos());
        for dir in path.iter() {
            current = grid[(current.0 + dir.0 as isize) as usize][(current.1 + dir.1 as isize) as usize].pos();
            visited.insert(grid[current.0 as usize][current.1 as usize].pos());
        };

        let mut idx = 0;
        current = head;
        loop {
            let cur_dir = path[idx];
            let next = grid[(current.0 + cur_dir.0 as isize) as usize][(current.1 + cur_dir.1 as isize) as usize].pos();

            let test_dirs: [(i8, i8); 2];
            if cur_dir.1 == 0 {
                test_dirs = [(0, 1), (0, -1)];
            } else if cur_dir.0 == 0 {
                test_dirs = [(1, 0), (-1, 0)];
            } else {
                test_dirs = [(0, 0), (0, 0)];
            }

            let mut extended = false;
            for test_dir in test_dirs.iter() {
                if ((current.0 + test_dir.0 as isize) < 0) || ((current.1 + test_dir.1 as isize) < 0) || (
                    (next.0 + test_dir.0 as isize) < 0) || ((next.1 + test_dir.1 as isize) < 0) || (
                    current.0 + test_dir.0 as isize >= self.grid_size as isize) || (current.1 + test_dir.1 as isize >= self.grid_size as isize) || (
                    next.0 + test_dir.0 as isize >= self.grid_size as isize) || (next.1 + test_dir.1 as isize >= self.grid_size as isize) {
                        continue;
                    }
                let cur_test = grid[(current.0 + test_dir.0 as isize) as usize][(current.1 + test_dir.1 as isize) as usize].pos();
                let next_test = grid[(next.0 + test_dir.0 as isize) as usize][(next.1 + test_dir.1 as isize) as usize].pos();
                if (self.is_safe(cur_test, grid) && !visited.contains(&cur_test)) && (self.is_safe(next_test, grid) && !visited.contains(&next_test)) {
                    visited.insert(cur_test);
                    visited.insert(next_test);
                    path.insert(idx, *test_dir);
                    path.insert(idx + 2, (test_dir.0 * -1, test_dir.1 * -1));
                    extended = true;
                    break;
                }
            };

            if !extended {
                current = next;
                idx += 1;
                if idx >= path.len() {
                    break;
                }
            }
        };
        return path;
    }
    pub fn build_cycle(&mut self, grid: &Vec<Vec<Cell>>) {
        self.cycle_table.clear();
        let path = self.longest_path_to_tail(grid);
        let mut current = self.segments[self.segments.len() - 1];
        let mut count = 0;

        for _ in 0..self.grid_size {
            let mut row = Vec::new();
            for _ in 0..self.grid_size {
                row.push((0, (0, 0)));
            }
            self.cycle_table.push(row);
        }

        for dir in path.iter() {
            self.cycle_table[current.0 as usize][current.1 as usize].0 = count;
            self.cycle_table[current.0 as usize][current.1 as usize].1 = *dir;
            current = grid[(current.0 + dir.0 as isize) as usize][(current.1 + dir.1 as isize) as usize].pos();
            count += 1;
        }
        current = self.segments[0];
        for _ in 0..(self.segments.len() - 1) {
            self.cycle_table[current.0 as usize][current.1 as usize].0 = count;
            self.cycle_table[current.0 as usize][current.1 as usize].1 = self.dir;
            current = grid[(current.0 + self.dir.0 as isize) as usize][(current.1 + self.dir.1 as isize) as usize].pos();
            count += 1;
        }

        for row in self.cycle_table.iter() {
            for cell in row.iter() {
                if cell.1 == (0, 0) {
                    self.should_reset = true;
                }
            }
        }
    }
    fn relative_dist(&self, original: i32, x: i32) -> i32 {
        let mut val = x;
        if original > x {
            val += self.grid_size as i32 * self.grid_size as i32;
        };
        return val - original;
    }
    pub fn get_next_direction(&mut self, food: &mut (isize, isize), grid: &Vec<Vec<Cell>>) -> (i8, i8) {
        let head = (self.x as usize, self.y as usize);
        let mut next_dir = self.cycle_table[head.0][head.1].1;

        if (self.length as f32) < (0.5 * self.grid_size as f32 * self.grid_size as f32) {
            let path = self.create_shortest_path(food, grid);
            if path.len() > 0 {
                let tail = self.segments[0];
                let next = grid[(head.0 as i8 + path[0].0) as usize][(head.1 as i8 + path[0].1) as usize].pos();
                let tail_idx = self.cycle_table[tail.0 as usize][tail.1 as usize].0;
                let head_idx = self.cycle_table[head.0][head.1].0;
                let next_idx = self.cycle_table[next.0 as usize][next.1 as usize].0;
                let food_idx = self.cycle_table[food.0 as usize][food.1 as usize].0;
                if !((path.len() == 1) && ((food_idx - tail_idx).abs() == 1)) {
                    let head_idx_rel = self.relative_dist(tail_idx, head_idx);
                    let next_idx_rel = self.relative_dist(tail_idx, next_idx);
                    let food_idx_rel = self.relative_dist(tail_idx, food_idx);
                    if (next_idx_rel > head_idx_rel) && (next_idx_rel <= food_idx_rel) {
                        next_dir = path[0];
                    }
                }
            }
        }
        return next_dir;
    }

    pub fn draw_debug(&self, tilesize: f32, font: Font) {
        for (x, col) in self.cycle_table.iter().enumerate() {
            for (y, cell) in col.iter().enumerate() {
                // let color0 = Color::from_rgba(255, 0, 0, 255);
                // let color1 = Color::from_rgba(0, 0, 255, 255);
                // let mut color = Color::from_vec(color0.to_vec().lerp(color1.to_vec(), cell.0 as f32 / ((self.grid_size * self.grid_size) as f32)));

                // if cell.0 <= 0i32 {
                //     color = Color::new(0.0, 0.0, 0.0, 1.0);
                // }
                
                let half = tilesize / 2.0;
                // let quarter = tilesize * 0.2;
                let x_pos = x as f32 * tilesize;
                let y_pos = y as f32 * tilesize;

                // draw_rectangle(x as f32 * tilesize, y as f32 * tilesize, tilesize, tilesize, color);
                // if cell.1 == (0, 1) {
                //     draw_triangle(
                //         Vec2::new(x_pos + half, y_pos + tilesize - quarter), 
                //         Vec2::new(x_pos + quarter, y_pos + quarter), 
                //         Vec2::new(x_pos + tilesize - quarter, y_pos + quarter), Color::new(1.0, 1.0, 1.0, 1.0));
                // }
                // else if cell.1 == (0, -1) {
                //     draw_triangle(
                //         Vec2::new(x_pos + half, y_pos + quarter), 
                //         Vec2::new(x_pos + tilesize - quarter, y_pos + tilesize - quarter), 
                //         Vec2::new(x_pos + quarter, y_pos + tilesize - quarter), Color::new(1.0, 1.0, 1.0, 1.0));
                // }
                // else if cell.1 == (1, 0) {
                //     draw_triangle(
                //         Vec2::new(x_pos + tilesize - quarter, y_pos + half), 
                //         Vec2::new(x_pos + quarter, y_pos + quarter), 
                //         Vec2::new(x_pos + quarter, y_pos + tilesize - quarter), Color::new(1.0, 1.0, 1.0, 1.0));
                // }
                // else if cell.1 == (-1, 0) {
                //     draw_triangle(
                //         Vec2::new(x_pos + quarter, y_pos + half), 
                //         Vec2::new(x_pos + tilesize - quarter, y_pos + quarter), 
                //         Vec2::new(x_pos + tilesize - quarter, y_pos + tilesize - quarter), Color::new(1.0, 1.0, 1.0, 1.0));
                // }
                // else {
                if cell.1 == (0, 0) {
                    draw_circle(x_pos + half, y_pos + half, 20.0, Color::new(1.0, 1.0, 1.0, 1.0));
                }
                // }
            }
        }
    }
    pub fn update(&mut self, food: &mut (isize, isize), grid: &Vec<Vec<Cell>>) {
        let time_now = SystemTime::now();

        if time_now.duration_since(self.time).unwrap() >= self.time_step {
            self.time = time_now;

            let new_dir = self.get_next_direction(food, grid);
            if new_dir != (0, 0) {
                self.dir = new_dir;
            }
            self.x += self.dir.0 as isize;
            self.y += self.dir.1 as isize;
            self.segments.push((self.x, self.y));
            if (self.x == food.0) && (self.y == food.1) {
                self.length += 1;
                self.respawn_food(food);
                self.score += 1;
            }
            self.segments = self.segments[((self.segments.len() - self.length as usize) as usize)..].to_vec();
            if self.length >= (self.grid_size as isize * self.grid_size as isize) - 1 {
                self.has_won = true;
            }
        }

        self.hit_body = self.segments[..(self.segments.len() - 1)].contains(&(self.x, self.y));

        if (self.x < 0) || (self.x >= self.grid_size as isize) || (self.y < 0) || (self.y >= self.grid_size as isize) || self.hit_body {
            self.should_reset = true;
        }
    }
}