use std::num::{NonZero, NonZeroU32};

use bevy::prelude::*;

use crate::game::TimeSteps;

#[derive(Resource)]
pub struct Settings {
    pub arena_size: UVec2,
    pub interval: Option<f32>,
    pub substeps: NonZeroU32,
    pub colors: ColorSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            arena_size: UVec2::splat(16),
            interval: Some(0.5),
            substeps: NonZero::new(1).unwrap(),
            colors: ColorSettings::default(),
        }
    }
}

pub struct ColorSettings {
    pub background_light: Color,
    pub background_dark: Color,
    pub snake_tail: Color,
    pub snake_head: Color,
    pub food: Color,
}

impl Default for ColorSettings {
    fn default() -> Self {
        Self {
            background_light: Color::srgb(0.12, 0.125, 0.14),
            background_dark: Color::srgb(0.09, 0.1, 0.11),
            snake_tail: Color::srgb_u8(45, 148, 10),
            snake_head: Color::srgb_u8(150, 235, 89),
            food: Color::srgb_u8(224, 176, 31),
        }
    }
}

pub fn setup_time_steps(
    settings: Res<Settings>,
    mut time_steps: ResMut<TimeSteps>,
) {
    time_steps.interval = settings.interval;
    time_steps.substeps = settings.substeps;
}

pub fn update_time_steps(
    settings: Res<Settings>,
    mut time_steps: ResMut<TimeSteps>,
) {
    if settings.is_changed() {
        time_steps.interval = settings.interval;
        time_steps.substeps = settings.substeps;
    }
}
