use bytemuck::Pod;
use glam::UVec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MeshGridBitfield {
    grid_name: String,
    dimensions: UVec3,
    data: Vec<u32>,
}

impl MeshGridBitfield {
    pub fn new(name: &str, dimensions: UVec3) -> Self {
        Self {
            grid_name: name.to_owned(),
            dimensions: dimensions,
            data: vec![0u32; ((dimensions.x / 32) * dimensions.y * dimensions.z) as usize],
        }
    }
    pub fn set_bit(&mut self, position: UVec3, value: bool) {
        assert!(position.x < self.dimensions.x);
        assert!(position.y < self.dimensions.y);
        assert!(position.z < self.dimensions.z);
        let uint_index = position.x + position.y * self.dimensions.x + position.z * self.dimensions.x * self.dimensions.y / 32;
        let bit = 1u32 << ((position.x + position.y * self.dimensions.x + position.z * self.dimensions.x * self.dimensions.y) % 32);
        match value {
            true => self.data[uint_index as usize] |= bit,
            false => self.data[uint_index as usize] &= !bit,
        }
    }
}
