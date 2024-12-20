use bevy::prelude::*;

pub mod components;
mod systems;
pub mod resources;
pub mod bundles;
pub mod events;

use events::ApplyForceEvent;
use resources::*;
use systems::*;

pub const BOIDS_COUNT: usize = 1800;
pub const BOIDS_ALIGNMENT_RANGE: f32 = 30.0;
pub const BOIDS_COHESION_RANGE: f32 = 10.0;
pub const BOIDS_SEPARATION_RANGE: f32 = 20.0;

pub struct Boids2DPlugin;

impl Plugin for Boids2DPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoidSettings::new(BOIDS_COUNT,BOIDS_ALIGNMENT_RANGE, BOIDS_COHESION_RANGE, BOIDS_SEPARATION_RANGE))
        .insert_resource(GroupsTargets::default())
        .add_event::<ApplyForceEvent>()
        .add_systems(Startup, spawn_boids)
        .add_systems(Startup, spawn_obstacles_system)
        .add_systems(Update, flocking)
        .add_systems(Update, apply_forces_system)
        .add_systems(Update, update_boid_position)
        .add_systems(Update, confine_movement)
        .add_systems(Update, adjust_population);
    }
}