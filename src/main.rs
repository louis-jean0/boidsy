use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::*;

mod boids_2d;
mod ui;
use boids_2d::Boids2DPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin)
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, setup_window)
    .add_plugins(Boids2DPlugin)
    .add_systems(Update, ui::setup_ui)
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

fn setup_window(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        window.title = "Boidsy".to_string();
    } 
}