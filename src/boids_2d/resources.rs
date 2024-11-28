use bevy::prelude::*;
use bevy_egui::egui::emath::align;
use crate::boids_2d::components::*;

#[derive(Resource)]
pub struct BoidSettings {
    pub count: usize,
    pub previous_count: usize,
    pub cohesion_range: f32,
    pub alignment_range: f32,
    pub separation_range: f32,
    pub min_distance_between_boids: f32,
    pub cohesion_coeff: f32,
    pub alignment_coeff: f32,
    pub separation_coeff: f32,
    pub collision_coeff: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub boid_type: BoidType,
    pub bounce_against_walls: bool
}

impl Default for BoidSettings {
    fn default() -> Self {
        BoidSettings {
            count: 500,
            previous_count: 500,
            alignment_range: 100.0,
            cohesion_range: 50.0,
            separation_range: 20.0,
            min_distance_between_boids: 16.0,
            cohesion_coeff: 20.0,
            alignment_coeff: 30.0,
            separation_coeff: 20.0,
            collision_coeff: 40.0,
            min_speed: 200.0,
            max_speed: 500.0,
            boid_type: BoidType::Fish,
            bounce_against_walls: false
        }
    }
}

impl BoidSettings {
    pub fn new(count: usize, alignment_range: f32, cohesion_range: f32, separation_range: f32, boid_type: BoidType) -> Self {
        BoidSettings {
            count: count,
            previous_count: count,
            alignment_range: alignment_range,
            cohesion_range: cohesion_range,
            separation_range: separation_range,
            boid_type: boid_type,
            ..Default::default()
        }
    }
}