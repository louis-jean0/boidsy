use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use crate::underwater::UnderwaterMarker;
use crate::boids_2d::components::ObstacleTag;
use super::components::*;

pub fn setup_submarine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn submarine
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/submarine/scene.gltf#Scene0"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, -1.0),
                ..default()
            },
            ..default()
        },
        Submarine::default(),
        UnderwaterMarker,
    )).with_children(|parent| {
        parent.spawn(SpotLightBundle {
            spot_light: SpotLight {
                color: Color::rgb(1.0, 0.95, 0.8),
                intensity: 40000.0,
                range: 100.0,
                outer_angle: 0.8,
                inner_angle: 0.6,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 5.0),
                scale: Vec3::new(1.0, 1.0, -1.0),
                rotation: Quat::from_rotation_x(-0.2)
            },
            ..default()
        });
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-100.0, 2.0, -100.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 1,
                ..default()
            },
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgb(0.1, 0.1, 0.5)),
                ..default()
            },
            ..default()
        },
        FogSettings {
            color: Color::rgb(0.25, 0.25, 0.75),
            falloff: FogFalloff::Linear { 
                start: 30.0, 
                end: 200.0 
            },
            ..default()
        },
        SubmarineCamera::default(),
        UnderwaterMarker,
        ObstacleTag
    ));
}

pub fn submarine_movement(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut submarine_query: Query<(&mut Transform, &mut Submarine)>
) {
    for (mut transform, mut submarine) in submarine_query.iter_mut() {
        let mut movement = Vec3::ZERO;
        let mut rotation = 0.0;
        let mut pitch_change = 0.0;

        // Forward/Backward
        if keyboard.pressed(KeyCode::Z) {
            movement += transform.forward() * submarine.speed;
        }
        if keyboard.pressed(KeyCode::S) {
            movement += transform.forward() * -submarine.speed;
        }

        // Rotation (Yaw)
        if keyboard.pressed(KeyCode::Q) {
            rotation += submarine.turn_speed;
        }
        if keyboard.pressed(KeyCode::D) {
            rotation -= submarine.turn_speed;
        }

        // Up/Down with pitch
        if keyboard.pressed(KeyCode::Space) {
            movement.y += submarine.vertical_speed;
            pitch_change += submarine.turn_speed * 0.5; // Reduced pitch rate
        }
        if keyboard.pressed(KeyCode::ShiftLeft) {
            movement.y -= submarine.vertical_speed;
            pitch_change -= submarine.turn_speed * 0.5; // Reduced pitch rate
        }

        // Update and clamp pitch
        submarine.current_pitch = (submarine.current_pitch + pitch_change * time.delta_seconds())
            .clamp(-submarine.max_pitch, submarine.max_pitch);

        // Return to level when not pressing up/down
        if !keyboard.pressed(KeyCode::Space) && !keyboard.pressed(KeyCode::ShiftLeft) {
            submarine.current_pitch = submarine.current_pitch * 0.95; // Gradual return to level
        }

        // Apply movement and rotation
        transform.translation += movement * time.delta_seconds();
        transform.rotate_y(rotation * time.delta_seconds());
        
        // Set absolute rotation for pitch to prevent accumulation
        let yaw = transform.rotation.to_euler(EulerRot::YXZ).0;
        transform.rotation = Quat::from_euler(
            EulerRot::YXZ,
            yaw,
            submarine.current_pitch,
            0.0
        );
    }
}

pub fn update_camera(
    submarine_query: Query<&Transform, (With<Submarine>, Without<SubmarineCamera>)>,
    mut camera_query: Query<(&mut Transform, &SubmarineCamera), Without<Submarine>>,
    time: Res<Time>,
) {
    let Ok(submarine_transform) = submarine_query.get_single() else { return };
    
    for (mut camera_transform, camera_settings) in camera_query.iter_mut() {
        let target = submarine_transform.translation;
        let desired_position = target - submarine_transform.forward() * camera_settings.follow_distance
            + Vec3::Y * camera_settings.height_offset;

        camera_transform.translation = camera_transform.translation.lerp(
            desired_position,
            time.delta_seconds() * 5.0
        );

        camera_transform.look_at(target, Vec3::Y);
    }
}