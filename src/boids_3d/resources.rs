use bevy::prelude::*;

#[derive(Resource)]
pub struct BoidSettings3D {
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

impl Default for BoidSettings3D {
    fn default() -> Self {
        BoidSettings3D {
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

impl BoidSettings3D {
    pub fn new(count: usize, alignment_range: f32, cohesion_range: f32, separation_range: f32) -> Self {
        BoidSettings3D {
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
    pub targets: Vec<Vec3>
}

impl Default for GroupsTargets {
    fn default() -> Self {
        GroupsTargets {
            targets: vec![
                Vec3::new(1290.0,540.0,10.0),
                Vec3::new(430.0,540.0,10.0)
            ]
        }
    }
}