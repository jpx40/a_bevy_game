pub mod default_plugin;
use bevy::{
    app::App,
    ecs::schedule::ScheduleLabel,
    prelude::{Bundle, Commands, EntityCommands, IntoSystemConfigs},
};
pub mod port;
use glam::f32::{Vec2, Vec3};

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

pub fn i32<T: Sized>(i: T) -> i32
where
    T: Into<i32>,
{
    i.into()
}
pub fn i64<T: Sized>(i: T) -> i64
where
    T: Into<i64>,
{
    i.into()
}

pub fn i16<T: Sized>(i: T) -> i16
where
    T: Into<i16>,
{
    i.into()
}

pub fn f32<T: Sized>(i: T) -> f32
where
    T: Into<f32>,
{
    i.into()
}
pub fn f64<T: Sized>(i: T) -> f64
where
    T: Into<f64>,
{
    i.into()
}
pub fn i128<T: Sized>(i: T) -> i128
where
    T: Into<i128>,
{
    i.into()
}
pub fn u128<T: Sized>(i: T) -> u128
where
    T: Into<u128>,
{
    i.into()
}

pub fn u32<T: Sized>(i: T) -> u32
where
    T: Into<u32>,
{
    i.into()
}

pub fn u16<T: Sized>(i: T) -> u16
where
    T: Into<u16>,
{
    i.into()
}
pub fn u8<T: Sized>(i: T) -> u8
where
    T: Into<u8>,
{
    i.into()
}

pub fn i8<T: Sized>(i: T) -> i8
where
    T: Into<i8>,
{
    i.into()
}

pub fn u64<T: Sized>(i: T) -> u64
where
    T: Into<u64>,
{
    i.into()
}

pub fn spawn<T: Bundle>(command: &mut Commands, bundle: T) {
    command.spawn(bundle);
}

pub fn add_system<M>(
    app: &mut App,
    schedule: impl ScheduleLabel,
    systems: impl IntoSystemConfigs<M>,
) -> &mut App {
    app.add(schedule, systems)
}

pub trait AddSystem {
    fn add<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self;
}

impl AddSystem for App {
    fn add<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoSystemConfigs<M>,
    ) -> &mut Self {
        self.add_systems(schedule, systems)
    }
}

use std::{fmt::Display, num::NonZeroU8};

use bevy::{
    log::LogPlugin,
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};

/// Helper system to enable closing the example application by pressing the
/// escape key (ESC).
pub fn close_on_esc(mut ev_app_exit: EventWriter<AppExit>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        ev_app_exit.send(AppExit::Success);
    }
}

/// Calculate a log filter for the LogPlugin based on the example app name.
pub fn get_log_filters(example_name: &str) -> String {
    [
        // The example app itself is at trace level so we can see everything
        &format!("{}=trace", example_name),
        // Default Hanabi to warn, probably don't need more
        "bevy_hanabi=warn",
        // Prevent HAL from dumping all naga-generated shader code in logs
        "wgpu_hal::dx12::device=warn",
        // Tune down the verbose Vulkan driver output
        "wgpu_hal::vulkan::instance=warn",
    ]
    .join(",")
}

/// Error struct wrapping an app error code.
#[derive(Debug)]
pub struct ExampleFailedError(pub NonZeroU8);

impl Display for ExampleFailedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "App terminated with error code {}", self.0.get())
    }
}

impl std::error::Error for ExampleFailedError {}

/// Convert an [`AppExit`] into a `Result`, for error code propagation to the
/// OS.
pub trait AppExitIntoResult {
    fn into_result(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl AppExitIntoResult for AppExit {
    fn into_result(&self) -> Result<(), Box<dyn std::error::Error>> {
        match *self {
            AppExit::Success => Ok(()),
            AppExit::Error(code) => Err(Box::new(ExampleFailedError(code))),
        }
    }
}

pub const COLOR_RED: Color = Color::linear_rgb(1., 0., 0.);
pub const COLOR_GREEN: Color = Color::linear_rgb(0., 1., 0.);
pub const COLOR_BLUE: Color = Color::linear_rgb(0., 0., 1.);
pub const COLOR_YELLOW: Color = Color::linear_rgb(1., 1., 0.);
pub const COLOR_CYAN: Color = Color::linear_rgb(0., 1., 1.);
pub const COLOR_OLIVE: Color = Color::linear_rgb(0.5, 0.5, 0.);
pub const COLOR_PURPLE: Color = Color::linear_rgb(0.5, 0., 0.5);
