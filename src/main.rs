use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_egui::*;

mod boids_2d;
mod ui;
mod input;
mod kd_tree_2d;
mod boids_3d;
mod kd_tree_3d;

use boids_2d::Boids2DPlugin;
use input::InputPlugin;
use ui::UiPlugin;
use kd_tree_2d::KDTree2DPlugin;
use boids_3d::Boids3DPlugin;
use kd_tree_3d::KDTree3DPlugin;
use ui::resources::SimulationState;
use crate::boids_2d::components::Mode2DMarker;
use crate::boids_3d::components::Mode3DMarker;
use crate::boids_2d::resources::BoidSettings2D;
use crate::boids_3d::resources::BoidSettings3D;

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

fn main() {
    App::new()
        .add_state::<SimulationState>()
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
        .add_plugins((
            KDTree2DPlugin,
            KDTree3DPlugin,
            Boids2DPlugin,
            Boids3DPlugin,
            UiPlugin,
            InputPlugin,
        ))
        .add_systems(OnEnter(SimulationState::Mode2D), setup_2d_mode)
        .add_systems(OnEnter(SimulationState::Mode3D), setup_3d_mode)
        .add_systems(OnExit(SimulationState::Mode2D), cleanup_2d_mode)
        .add_systems(OnExit(SimulationState::Mode3D), cleanup_3d_mode)
        .run();
}

fn setup_2d_mode(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    boid_settings: Res<BoidSettings2D>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Mode2DMarker,
    ));
    
   boids_2d::systems::spawn_boids(commands, window_query, asset_server, boid_settings);
}

fn setup_3d_mode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    boid_settings: Res<BoidSettings3D>
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(100.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Mode3DMarker,
    ));
    
    boids_3d::setup_3d_scene(&mut commands, &mut meshes, &mut materials);
    boids_3d::systems::spawn_boids(commands, boid_settings, meshes, materials);
}

fn cleanup_2d_mode(
    mut commands: Commands,
    entities: Query<Entity, With<Mode2DMarker>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

fn cleanup_3d_mode(
    mut commands: Commands,
    entities: Query<Entity, With<Mode3DMarker>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}