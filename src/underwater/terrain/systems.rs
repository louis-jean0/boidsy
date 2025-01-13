use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use crate::boids_2d::components::ObstacleTag;
use crate::underwater::UnderwaterMarker;

use super::{components::*, TerrainChunk};
use noise::{NoiseFn, Perlin};
use super::marching_cubes::{generate_vertices, CORNERS};

const DEEP_COLOR: Color = Color::rgb(0.1, 0.2, 0.4);
const MID_COLOR: Color = Color::rgb(0.2, 0.5, 0.4);
const HIGH_COLOR: Color = Color::rgb(0.8, 0.6, 0.1);

const ISOLEVEL: f32 = 0.7;
pub const TERRAIN_SIZE: f32 = 500.0;
pub const TERRAIN_SCALE: f32 = 75.0;
const TERRAIN_HEIGHT_SCALE: f32 = 1.0;
const CHUNK_RANGE: i32 = 2;
pub const GROUND_Y_POSITION: f32 = -30.0;
const CHUNK_OVERLAP: u32 = 1;

pub fn generate_terrain_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = TERRAIN_SIZE;
    let divisions = 2000;
    let bump_height = 15.0;
    let perlin = Perlin::new(3);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for z in 0..=divisions {
        for x in 0..=divisions {
            let px = x as f32 / divisions as f32;
            let pz = z as f32 / divisions as f32;

            let noise1 = perlin.get([px as f64 * 3.0, pz as f64 * 3.0]) as f32;
            let noise2 = perlin.get([px as f64 * 6.0, pz as f64 * 6.0]) as f32 * 0.5;
            let height = (noise1 + noise2) * bump_height;

            vertices.push([
                (px - 0.5) * size,
                height + GROUND_Y_POSITION,
                (pz - 0.5) * size,
            ]);
        }
    }

    for z in 0..divisions {
        for x in 0..divisions {
            let tl = z * (divisions + 1) + x;
            let tr = tl + 1;
            let bl = (z + 1) * (divisions + 1) + x;
            let br = bl + 1;

            indices.extend_from_slice(&[
                tl as u32, bl as u32, tr as u32,
                tr as u32, bl as u32, br as u32,
            ]);
        }
    }

    let normals = calculate_normals(&vertices, &indices);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: DEEP_COLOR,
                metallic: 0.2,
                perceptual_roughness: 0.9,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        UnderwaterMarker,
        ObstacleTag
    ));

    let terrain_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    for chunk_x in -CHUNK_RANGE..=CHUNK_RANGE {
        for chunk_z in -CHUNK_RANGE..=CHUNK_RANGE {
            let chunk_position = IVec3::new(chunk_x, 0, chunk_z);
            generate_chunk(
                &mut commands,
                &mut meshes,
                &terrain_material,
                chunk_position,
            );
        }
    }
}

