use super::{bounding_box, linear_algebra};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Copy, Clone, Debug)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Debug)]
pub struct Coords3D {
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
    pub zs: Vec<f32>,
}

#[derive(Clone, Debug)]
pub struct Coords2D {
    pub us: Vec<f32>,
    pub vs: Vec<f32>,
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub vertices: [usize; 3],
    pub normals: [usize; 3],
    pub textures: [usize; 3],
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub v_positions: Coords3D,
    pub v_normals: Coords3D,
    pub v_textures: Coords2D,
    pub triangles: Vec<Triangle>,
}

impl Coords3D {
    pub fn add_vector(&mut self, x: f32, y: f32, z: f32) {
        self.xs.push(x);
        self.ys.push(y);
        self.zs.push(z);
    }

    pub fn get_at(&self, i: usize) -> Point3D<f32> {
        return Point3D {
            x: self.xs[i],
            y: self.ys[i],
            z: self.zs[i],
        };
    }

    pub fn get_at_in_projective_space(
        &self,
        i: usize,
        observer_distance: f32,
        focal_length: f32,
    ) -> (f32, f32) {
        let p = self.get_at(i);
        let t: f32 = focal_length / (observer_distance - p.z);
        return (t * p.x, t * p.y);
    }

    pub fn get_at_orthonormal_projection(&self, i: usize) -> Point2D<f32> {
        let p = self.get_at(i);
        return Point2D { x: p.x, y: p.y };
    }
}

impl Coords2D {
    pub fn add_vector(&mut self, u: f32, v: f32) {
        self.us.push(u);
        self.vs.push(v);
    }

    pub fn get_at(&self, i: usize) -> (f32, f32) {
        return (self.us[i], self.vs[i]);
    }
}

impl Mesh {
    pub fn bounding_box(&self) -> bounding_box::BoundingBox3D {
        let (min_x, max_x) = get_vec_min_max(&self.v_positions.xs);
        let (min_y, max_y) = get_vec_min_max(&self.v_positions.ys);
        let (min_z, max_z) = get_vec_min_max(&self.v_positions.zs);

        return bounding_box::BoundingBox3D {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        };
    }

    pub fn triangle_normal(&self, triangle: &Triangle) -> Point3D<f32> {
        return linear_algebra::get_plane_normal(
            self.v_positions.get_at(triangle.vertices[0]),
            self.v_positions.get_at(triangle.vertices[1]),
            self.v_positions.get_at(triangle.vertices[2]),
        );
    }

    pub fn triangle_is_backface(&self, triangle: &Triangle, view_direction: Point3D<f32>) -> bool {
        let triangle_normal = self.triangle_normal(triangle);
        return linear_algebra::dot_product(triangle_normal, view_direction) > 0f32;
    }
}

fn get_vec_min_max(elements: &Vec<f32>) -> (f32, f32) {
    let mut min = elements[0];
    let mut max = elements[0];
    for element in elements {
        match element.partial_cmp(&min) {
            Some(Ordering::Less) => min = *element,
            _ => (),
        }
        match element.partial_cmp(&max) {
            Some(Ordering::Greater) => max = *element,
            _ => (),
        }
    }
    return (min, max);
}
