use bevy::prelude::*;

pub mod systems;
pub use systems::*;

pub struct UiPlugin;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
    	app.add_plugins(FrameTimeDiagnosticsPlugin::default())
    	.add_systems(Startup, setup_fps_counter)
        .add_systems(Update, setup_ui)
        .add_systems(Update, (
		    fps_text_update_system,
		    fps_counter_showhide,
		));
    }
}