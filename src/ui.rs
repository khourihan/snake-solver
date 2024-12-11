use std::num::{NonZero, NonZeroU32};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::{EguiContext, EguiPlugin}, bevy_inspector, egui, prelude::*, DefaultInspectorConfigPlugin};

use crate::{arena::Arena, game::GameState, settings::Settings, snake::Snake, solver::{astar::AstarSolver, greedy::GreedySolver, hamilton::HamiltonSolver, Solver}}; 

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((DefaultInspectorConfigPlugin, EguiPlugin))
            .add_systems(Update, (
                update_settings,
                update_ui,
                update_game_state,
                update_solver,
            ));
    }
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Configuration {
    pub debug_adjacencies: bool,
    pub debug_solver_tables: bool,
    pub debug_solver_paths: bool,
    pub debug_solver_points: bool,
    solver: SolverVariant,
    interval: Option<f32>,
    substeps: NonZeroU32,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            debug_adjacencies: false,
            debug_solver_tables: true,
            debug_solver_paths: true,
            debug_solver_points: false,
            solver: SolverVariant::default(),
            interval: None,
            substeps: NonZero::new(1).unwrap(),
        }
    }
}

#[derive(Default, Reflect, PartialEq, Copy, Clone)]
enum SolverVariant {
    Astar,
    Greedy,
    #[default]
    Hamilton,
}

fn update_ui(world: &mut World) {
    let window = world.query_filtered::<&Window, With<PrimaryWindow>>().single(world);
    let window_size = window.size();

    let Ok(context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };

    let mut context = context.clone();

    let width = (window_size.x - window_size.y) / 2.0;

    egui::SidePanel::right("options")
        .resizable(false)
        .exact_width(width)
        .show(context.get_mut(), |ui| {
            bevy_inspector::ui_for_resource::<Configuration>(world, ui);
        });
}

fn update_settings(
    mut settings: ResMut<Settings>,
    config: Res<Configuration>
) {
    settings.interval = config.interval;
    settings.substeps = config.substeps;
}

fn update_solver(
    mut solver: ResMut<Solver>,
    config: Res<Configuration>,
    snake: Res<Snake>,
    arena: Res<Arena>,
    mut last_solver: Local<SolverVariant>,
) {
    if !config.is_changed() {
        return;
    }

    if config.solver == *last_solver {
        return;
    }

    *last_solver = config.solver;

    match config.solver {
        SolverVariant::Astar => *solver = Solver::Astar(AstarSolver::default()),
        SolverVariant::Greedy => *solver = Solver::Greedy(GreedySolver::default()),
        SolverVariant::Hamilton => *solver = Solver::Hamilton(HamiltonSolver::default()),
    }

    solver.initialize(&snake, &arena);
}

fn update_game_state(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if *state == GameState::Running {
            next_state.set(GameState::Paused);
        } else {
            next_state.set(GameState::Running);
        }
    }

    if keys.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Stopped);
    }
}
