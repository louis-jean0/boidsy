use bevy::prelude::*;

pub const CORNERS: [(f32, f32, f32); 8] = [
    (0.0, 0.0, 0.0), // 0
    (1.0, 0.0, 0.0), // 1
    (1.0, 1.0, 0.0), // 2
    (0.0, 1.0, 0.0), // 3
    (0.0, 0.0, 1.0), // 4
    (1.0, 0.0, 1.0), // 5
    (1.0, 1.0, 1.0), // 6
    (0.0, 1.0, 1.0), // 7
];

pub const EDGES: [(usize, usize); 12] = [
    (0, 1), (1, 2), (2, 3), (3, 0),  // bottom face
    (4, 5), (5, 6), (6, 7), (7, 4),  // top face
    (0, 4), (1, 5), (2, 6), (3, 7),  // vertical edges
];

pub fn interpolate_vertex(p1: Vec3, p2: Vec3, v1: f32, v2: f32, isolevel: f32) -> Vec3 {
    if (v2 - v1).abs() < 0.00001 {
        return p1;
    }
    
    let t = (isolevel - v1) / (v2 - v1);
    p1.lerp(p2, t)
}

pub fn generate_vertices(points: &[Vec3; 8], values: &[f32; 8], isolevel: f32) -> Vec<Vec3> {
    let mut vertices = Vec::new();
    let mut cubeindex = 0;

    // Determine which vertices are inside/outside the surface
    for i in 0..8 {
        if values[i] < isolevel {
            cubeindex |= 1 << i;
        }
    }

    // Calculate vertices where surface intersects cube
    let mut edge_vertices: [Option<Vec3>; 12] = [None; 12];
    
    for (i, edge) in EDGES.iter().enumerate() {
        if EDGE_TABLE[cubeindex] & (1 << i) != 0 {
            edge_vertices[i] = Some(interpolate_vertex(
                points[edge.0],
                points[edge.1],
                values[edge.0],
                values[edge.1],
                isolevel
            ));
        }
    }

    // Create triangles according to triangulation table
    let mut i = 0;
    while i < 16 && TRIANGLE_TABLE[cubeindex][i] != -1 {
        if let (Some(v1), Some(v2), Some(v3)) = (
            edge_vertices[TRIANGLE_TABLE[cubeindex][i] as usize],
            edge_vertices[TRIANGLE_TABLE[cubeindex][i + 1] as usize],
            edge_vertices[TRIANGLE_TABLE[cubeindex][i + 2] as usize],
        ) {
            vertices.push(v1);
            vertices.push(v2);
            vertices.push(v3);
        }
        i += 3;
    }

    vertices
}

include!("marching_cubes_tables.rs");
