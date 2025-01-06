use bevy::prelude::*;

#[derive(Resource)]
pub struct BoidSettings2D {
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

impl Default for BoidSettings2D {
    fn default() -> Self {
        BoidSettings2D {
            count: 2000,
            previous_count: 2000,
            cohesion_range: 50.0,
            alignment_range: 30.0,
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