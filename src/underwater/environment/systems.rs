use bevy::prelude::*;
use rand::prelude::*;
use super::components::*;
use crate::underwater::{UnderwaterMarker, submarine::components::Submarine};

pub fn setup_environment(
    mut commands: Commands
) {
    // Initialize underwater effect resource
    commands.insert_resource(UnderwaterEffect::default());
}

pub fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut effect: ResMut<UnderwaterEffect>,
    time: Res<Time>,
    submarine_query: Query<&Transform, With<Submarine>>,
) {
    effect.particle_spawn_timer.tick(time.delta());

    if effect.particle_spawn_timer.just_finished() {
        if let Ok(submarine_transform) = submarine_query.get_single() {
            let mut rng = rand::thread_rng();
            
            // Spawn bubbles around the submarine
            let spawn_position = submarine_transform.translation + Vec3::new(
                rng.gen_range(-5.0..5.0),
                rng.gen_range(-2.0..0.0),
                rng.gen_range(-5.0..5.0),
            );

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere { 
                        radius: rng.gen_range(0.05..0.15),  // Slightly larger bubbles
                        ..default()
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(0.8, 0.8, 1.0, 0.3),
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    }),
                    transform: Transform::from_translation(spawn_position),
                    ..default()
                },
                Bubble {
                    velocity: Vec3::new(
                        rng.gen_range(-0.5..0.5),
                        rng.gen_range(2.0..4.0),  // Faster upward movement
                        rng.gen_range(-0.5..0.5),
                    ),
                    lifetime: Timer::from_seconds(rng.gen_range(2.0..4.0), TimerMode::Once),
                },
                UnderwaterMarker,
            ));
        }
    }
}

pub fn update_bubbles(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bubbles: Query<(Entity, &mut Transform, &mut Bubble, &Handle<StandardMaterial>)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut bubble, material_handle) in bubbles.iter_mut() {
        bubble.lifetime.tick(time.delta());
        if bubble.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Add some wavey motion
        bubble.velocity.x += (time.elapsed_seconds() * 2.0).sin() * 0.01;
        bubble.velocity.z += (time.elapsed_seconds() * 2.0).cos() * 0.01;
        
        transform.translation += bubble.velocity * time.delta_seconds();

        // Fade out near end of lifetime
        if let Some(material) = materials.get_mut(material_handle) {
            let alpha = (bubble.lifetime.percent() * 0.3).min(0.3);
            material.base_color.set_a(alpha);
        }
    }
}
