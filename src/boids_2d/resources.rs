use bevy::prelude::*;
use crate::boids_2d::components::*;

#[derive(Resource)]
pub struct BoidSettings {
    pub count: usize,
    pub visual_range: f32,
    pub separation_range: f32,
    pub min_distance_between_boids: f32,
    pub cohesion_coeff: f32,
    pub alignment_coeff: f32,
    pub separation_coeff: f32,
    pub collision_coeff: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub boid_type: BoidType
}

impl Default for BoidSettings {
    fn default() -> Self {
        BoidSettings {
            count: 500,
            visual_range: 100.0,
            separation_range: 20.0,
            min_distance_between_boids: 16.0,
            cohesion_coeff: 20.0,
            alignment_coeff: 30.0,
            separation_coeff: 20.0,
            collision_coeff: 40.0,
            min_speed: 200.0,
            max_speed: 500.0,
            boid_type: BoidType::Fish
        }
    }
}

impl BoidSettings {
    pub fn new(count: usize, visual_range: f32, separation_range: f32, boid_type: BoidType) -> Self {
        BoidSettings {
            count: count,
            visual_range: visual_range,
            separation_range: separation_range,
            boid_type: boid_type,
            ..Default::default()
        }
    }
}