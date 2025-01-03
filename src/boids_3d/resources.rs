use bevy::prelude::*;

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
    pub bounce_against_walls: bool,
    pub attraction_coeff: f32,
    pub field_of_view: f32
}

impl Default for BoidSettings {
    fn default() -> Self {
        BoidSettings {
            count: 500,
            previous_count: 500,
            alignment_range: 30.0,
            cohesion_range: 10.0,
            separation_range: 20.0,
            min_distance_between_boids: 20.0,
            cohesion_coeff: 20.0,
            alignment_coeff: 5.0,
            separation_coeff: 20.0,
            collision_coeff: 24.0,
            min_speed: 500.0,
            max_speed: 1000.0,
            bounce_against_walls: true,
            attraction_coeff: 1.0,
            field_of_view: 90.0
        }
    }
}

impl BoidSettings {
    pub fn new(count: usize, alignment_range: f32, cohesion_range: f32, separation_range: f32) -> Self {
        BoidSettings {
            count: count,
            previous_count: count,
            alignment_range: alignment_range,
            cohesion_range: cohesion_range,
            separation_range: separation_range,
            ..Default::default()
        }
    }
}

#[derive(Resource)]
pub struct GroupsTargets {
    pub targets: Vec<Vec2>
}

impl Default for GroupsTargets {
    fn default() -> Self {
        GroupsTargets {
            targets: vec![
                Vec2::new(1290.0,540.0),
                Vec2::new(430.0,540.0)
            ]
        }
    }
}