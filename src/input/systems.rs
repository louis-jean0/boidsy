use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::input::resources::*;

use crate::boids_2d::systems::*;
use bevy::input::mouse::MouseWheel;

pub fn mouse_buttons_input(
    mouse_buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    shape_settings: Res<ShapeSettings>,
    keys: Res<Input<KeyCode>>,
    query: Query<Entity, With<ObstacleTag>>, // Ajout pour accéder aux obstacles
) {
    if mouse_buttons.pressed(MouseButton::Right) {
        if let Some(position) = cursor_position(&q_windows) {
            let window = q_windows.get_single().unwrap();
            spawn_obstacle(
                &mut commands,
                Vec2::new(position.x, window.height() - position.y),
                Vec3::new(position.x, position.y, 0.5),
                shape_settings.radius,
                &mut meshes,
                &mut materials,
            );
        }
    }

    if keys.just_pressed(KeyCode::R) {
        println!("R pressed");
        remove_all_obstacles(commands, query);
    }
}

pub fn cursor_position(
    q_windows: &Query<&Window, With<PrimaryWindow>>,
) -> Option<Vec2> {
    q_windows.single().cursor_position()
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