use astar::shortest_path;
use bevy::{prelude::*, utils::HashMap};

use crate::{arena::{Arena, Direction}, snake::Snake};

mod astar;

#[derive(Resource, Debug, Clone)]
pub struct Solver {
    pub shortest_path: Option<HashMap<UVec2, Direction>>,
}

impl Solver {
    pub fn get_direction(&mut self, _snake: &Snake, arena: &Arena) -> Direction {
        if let Some(route) = &mut self.shortest_path {
            let dir = route.remove(&arena.head).unwrap();
            if route.is_empty() {
                self.shortest_path = None;
            }
            dir
        } else {
            let mut route = shortest_path(arena.head, arena.food.unwrap(), arena);
            let dir = route.remove(&arena.head).unwrap();
            self.shortest_path = Some(route);
            dir
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            shortest_path: None,
        }
    }
}
