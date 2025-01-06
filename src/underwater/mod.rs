use bevy::prelude::*;

mod submarine;
mod terrain;
mod environment;

use submarine::SubmarinePlugin;
use terrain::TerrainPlugin;
use environment::EnvironmentPlugin;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum UnderwaterState {
    #[default]
    Disabled,
    Enabled,
}

pub struct UnderwaterPlugin;

impl Plugin for UnderwaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<UnderwaterState>()
           .add_plugins((
               SubmarinePlugin,
               TerrainPlugin,
               EnvironmentPlugin,
           ))
           .add_systems(OnEnter(UnderwaterState::Enabled), setup_underwater_scene)
           .add_systems(OnExit(UnderwaterState::Enabled), cleanup_underwater_scene);
    }
}

#[derive(Component)]
pub struct UnderwaterMarker;

pub fn setup_underwater_scene(mut commands: Commands) {
    // Initial scene setup
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.1, 0.1, 0.3),
        brightness: 0.3,
    });
}

fn cleanup_underwater_scene(
    mut commands: Commands,
    query: Query<Entity, With<UnderwaterMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
