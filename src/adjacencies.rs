use bevy::{math::UVec2, utils::HashMap};
use smallvec::SmallVec;

use crate::arena::Directions;

pub struct AdjacencyGraph{
    size: UVec2,
    graph: HashMap<UVec2, Directions>,
}

impl AdjacencyGraph {
    #[inline]
    pub const fn new(adjacencies: HashMap<UVec2, Directions>, size: UVec2) -> AdjacencyGraph {
        Self {
            size,
            graph: adjacencies,
        }
    }

    pub fn reset(&mut self) {
        self.graph.clear();
    }

    #[inline]
    pub fn get_directions(&self, pos: UVec2) -> Directions {
        self.graph.get(&pos).copied().unwrap_or(Directions::NONE)
    }

    pub fn get_neighbors(&self, pos: UVec2) -> SmallVec<[UVec2; 4]> {
        let dirs = self.get_directions(pos);
        let mut neighbors = SmallVec::new();

        if dirs.up() {
            neighbors.push(pos + UVec2::new(0, 1));
        }

        if dirs.down() {
            neighbors.push(pos - UVec2::new(0, 1));
        }

        if dirs.right() {
            neighbors.push(pos + UVec2::new(1, 0));
        }

        if dirs.left() {
            neighbors.push(pos - UVec2::new(1, 0));
        }

        neighbors
    }

    pub fn contains(&self, pos: UVec2) -> bool {
        self.graph.contains_key(&pos)
    }

    pub fn remove(&mut self, pos: UVec2) {
        self.graph.remove(&pos);

        if pos.x != 0 {
            self.graph.entry(pos - UVec2::new(1, 0))
                .and_modify(|c| *c &= !Directions::RIGHT);
        }

        if pos.x < self.size.x - 1 {
            self.graph.entry(pos + UVec2::new(1, 0))
                .and_modify(|c| *c &= !Directions::LEFT);
        }

        if pos.y != 0 {
            self.graph.entry(pos - UVec2::new(0, 1))
                .and_modify(|c| *c &= !Directions::UP);
        }

        if pos.y < self.size.y - 1 {
            self.graph.entry(pos + UVec2::new(0, 1))
                .and_modify(|c| *c &= !Directions::DOWN);
        }
    }

    pub fn insert(&mut self, pos: UVec2) {
        let mut dirs = Directions::NONE;

        if pos.x != 0 {
            self.graph.entry(pos - UVec2::new(1, 0))
                .and_modify(|c| {
                    *c |= Directions::RIGHT;
                    dirs |= Directions::LEFT;
                });
        }

        if pos.x < self.size.x - 1 {
            self.graph.entry(pos + UVec2::new(1, 0))
                .and_modify(|c| {
                    *c |= Directions::LEFT;
                    dirs |= Directions::RIGHT;
                });
        }

        if pos.y != 0 {
            self.graph.entry(pos - UVec2::new(0, 1))
                .and_modify(|c| {
                    *c |= Directions::UP;
                    dirs |= Directions::DOWN;
                });
        }

        if pos.y < self.size.y - 1 {
            self.graph.entry(pos + UVec2::new(0, 1))
                .and_modify(|c| {
                    *c |= Directions::DOWN;
                    dirs |= Directions::UP;
                });
        }

        self.graph.insert(pos, dirs);
    }

    pub fn nodes(&self) -> impl Iterator<Item = (&UVec2, &Directions)> {
        self.graph.iter()
    }
}
