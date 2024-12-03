use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_egui::*;

mod boids_2d;
mod ui;
mod input;
mod kd_tree_2d;
use boids_2d::Boids2DPlugin;
use input::InputPlugin;
use ui::UiPlugin;
use kd_tree_2d::KDTreePlugin;

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;



fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            title: "Boidsy".to_string(),
            resizable: true,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(EguiPlugin)
    .add_systems(Startup, spawn_camera)
    .add_plugins(Boids2DPlugin)
    .add_plugins(UiPlugin)
    .run();
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
    ) {

    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}