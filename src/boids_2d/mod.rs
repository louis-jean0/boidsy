use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;
pub mod bundles;
pub mod events;

use events::ApplyForceEvent;
use resources::*;
use systems::*;
use crate::SimulationState;

pub struct Boids2DPlugin;

impl Plugin for Boids2DPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BoidSettings2D::default())
        .insert_resource(GroupsTargets::default())
        .add_event::<ApplyForceEvent>()
        .add_systems(Startup, spawn_boids)
        .add_systems(Update, (
            flocking,
            avoid_obstacles,
            scare_with_cursor,
            apply_forces_system,
            update_boid_position,
            confine_movement,
            adjust_population
        ).run_if(in_state(SimulationState::Mode2D)))
        .add_systems(OnEnter(SimulationState::Mode2D), setup_background);
    }
}

fn setup_background(
    mut commands: Commands,
) {
    commands.insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)));
}