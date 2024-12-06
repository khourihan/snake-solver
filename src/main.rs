use bevy::prelude::*;

mod cell;
mod settings;
mod arena;
mod snake;
mod game;

const TIME_STEP: u64 = 0;
const STEPS_PER_FRAME: u64 = 15;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, game::SchedulesPlugin))
        .init_resource::<settings::Settings>()
        .init_resource::<snake::Snake>()
        .init_state::<game::GameState>()
        .add_systems(Startup, (
            setup_camera,
            cell::setup_cells,
            arena::setup_arena,
        ))
        .add_systems(PostStartup, (snake::setup_snake, settings::setup_time_steps))
        .add_systems(OnEnter(game::GameState::Stopped), (snake::setup_snake, arena::respawn_food))
        .add_systems(Update, (
            game::restart,
            settings::update_time_steps,
        ))
        .add_systems(game::SolveStep, (
            snake::update_snake_direction,
            arena::update_snake_position,
            arena::check_win,
            arena::spawn_food,
        ).chain())
        .add_systems(game::Draw, (
            arena::update_cell,
            cell::update_cell_transform,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
