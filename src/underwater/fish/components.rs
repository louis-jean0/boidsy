use bevy::prelude::*;
use crate::boids_3d::resources::BoidSettings3D;
use std::ops::{Deref, DerefMut};

#[derive(Resource)]
pub struct UnderwaterBoidSettings(BoidSettings3D);

impl Default for UnderwaterBoidSettings {
    fn default() -> Self {
        Self(BoidSettings3D {
            count: 500,
            previous_count: 500,
            size: 1.0,
            cohesion_range: 30.0,
            alignment_range: 20.0,
            separation_range: 15.0,
            min_distance_between_boids: 30.0,
            cohesion_coeff: 15.0,
            alignment_coeff: 4.0,
            separation_coeff: 20.0,
            collision_coeff: 20.0,
            min_speed: 30.0,
            max_speed: 100.0,
            bounce_against_walls: true,
            attraction_coeff: 1.0,
            field_of_view: 270.0
        })
    }
}

impl Deref for UnderwaterBoidSettings {
    type Target = BoidSettings3D;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for UnderwaterBoidSettings {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[derive(Resource)]
pub struct FishModel(pub Handle<Scene>);

#[derive(Component)]
pub struct FishType {
    pub species: Species,
    pub school_id: usize,
}

#[derive(Clone)]
pub enum Species {
    SmallFish,
    MediumFish,
    LargeFish,
}

impl Species {
    pub fn get_settings(&self) -> UnderwaterBoidSettings {
        match self {
            Species::SmallFish => UnderwaterBoidSettings(BoidSettings3D {
                count: 200,
                size: 10.0,
                min_speed: 30.0,
                max_speed: 300.0,
                ..default()
            }),
            Species::MediumFish => UnderwaterBoidSettings(BoidSettings3D {
                count: 100,
                size: 0.1,
                min_speed: 20.0,
                max_speed: 100.0,
                ..default()
            }),
            Species::LargeFish => UnderwaterBoidSettings(BoidSettings3D {
                count: 50,
                size: 10.0,
                min_speed: 20.0,
                max_speed: 50.0,
                ..default()
            })
        }
    }
}

impl Default for Species {
    fn default() -> Self {
        Species::SmallFish
    }
}

#[derive(Resource)]
pub struct FishModels {
    pub small_fish: Handle<Scene>,
    pub medium_fish: Handle<Scene>,
    pub large_fish: Handle<Scene>,
}
