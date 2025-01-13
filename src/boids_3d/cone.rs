use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};

#[derive(Debug, Clone, Copy)]
pub struct Cone {
    pub radius: f32,
    pub height: f32,
    pub segments: usize,
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 5.0,
            segments: 12,
        }
    }
}

impl From<Cone> for Mesh {
    fn from(cone: Cone) -> Self {

        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let height = cone.height;
        let radius = cone.radius;
        let segments = cone.segments;

        vertices.push([0.0, 0.0, -height]);
        uvs.push([0.5, 1.0]);

        for i in 0..=segments {
            let angle = i as f32 * std::f32::consts::TAU / segments as f32;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            vertices.push([x, y, 0.0]);
            uvs.push([(x / radius + 1.0) * 0.5, (y / radius + 1.0) * 0.5]);
        }

        for i in 1..=segments {
            indices.push(0);
            indices.push(i as u32);
            indices.push((i + 1) as u32);
        }

        let base_center_index = vertices.len() as u32;
        vertices.push([0.0, 0.0, 0.0]);
        uvs.push([0.5, 0.5]);
        for i in 1..=segments {
            indices.push(base_center_index);
            indices.push((i + 1) as u32);
            indices.push(i as u32);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList)
            .with_indices(Some(Indices::U32(indices)))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        mesh.duplicate_vertices();
        mesh.compute_flat_normals();
        
        mesh
    }
}