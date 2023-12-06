use super::bounding_box;
use std::cmp::Ordering;

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

    pub fn get_at(&self, i: usize) -> (f32, f32, f32) {
        return (self.xs[i], self.ys[i], self.zs[i]);
    }

    pub fn get_at_in_projective_space(&self, i: usize, observer_distance: f32, focal_length: f32) -> (f32, f32) {
        let (x, y, z): (f32, f32, f32) = self.get_at(i);
        // let t: f32 = focal_length / (observer_distance - z);
        // return (t * x, t * y);
        (x,y)
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
}

impl Triangle {
    pub fn get_2d_bounding_box(
        &self,
        positions: &Coords3D,
        observer_distance: f32,
        focal_length: f32,
    ) -> (f32, f32, f32, f32) {
        let mut x_min: f32 = f32::MIN;
        let mut x_max: f32 = f32::MAX;
        let mut y_min: f32 = f32::MIN;
        let mut y_max: f32 = f32::MAX;

        self.vertices.iter().for_each(|&index| {
            let (proj_x, proj_y) = positions.get_at_in_projective_space(index, observer_distance, focal_length);
            x_min = x_min.min(proj_x);
            x_max = x_max.max(proj_x);
            y_min = y_min.min(proj_y);
            y_max = y_max.max(proj_y);
        });
        return (x_min, x_max, y_min, y_max);
    }

    pub fn bounding_box(&self, positions: &Coords3D) -> bounding_box::BoundingBox3D {
        let (x1, y1, z1) = positions.get_at(self.vertices[0]);
        let (x2, y2, z2) = positions.get_at(self.vertices[1]);
        let (x3, y3, z3) = positions.get_at(self.vertices[2]);
        let (min_x, max_x) = get_vec_min_max(&vec![x1, x2, x3]);
        let (min_y, max_y) = get_vec_min_max(&vec![y1, y2, y3]);
        let (min_z, max_z) = get_vec_min_max(&vec![z1, z2, z3]);

        return bounding_box::BoundingBox3D {
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
