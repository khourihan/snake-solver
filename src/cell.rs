use bevy::{prelude::*, window::PrimaryWindow};

use crate::{arena::Cell, settings::Settings};

#[derive(Component)]
pub struct DrawCell {
    pub pos: UVec2,
}

#[derive(Component)]
pub struct DrawCellTransform {
    /// Size of the cell, where `Vec2::ONE` is a full cell.
    pub size: Vec2,
    /// Offset of the cell inside a full cell.
    pub offset: Vec2,
}

#[derive(Component)]
pub struct ForegroundCell {
    pub contents: Cell,
}

pub fn setup_cells(
    mut commands: Commands,
    settings: Res<Settings>,
) {
    for x in 0..settings.arena_size.x {
        for y in 0..settings.arena_size.y {
            let color = if (x + y) % 2 == 0 {
                settings.colors.background_light
            } else {
                settings.colors.background_dark
            };

            commands.spawn((
                DrawCell {
                    pos: UVec2::new(x, y),
                },
                Sprite::from_color(color, Vec2::ONE),
                Transform::default(),
                Visibility::default(),
            ));
        }
    }
}

pub fn update_cell_transform(
    windows: Query<&Window, With<PrimaryWindow>>,
    settings: Res<Settings>,
    mut cells: Query<(&DrawCell, &mut Transform, Option<&DrawCellTransform>, Option<&ForegroundCell>)>
) {
    let window = windows.single();
    let mut window_size = window.size();

    let max_dim = if window_size.x > window_size.y { 0 } else { 1 };
    let aspect = window_size[1 - max_dim] / window_size[max_dim];

    let mut tile_size = window_size / settings.arena_size.as_vec2();
    tile_size[max_dim] *= aspect;
    window_size[max_dim] *= aspect;

    for (cell, mut transform, cell_transform, fg) in &mut cells {
        let (size, offset) = if let Some(t) = cell_transform {
            (t.size, t.offset)
        } else {
            (Vec2::ONE, Vec2::ZERO)
        };

        transform.scale = Vec3::new(tile_size.x * size.x, tile_size.y * size.y, 1.0);

        let pos = (cell.pos.as_vec2() + offset) / settings.arena_size.as_vec2() * window_size - (window_size / 2.0) + (tile_size / 2.0);
        let z = if fg.is_some() { 1.0 } else { 0.0 };
        transform.translation = Vec3::new(pos.x, pos.y, z);
    }
}
