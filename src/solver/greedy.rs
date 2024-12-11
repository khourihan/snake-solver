use bevy::prelude::*;

use crate::{arena::{Arena, Direction}, snake::Snake};

use super::{pathfinding::{longest_path, shortest_path}, SolveMethod};

/// Greedy solver that uses A* and forward checking, falling back to the longest path from head to
/// tail.
///
/// While the greedy solver quickly makes progress, it isn't guaranteed to find a valid Hamiltonian 
/// cycle nor follow it by the end of the game, meaning it can occasionally get stuck in unwinnable 
/// loops and lose near the end. Also, the greedy solver cannot account for food spawning in the
/// longest path from its head to its tail, which can cause it to lose by running into its tail.
#[derive(Debug, Clone, Default)]
pub struct GreedySolver {
    shortest_path: Option<Vec<Direction>>,
    longest_path: Option<Vec<Direction>>,
    head: UVec2,
    virtual_tail: Option<UVec2>,
}

impl SolveMethod for GreedySolver {
    fn get_direction(&mut self, snake: &Snake, arena: &Arena) -> Direction {
        let shortest = shortest_path(arena.head, arena.food.unwrap(), snake.direction, &arena.adjacencies);
        self.head = arena.head;
        self.longest_path = None;
        self.virtual_tail = None;

        if let Some(shortest) = shortest {
            let mut adjacencies = arena.adjacencies.clone();
            let mut head = arena.head;
            let mut tail = arena.tail;
            let mut behind = arena.behind;
            let mut behind_behind = arena.behind;

            // Move a virtual snake to eat the food along the shortest path.
            for dir in shortest.iter() {
                adjacencies.remove(head);
                adjacencies.insert_snake_segment(head, *dir);
                adjacencies.insert(tail);
                head = head + dir;
                behind_behind = behind;
                behind = tail;
                tail = tail + adjacencies.get_segment_direction(tail).unwrap();
                adjacencies.remove_snake_segment(behind);
            }

            // Account for growth of snake from eating food.
            if arena.just_ate {
                adjacencies.remove(behind);
                behind = behind_behind;
            }

            self.virtual_tail = Some(behind);

            // Compute the longest path from the virtual snake's head to its tail.
            let longest = shortest_path(head, behind, shortest[0], &adjacencies)
                .and_then(|path| longest_path(head, &adjacencies, path));

            let dir = shortest[0];
            self.shortest_path = Some(shortest);

            // If this path exists for the virtual snake, account for growth of virtual snake and
            // check if it can still reach its tail.
            if let Some(longest) = longest {
                let test_head = head + longest[0];
                adjacencies.remove(head);

                // If it can still reach its tail without running into it, move in the direction 
                // of the shortest path to the food.
                if let Some(path) = shortest_path(test_head, behind, longest[0], &adjacencies) {
                    if path.len() > 1 {
                        return dir;
                    }
                }
            }
        }

        self.shortest_path = None;

        // Otherwise, compute longest path from the snake's head to its tail.
        let longest = shortest_path(arena.head, arena.behind, snake.direction, &arena.adjacencies)
            .and_then(|path| longest_path(arena.head, &arena.adjacencies, path));

        if let Some(longest) = longest {
            // If this path exists, take it.
            let dir = longest[0];
            self.longest_path = Some(longest);
            dir
        } else {
            info!("cannot follow food or tail");
            // Otherwise, move in the direction that takes the snake farthest from the food.
            let start = arena.head.as_ivec2();
            let food = arena.food.unwrap().as_ivec2();
            let dirs = arena.adjacencies.get_directions(arena.head);
            let dist = |p1: IVec2, p2: IVec2| (p1.x - p2.x).abs() + (p1.y - p2.y).abs();
            let mut max_dist = -1;
            let mut dir = snake.direction;

            if dirs.up() {
                let d = dist(start + Direction::UP_OFFSET, food);
                if d > max_dist {
                    max_dist = d;
                    dir = Direction::Up;
                }
            }

            if dirs.down() {
                let d = dist(start + Direction::DOWN_OFFSET, food);
                if d > max_dist {
                    max_dist = d;
                    dir = Direction::Down;
                }
            }

            if dirs.right() {
                let d = dist(start + Direction::RIGHT_OFFSET, food);
                if d > max_dist {
                    max_dist = d;
                    dir = Direction::Right;
                }
            }

            if dirs.left() {
                let d = dist(start + Direction::LEFT_OFFSET, food);
                if d > max_dist {
                    dir = Direction::Left;
                }
            }

            dir
        }
    }

    fn debug_paths(&self, _arena: &Arena) -> Vec<(UVec2, Option<&[Direction]>)> {
        vec![(self.head, self.shortest_path.as_deref()), (self.head, self.longest_path.as_deref())]
    }

    fn debug_points(&self, _arena: &Arena) -> Vec<Option<UVec2>> {
        vec![None, None, self.virtual_tail]
    }
}
