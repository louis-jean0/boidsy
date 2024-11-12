use bevy::{prelude::*, render::view::window, window::PrimaryWindow};
mod boids;
use boids::boids_2d::BoidType;

pub const BOIDS_COUNT: usize = 50;
pub const BOIDS_COHESION_RANGE: f32 = 50.0;
pub const BOIDS_REPULSION_RANGE: f32 = 10.0;
pub const BOIDS_ALIGNMENT_RANGE: f32 = 50.0;
pub const BOIDS_TYPE: BoidType = BoidType::Bird;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_camera)
    .insert_resource(boids::boids_2d::BoidSettings::new(BOIDS_COUNT, BOIDS_COHESION_RANGE, BOIDS_REPULSION_RANGE, BOIDS_ALIGNMENT_RANGE, BOIDS_TYPE))
    .add_systems(Startup, boids::boids_2d::spawn_boid)
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