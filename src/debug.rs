use bevy::{prelude::*, window::PrimaryWindow, color::palettes::css as colors};

use crate::{arena::{Arena, Direction}, solver::Solver};

pub fn debug_adjacencies(
    mut gizmos: Gizmos,
    windows: Query<&Window, With<PrimaryWindow>>,
    arena: Res<Arena>,
) {
    let cell_size = compute_cell_size(windows.single(), arena.size);
    let half_cell = cell_size.0 / 2.0;

    for (pos, dirs) in arena.adjacencies.nodes() {
        let center = get_cell_center(*pos, arena.size, cell_size);

        if dirs.up() {
            gizmos.line_2d(center, center + Vec2::new(0.0, half_cell.y), colors::PINK)
        }

        if dirs.down() {
            gizmos.line_2d(center, center - Vec2::new(0.0, half_cell.y), colors::PINK)
        }

        if dirs.right() {
            gizmos.line_2d(center, center + Vec2::new(half_cell.x, 0.0), colors::PINK)
        }

        if dirs.left() {
            gizmos.line_2d(center, center - Vec2::new(half_cell.x, 0.0), colors::PINK)
        }
    }
}

pub fn debug_solver_paths(
    mut gizmos: Gizmos,
    windows: Query<&Window, With<PrimaryWindow>>,
    arena: Res<Arena>,
    solver: Res<Solver>,
) {
    let cell_size = compute_cell_size(windows.single(), arena.size);
    let cell = cell_size.0;

    let colors = [
        colors::AQUA,
        colors::GREEN,
    ];

    for (i, (mut pos, path)) in solver.debug_paths(&arena).into_iter().enumerate() {
        let col = colors[i % colors.len()];

        for dir in path.iter().take(path.len() - 1) {
            let p = get_cell_center(pos, arena.size, cell_size);
            pos = (pos.as_ivec2() + dir.offset()).as_uvec2();

            match dir {
                Direction::Up => gizmos.line_2d(p, p + Vec2::new(0.0, cell.y), col),
                Direction::Down => gizmos.line_2d(p, p + Vec2::new(0.0, -cell.y), col),
                Direction::Left => gizmos.line_2d(p, p + Vec2::new(-cell.x, 0.0), col),
                Direction::Right => gizmos.line_2d(p, p + Vec2::new(cell.x, 0.0), col),
            };
        }
    }
}

fn get_cell_center(pos: UVec2, arena_size: UVec2, cell_size: (Vec2, Vec2)) -> Vec2 {
    pos.as_vec2() / arena_size.as_vec2() * cell_size.1 - (cell_size.1 / 2.0) + (cell_size.0 / 2.0)
}

fn compute_cell_size(window: &Window, arena_size: UVec2) -> (Vec2, Vec2) {
    let mut window_size = window.size();
    
    let max_dim = if window_size.x > window_size.y { 0 } else { 1 };
    let aspect = window_size[1 - max_dim] / window_size[max_dim];

    let mut cell_size = window_size / arena_size.as_vec2();
    cell_size[max_dim] *= aspect;
    window_size[max_dim] *= aspect;

    (cell_size, window_size)
}
