use std::{
    num::{NonZero, NonZeroU32},
    time::Duration,
};

use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    Running,
    Paused,
    #[default]
    Stopped,
}

#[derive(States, Hash, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GameMode {
    Human,
    #[default]
    Computer,
}

#[derive(Resource, Default)]
pub struct GameOver(pub bool);

pub struct SchedulesPlugin;

impl Plugin for SchedulesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeSteps>()
            .init_resource::<PreviousTime>()
            .init_resource::<LastSolverInput>()
            .init_schedule(SolveStep)
            .init_schedule(Draw);

        let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();

        order.insert_after(Update, Solve);
        order.insert_after(Solve, Draw);

        app.add_systems(Solve, run_solve_schedule);
    }
}

#[derive(ScheduleLabel, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Solve;

#[derive(ScheduleLabel, Clone, PartialEq, Eq, Debug, Hash)]
pub struct SolveStep;

#[derive(ScheduleLabel, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Draw;

#[derive(Resource)]
pub struct TimeSteps {
    /// Interval of steps, in seconds.
    ///
    /// [`None`] indicates it will step every frame.
    pub interval: Option<f32>,
    /// The number of steps to take per interval.
    pub substeps: NonZeroU32,
}

impl Default for TimeSteps {
    fn default() -> Self {
        Self {
            interval: None,
            substeps: NonZero::new(1).unwrap(),
        }
    }
}

#[derive(Resource, Default)]
pub struct LastSolverInput {
    last: Option<KeyCode>,
}

impl LastSolverInput {
    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.last == Some(key)
    }
}

#[derive(Resource, Default)]
struct PreviousTime(Duration);

fn run_solve_schedule(world: &mut World) {
    let state = world.resource::<State<GameState>>();

    if *state != GameState::Running {
        return;
    }

    let keys = world.resource::<ButtonInput<KeyCode>>();

    if let Some(&pressed) = keys.get_just_pressed().last() {
        world.resource_mut::<LastSolverInput>().last = Some(pressed);
    };

    let steps = world.resource::<TimeSteps>();
    let substeps: u32 = steps.substeps.into();

    if let Some(interval) = steps.interval {
        let time = world.resource::<Time>();
        let previous = world.resource::<PreviousTime>().0;

        let elapsed = time.elapsed();
        if (elapsed - previous).as_secs_f32() >= interval {
            world.resource_mut::<PreviousTime>().0 = elapsed;
        } else {
            return;
        }
    }

    let _ = world.try_schedule_scope(SolveStep, |world, schedule| {
        schedule.run(world);
        world.resource_mut::<LastSolverInput>().last = None;

        for _ in 0..substeps - 1 {
            let mut game_over = world.resource_mut::<GameOver>();

            if game_over.0 {
                game_over.0 = false;
                world.resource_mut::<NextState<GameState>>().set(GameState::Stopped);
                break;
            }

            schedule.run(world);
        }
    });

    let mut game_over = world.resource_mut::<GameOver>();

    if game_over.0 {
        game_over.0 = false;
        world.resource_mut::<NextState<GameState>>().set(GameState::Stopped);
    }
}
