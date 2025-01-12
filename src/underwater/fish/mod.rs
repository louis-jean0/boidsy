use bevy::prelude::*;
use components::UnderwaterBoidSettings;
use crate::ui::resources::SimulationState;
use crate::boids_3d::systems::*;

mod components;
mod systems;

pub use systems::*;
pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnderwaterBoidSettings>()
           .add_systems(Startup, load_fish_models)
           .add_systems(OnEnter(SimulationState::Underwater), spawn_fish_schools)
           .add_systems(Update, (
                apply_underwater_flocking,
                apply_forces_system,
                update_fish_positions,
                confine_fishes_movement,
                avoid_obstacles
           ).run_if(in_state(SimulationState::Underwater)));
    }
}
