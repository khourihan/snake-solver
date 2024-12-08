use bevy::{math::UVec2, utils::HashSet};

use crate::{adjacencies::AdjacencyGraph, arena::Direction};

/// Computes the longest path from `start` by modifying the shortest path.
pub(super) fn longest_path(
    start: UVec2,
    adjacencies: &AdjacencyGraph,
    mut path: Vec<Direction>,
) -> Option<Vec<Direction>> {
    if path.is_empty() {
        return None;
    }

    let mut current = start;
    let mut visited = HashSet::new();

    visited.insert(current);
    
    for dir in path.iter() {
        current = (current.as_ivec2() + dir.offset()).as_uvec2();
        visited.insert(current);
    }

    let mut index = 0;
    current = start;

    loop {
        let cur_dir = path[index];
        let next = (current.as_ivec2() + cur_dir.offset()).as_uvec2();

        let test_dirs = if cur_dir.is_horizontal() {
            Some([Direction::Up, Direction::Down])
        } else if cur_dir.is_vertical() {
            Some([Direction::Right, Direction::Left])
        } else {
            None
        };

        let mut extended = false;
        for (dir, offset) in test_dirs.into_iter().flat_map(|v| v.into_iter()).map(|dir| (dir, dir.offset())) {
            let cur_test = current.as_ivec2() + offset;
            let next_test = next.as_ivec2() + offset;

            if cur_test.x < 0 || cur_test.y < 0 || next_test.x < 0 || next_test.y < 0 {
                continue;
            }

            let cur_test = cur_test.as_uvec2();
            let next_test = next_test.as_uvec2();

            if adjacencies.contains(cur_test) && !visited.contains(&cur_test)
                && adjacencies.contains(next_test) && !visited.contains(&next_test)
            {
                visited.insert(cur_test);
                visited.insert(next_test);
                path.insert(index, dir);
                path.insert(index + 2, dir.flip());
                extended = true;
                break;
            }
        }

        if !extended {
            current = next;
            index += 1;
            if index >= path.len() {
                break;
            }
        }
    }

    Some(path)
}
