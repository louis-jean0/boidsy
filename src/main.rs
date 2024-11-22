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
    .add_plugins(Boids2DPlugin)
    //.add_systems(Update, ui_example)
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

pub fn ui_example(mut egui_context: EguiContexts) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}