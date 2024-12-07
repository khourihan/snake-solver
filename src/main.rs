use bevy::prelude::*;

mod cell;
mod settings;
mod arena;
mod snake;
mod game;
mod adjacencies;
mod debug;
mod solver;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, game::SchedulesPlugin))
        .init_resource::<settings::Settings>()
        .init_resource::<snake::Snake>()
        .init_state::<game::GameState>()
        .init_state::<game::GameMode>()
        .add_systems(Startup, (
            setup_camera,
            cell::setup_cells,
            arena::setup_arena,
        ))
        .add_systems(PostStartup, (snake::setup_snake, settings::setup_time_steps))
        .add_systems(OnEnter(game::GameState::Running), (snake::setup_snake, arena::respawn_food))
        .add_systems(OnEnter(game::GameMode::Computer), snake::setup_solver)
        .add_systems(Update, (
            game::restart,
            settings::update_time_steps,
        ))
        .add_systems(game::SolveStep, (
            arena::spawn_food,
            snake::update_snake_direction_human.run_if(in_state(game::GameMode::Human)),
            snake::compute_snake_direction.run_if(in_state(game::GameMode::Computer)),
            arena::update_snake_position,
            arena::check_win,
        ).chain())
        .add_systems(game::Draw, (
            arena::update_cell,
            cell::update_cell_transform,
            // debug::debug_adjacencies,
            debug::debug_shortest_path,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
