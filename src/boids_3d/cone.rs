use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use std::f32::consts::PI;

/// Un cône avec un sommet et une base circulaire.
#[derive(Debug, Clone, Copy)]
pub struct Cone {
    /// Le rayon de la base du cône.
    pub radius: f32,
    /// La hauteur du cône.
    pub height: f32,
    /// Le nombre de segments sur la base.
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

        // Génération des sommets (vertices) et indices
        let mut vertices: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let height = cone.height;
        let radius = cone.radius;
        let segments = cone.segments;

        // Ajouter le sommet de la pointe
        vertices.push([0.0, 0.0, 0.0]); // La pointe (axe Z négatif)
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0.5, 1.0]);

        // Ajouter les sommets de la base
        for i in 0..=segments {
            let angle = i as f32 * std::f32::consts::TAU / segments as f32;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            vertices.push([x, y, -height]);
            normals.push([x, y, 0.0]);
            uvs.push([(x / radius + 1.0) * 0.5, (y / radius + 1.0) * 0.5]);
        }

        // Génération des indices pour la surface latérale
        for i in 1..=segments {
            indices.push(0); // Pointe du cône
            indices.push(i as u32);
            indices.push((i + 1) as u32);
        }

        // Génération des indices pour la base
        let base_center_index = vertices.len() as u32;
        vertices.push([0.0, 0.0, -height]); // Centre de la base
        normals.push([0.0,0.0,-1.0]);
        uvs.push([0.5, 0.5]);
        for i in 1..=segments {
            indices.push(base_center_index);
            indices.push((i + 1) as u32);
            indices.push(i as u32);
        }

        // Assigner les attributs au maillage
        Mesh::new(PrimitiveTopology::TriangleList)
        .with_indices(Some(Indices::U32(indices)))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}