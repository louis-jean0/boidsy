use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::window::PrimaryWindow;
use crate::input::resources::*;
use crate::boids_3d::resources::CameraControlState;

use crate::boids_2d::components::ObstacleTag;
use crate::boids_2d::systems::spawn_obstacle_2d;
use crate::boids_2d::systems::remove_all_obstacles;
use crate::boids_3d::systems::spawn_obstacle_3d;
use crate::ui::resources::SimulationState;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseMotion;

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
                        Vec3::new(0.0, 0.0, 0.5),
                        shape_settings.radius,
                        &mut meshes,
                        &mut standard_materials
                    );
                }
                SimulationState::Underwater => {
                    return;
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

pub fn handle_camera_control(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    keyboard: Res<Input<KeyCode>>,
    mut camera_control: ResMut<CameraControlState>,
    simulation_state: Res<State<SimulationState>>,
) {
    if *simulation_state.get() != SimulationState::Mode3D && *simulation_state.get() != SimulationState::Underwater {
        return;
    }

    if keyboard.just_pressed(KeyCode::E) {
        let mut window = windows.single_mut();
        camera_control.is_active = !camera_control.is_active;
        
        if camera_control.is_active {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        } else {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
}

pub fn handle_camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut settings: ResMut<MouseSettings>,
    mut query: Query<&mut Transform, With<Camera3d>>,
    camera_control_state: ResMut<CameraControlState>
) {
    if !camera_control_state.is_active {return;}
    let Ok(mut transform) = query.get_single_mut() else { return };
    
    for ev in mouse_motion.read() {
        settings.pitch = (settings.pitch - ev.delta.y * settings.sensitivity)
            .clamp(-1.54, 1.54);
        settings.yaw -= ev.delta.x * settings.sensitivity;
    }

    let rotation = Quat::from_euler(EulerRot::YXZ, settings.yaw, settings.pitch, 0.0);
    transform.rotation = rotation;

    let mut movement = Vec3::ZERO;
    let mut speed = 500.0;

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        speed *= 2.0;
    }

    let forward = transform.forward();
    let right = transform.right();
    
    if keyboard_input.pressed(KeyCode::Z) {
        movement += forward;
    }
    if keyboard_input.pressed(KeyCode::S) {
        movement -= forward;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        movement -= right;
    }
    if keyboard_input.pressed(KeyCode::D) {
        movement += right;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        movement.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        movement.y -= 1.0;
    }

    transform.translation += movement.normalize_or_zero() * speed * time.delta_seconds();
}