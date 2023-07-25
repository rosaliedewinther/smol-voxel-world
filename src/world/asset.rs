use crate::io::{read_and_decompress_file, read_file, write_and_compress_to_file};

use super::voxelized::MeshGridBitfield;
use anyhow::{bail, Context, Result};
use bvh::{
    aabb::Bounded,
    bounding_hierarchy::BHShape,
    ray::{Intersection, Ray},
    Point3, Vector3,
};
use glam::{uvec3, vec3, UVec3, Vec3};
use log::debug;
use std::{
    f32::{EPSILON, INFINITY},
    io::BufReader,
    path::Path,
    time::Instant,
};
use tobj::{load_obj, load_obj_buf, Model};

pub fn cache_file(file: &str) -> Result<Vec<u8>> {
    if !Path::new(&file).exists() {
        bail!("file {file} does not exist");
    }
    let compressed_file = file.to_string() + ".compressed";
    if Path::new(&compressed_file).exists() {
        read_and_decompress_file(&compressed_file)
    } else {
        let data = read_file(file)?;
        write_and_compress_to_file(&data, &compressed_file)?;
        Ok(data)
    }
}

struct Triangle {
    p0: Vector3,
    p1: Vector3,
    p2: Vector3,
    node_index: usize,
}
impl Bounded for Triangle {
    fn aabb(&self) -> bvh::aabb::AABB {
        let min = self.p0.min(self.p1).min(self.p2);
        let max = self.p0.max(self.p1).max(self.p2);

        bvh::aabb::AABB::with_bounds(Point3::new(min.x, min.y, min.z), Point3::new(max.x, max.y, max.z))
    }
}

impl BHShape for Triangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}
pub fn intersects_triangle(ray: &Ray, a: &Point3, b: &Point3, c: &Point3) -> Intersection {
    let a_to_b = *b - *a;
    let a_to_c = *c - *a;

    // Begin calculating determinant - also used to calculate u parameter
    // u_vec lies in view plane
    // length of a_to_c in view_plane = |u_vec| = |a_to_c|*sin(a_to_c, dir)
    let u_vec = ray.direction.cross(a_to_c);

    // If determinant is near zero, ray lies in plane of triangle
    // The determinant corresponds to the parallelepiped volume:
    // det = 0 => [dir, a_to_b, a_to_c] not linearly independant
    let det = a_to_b.dot(u_vec);

    // Only testing positive bound, thus enabling backface culling
    // If backface culling is not desired write:
    // det < EPSILON && det > -EPSILON
    if det < EPSILON && det > -EPSILON {
        return Intersection::new(INFINITY, 0.0, 0.0);
    }

    let inv_det = 1.0 / det;

    // Vector from point a to ray origin
    let a_to_origin = ray.origin - *a;

    // Calculate u parameter
    let u = a_to_origin.dot(u_vec) * inv_det;

    // Test bounds: u < 0 || u > 1 => outside of triangle
    if !(0.0..=1.0).contains(&u) {
        return Intersection::new(INFINITY, u, 0.0);
    }

    // Prepare to test v parameter
    let v_vec = a_to_origin.cross(a_to_b);

    // Calculate v parameter and test bound
    let v = ray.direction.dot(v_vec) * inv_det;
    // The intersection lies outside of the triangle
    if v < 0.0 || u + v > 1.0 {
        return Intersection::new(INFINITY, u, v);
    }

    let dist = a_to_c.dot(v_vec) * inv_det;

    if dist > EPSILON {
        Intersection::new(dist, u, v)
    } else {
        Intersection::new(INFINITY, u, v)
    }
}

pub fn place_in_bitfield(grid: &mut MeshGridBitfield, size: u32, models: Vec<Model>) {
    let mut max_dim = Vec3::splat(f32::MIN);
    let mut min_dim = Vec3::splat(f32::MAX);

    for model in &models {
        for vertex in model.mesh.positions.chunks(3) {
            let point = vec3(vertex[0], vertex[1], vertex[2]);

            max_dim = Vec3::max(max_dim, point);
            min_dim = Vec3::min(min_dim, point);
        }
    }

    let scale_factor = (size as f32 - 3f32) / f32::abs(max_dim.max_element() - min_dim.min_element()) / 4f32;
    let offset = ((size as f32 - 1f32) / 2f32) - (max_dim + min_dim) * scale_factor / 2f32;
    println!(
        "{} {} {} {} {} {}",
        min_dim,
        max_dim,
        scale_factor,
        offset,
        min_dim * scale_factor + offset,
        max_dim * scale_factor + offset
    );
    for model in models {
        let mut primitives = Vec::with_capacity(model.mesh.indices.len() / 9);
        model.mesh.indices.chunks(3).for_each(|index| {
            let vertex0 = vec3(
                model.mesh.positions[index[0] as usize * 3],
                model.mesh.positions[index[0] as usize * 3 + 1],
                model.mesh.positions[index[0] as usize * 3 + 2],
            ) * scale_factor
                + offset;
            let vertex1 = vec3(
                model.mesh.positions[index[1] as usize * 3],
                model.mesh.positions[index[1] as usize * 3 + 1],
                model.mesh.positions[index[1] as usize * 3 + 2],
            ) * scale_factor
                + offset;
            let vertex2 = vec3(
                model.mesh.positions[index[2] as usize * 3],
                model.mesh.positions[index[2] as usize * 3 + 1],
                model.mesh.positions[index[2] as usize * 3 + 2],
            ) * scale_factor
                + offset;

            let triangle = Triangle {
                p2: Vector3::new(vertex0.x, vertex0.y, vertex0.z),
                p1: Vector3::new(vertex1.x, vertex1.y, vertex1.z),
                p0: Vector3::new(vertex2.x, vertex2.y, vertex2.z),
                node_index: 0,
            };
            primitives.push(triangle);
        });
        let bvh = bvh::bvh::BVH::build(&mut primitives);
        //bvh.pretty_print();
        println!("built bvh");

        for z in 0..size {
            for y in 0..size {
                for x in 0..size {
                    let origin = Point3::new(x as f32, y as f32, z as f32) + 0.5;
                    let dir = Vector3::new(1 as f32, 0 as f32, 0 as f32);
                    let ray = Ray::new(origin, dir);
                    let hit_primitives = bvh.traverse(&ray, &primitives);
                    let hit_primitives: Vec<&Triangle> = hit_primitives
                        .into_iter()
                        .filter(|val| intersects_triangle(&ray, &val.p0, &val.p1, &val.p2).distance < INFINITY)
                        .collect();
                    if hit_primitives.len() > 0 && hit_primitives.len() % 2 == 1 {
                        grid.set_bit(UVec3 { x, y, z }, true);
                    }
                }
            }
        }
    }
}

pub fn load_obj_to_bitfield<T, B>(mesh_file: &str, size: u32) -> Result<MeshGridBitfield> {
    let data = cache_file(mesh_file)?;
    let compressed_file = mesh_file.to_string() + ".bitcompressed";
    if Path::new(&compressed_file).exists() {
        read_and_decompress_file(&compressed_file)
    } else {
        let mut bitfield = MeshGridBitfield::new(mesh_file, uvec3(size, size, size));

        let (models, _) = load_obj_buf(&mut BufReader::new(data.as_slice()), &tobj::GPU_LOAD_OPTIONS, |_| {
            panic!("remove materials from {mesh_file}")
        })
        .expect("Could not parse obj");

        place_in_bitfield(&mut bitfield, size, models);
        write_and_compress_to_file(&bitfield, &compressed_file)?;
        Ok(bitfield)
    }
}
