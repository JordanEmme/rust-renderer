use std::cmp::Ordering;
use super::bounding_box;

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
}

impl Coords2D {
    pub fn add_vector(&mut self, u: f32, v: f32) {
        self.us.push(u);
        self.vs.push(v);
    }
}

impl Mesh {
    pub fn bounding_box(&self) -> bounding_box::BoundingBox {
        let (min_x, max_x) = get_vec_min_max(&self.v_positions.xs);
        let (min_y, max_y) = get_vec_min_max(&self.v_positions.ys);
        let (min_z, max_z) = get_vec_min_max(&self.v_positions.zs);

        return bounding_box::BoundingBox {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        };
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
    (min, max)
}
