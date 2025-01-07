use bevy::prelude::*;
use crate::input::handle_camera_movement;
use crate::ui::resources::SimulationState;

pub mod components;
pub mod systems;
pub mod resources;
pub mod bundles;
pub mod events;

use events::ApplyForceEvent;
use resources::*;
use systems::*;

pub struct Boids3DPlugin;

impl Plugin for Boids3DPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraControlState>()
        .insert_resource(BoidSettings3D::default())
        .insert_resource(GroupsTargets::default())
        .add_event::<ApplyForceEvent>()
        .add_systems(Update, (
            flocking,
            apply_forces_system,
            update_boid_position,
            confine_movement,
            adjust_population,
            handle_camera_movement,
        ).run_if(in_state(SimulationState::Mode3D)));
    }
}