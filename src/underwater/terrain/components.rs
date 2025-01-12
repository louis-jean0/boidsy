use bevy::prelude::*;

#[derive(Component)]
pub struct DensityField {
    pub values: Vec<f32>,
    pub size: UVec3,
}

impl DensityField {
    pub fn new(size: UVec3) -> Self {
        let total_size = (size.x * size.y * size.z) as usize;
        Self {
            values: vec![0.0; total_size],
            size,
        }
    }

    pub fn get_index(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.size.x + z * self.size.x * self.size.y) as usize
    }

    pub fn get_value(&self, x: u32, y: u32, z: u32) -> f32 {
        self.values[self.get_index(x, y, z)]
    }
}

pub const CHUNK_SIZE: u32 = 100;