fn generate_chunk(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>, 
    material: &Handle<StandardMaterial>, 
    chunk_position: IVec3,
) {
    let mut density = DensityField::new(UVec3::new(CHUNK_SIZE + CHUNK_OVERLAP, CHUNK_SIZE + CHUNK_OVERLAP, CHUNK_SIZE + CHUNK_OVERLAP));
    let perlin = Perlin::new(1);
    
    for x in 0..=CHUNK_SIZE {
        for y in 0..=CHUNK_SIZE {
            for z in 0..=CHUNK_SIZE {
                let world_x = (x as f64 + (chunk_position.x * CHUNK_SIZE as i32) as f64) / CHUNK_SIZE as f64;
                let world_y = y as f64 / CHUNK_SIZE as f64;
                let world_z = (z as f64 + (chunk_position.z * CHUNK_SIZE as i32) as f64) / CHUNK_SIZE as f64;

                let base_height = y as f64 / CHUNK_SIZE as f64;
                
                // Large features (caves and mountains)
                let mountain_noise = perlin.get([
                    world_x * 1.5,
                    world_y * 0.8,
                    world_z * 1.5
                ]);
                
                // Medium features (overhangs and ledges)
                let medium_noise = perlin.get([
                    world_x * 3.0,
                    world_y * 3.0,
                    world_z * 3.0
                ]) * 0.6;
                
                // Small features (detail and roughness)
                let detail_noise = perlin.get([
                    world_x * 6.0,
                    world_y * 6.0,
                    world_z * 6.0
                ]) * 0.3;

                // Overhang bias (creates more overhangs)
                let overhang_bias = perlin.get([
                    world_x * 2.0,
                    world_z * 2.0,
                    0.0
                ]) * 0.4;

                // Combine all noise layers
                let combined_noise = (
                    mountain_noise +
                    medium_noise +
                    detail_noise +
                    overhang_bias
                ) as f32;

                let sharpness = 5.0;
                let value = (combined_noise * sharpness).tanh() * 0.5 + 0.5;
                
                let height_falloff = (base_height as f32 * 0.8).powf(2.0);
                
                let final_value = value - height_falloff;
                
                let index = density.get_index(x, y, z);
                density.values[index] = final_value;
            }
        }
    }

    let (vertices, indices) = generate_mesh(&density);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    
    let scaled_vertices: Vec<[f32; 3]> = vertices.iter()
        .map(|[x, y, z]| {
            let chunk_x = chunk_position.x as f32 * CHUNK_SIZE as f32;
            let chunk_z = chunk_position.z as f32 * CHUNK_SIZE as f32;
            [
                (x * CHUNK_SIZE as f32 + chunk_x),
                y * TERRAIN_SCALE * TERRAIN_HEIGHT_SCALE,
                (z * CHUNK_SIZE as f32 + chunk_z),
            ]
        })
        .collect();

    let colors: Vec<[f32; 4]> = scaled_vertices.iter()
        .map(|[_, y, _]| {
            let normalized_height = (*y / (TERRAIN_SCALE * TERRAIN_HEIGHT_SCALE) + 0.5).clamp(0.0, 1.0);
            
            if normalized_height < 0.7 {
                DEEP_COLOR.as_rgba_f32()
            } else if normalized_height < 0.9 {
                let t = (normalized_height - 0.4) / 0.3;
                Color::rgb(
                    lerp(DEEP_COLOR.r(), MID_COLOR.r(), t),
                    lerp(DEEP_COLOR.g(), MID_COLOR.g(), t),
                    lerp(DEEP_COLOR.b(), MID_COLOR.b(), t),
                ).as_rgba_f32()
            } else {
                let t = (normalized_height - 0.7) / 0.3;
                Color::rgb(
                    lerp(MID_COLOR.r(), HIGH_COLOR.r(), t),
                    lerp(MID_COLOR.g(), HIGH_COLOR.g(), t),
                    lerp(MID_COLOR.b(), HIGH_COLOR.b(), t),
                ).as_rgba_f32()
            }
        })
        .collect();
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, scaled_vertices.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    let normals = calculate_normals(&scaled_vertices, &indices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: material.clone(),
            transform: Transform::from_xyz(0.0, GROUND_Y_POSITION, 0.0),
            ..default()
        },
        TerrainChunk {
            position: chunk_position,
        },
        UnderwaterMarker,
        ObstacleTag
    ));
}

fn generate_mesh(density: &DensityField) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let scale = 1.0 / CHUNK_SIZE as f32;

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let pos = Vec3::new(x as f32, y as f32, z as f32);
                
                let mut cube_values = [0.0; 8];
                let mut cube_positions = [Vec3::ZERO; 8];
                
                for (i, (dx, dy, dz)) in CORNERS.iter().enumerate() {
                    let sx = (x as f32 + dx) as u32;
                    let sy = (y as f32 + dy) as u32;
                    let sz = (z as f32 + dz) as u32;
                    
                    cube_values[i] = density.get_value(sx, sy, sz);
                    cube_positions[i] = (pos + Vec3::new(*dx, *dy, *dz)) * scale;
                }

                let cube_vertices = generate_vertices(&cube_positions, &cube_values, ISOLEVEL);
                
                for vertex in cube_vertices {
                    indices.push(vertices.len() as u32);
                    vertices.push([vertex.x, vertex.y, vertex.z]);
                }
            }
        }
    }
    (vertices, indices)
}

fn calculate_normals(vertices: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let mut normals = vec![[0.0, 0.0, 0.0]; vertices.len()];
    
    for triangle in indices.chunks(3) {
        if triangle.len() == 3 {
            let v0 = Vec3::from(vertices[triangle[0] as usize]);
            let v1 = Vec3::from(vertices[triangle[1] as usize]);
            let v2 = Vec3::from(vertices[triangle[2] as usize]);
            
            let normal = (v1 - v0).cross(v2 - v0).normalize();
            
            for &index in triangle {
                normals[index as usize][0] += normal.x;
                normals[index as usize][1] += normal.y;
                normals[index as usize][2] += normal.z;
            }
        }
    }
    
    normals.iter_mut().for_each(|n| {
        let normal = Vec3::from(*n).normalize();
        *n = [normal.x, normal.y, normal.z];
    });
    
    normals
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}