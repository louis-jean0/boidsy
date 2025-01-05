use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::input::resources::*;

use crate::boids_2d::components::ObstacleTag;
use crate::boids_2d::systems::spawn_obstacle_2d;
use crate::boids_2d::systems::remove_all_obstacles;
use crate::boids_3d::systems::spawn_obstacle_3d;
use crate::ui::resources::SimulationState;
use bevy::input::mouse::MouseWheel;

pub fn mouse_buttons_input(
    mouse_buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    shape_settings: Res<ShapeSettings>,
    keys: Res<Input<KeyCode>>,
    query: Query<Entity, With<ObstacleTag>>,
    simulation_state: Res<State<SimulationState>>
) {
    if *simulation_state.get() != SimulationState::Mode2D {return;}
    if mouse_buttons.pressed(MouseButton::Right) {
        if let Some(position) = cursor_position(&q_windows) {
            let window = q_windows.get_single().unwrap();
            match simulation_state.get() {
                SimulationState::Mode2D => {
                    spawn_obstacle_2d(
                        &mut commands,
                        Vec2::new(position.x, position.y),
                        Vec3::new(position.x / window.width(), position.y / window.height(), 0.5),
                        shape_settings.radius,
                        &mut meshes,
                        &mut color_materials
                    );
                }
                SimulationState::Mode3D => {
                    spawn_obstacle_3d(
                        &mut commands,
                        Vec3::new(position.x, position.y, 0.0),
                        Vec3::new(position.x / window.width(), position.y / window.height(), 0.5),
                        shape_settings.radius,
                        &mut meshes,
                        &mut standard_materials
                    );
                }
            }
        }
    }

    if keys.just_pressed(KeyCode::R) {
        remove_all_obstacles(commands, query);
    }
}

pub fn cursor_position(
    q_windows: &Query<&Window, With<PrimaryWindow>>,
) -> Option<Vec2> {
    if let Some(cursor_pos) = q_windows.single().cursor_position() {
        Some(Vec2::new(cursor_pos.x, q_windows.get_single().unwrap().height() - cursor_pos.y))
    } else {
        None
    }
}

pub fn scroll_events(
    mut evr_scroll: EventReader<MouseWheel>,
    mut shape_settings: ResMut<ShapeSettings>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
            	if ev.y > 0.0 {
	            	if shape_settings.radius < 100.0 {
	            		shape_settings.radius += 1.0;
	            	}
	            }
            	if ev.y < 0.0 {
            		if shape_settings.radius > 0.0 {
            		    shape_settings.radius -= 1.0;
            		}
            	}    
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
        }
    }
}