use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct Coords3D<T: PartialOrd + Copy> {
    pub xs: Vec<T>,
    pub ys: Vec<T>,
    pub zs: Vec<T>,
}

#[derive(Clone, Debug)]
pub struct Coords2D<T: PartialOrd + Copy> {
    pub us: Vec<T>,
    pub vs: Vec<T>,
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub vertices: [usize; 3],
    pub normals: [usize; 3],
    pub textures: [usize; 3],
}

#[derive(Clone, Debug)]
pub struct Mesh<T: PartialOrd + Copy> {
    pub v_positions: Coords3D<T>, 
    pub v_normals: Coords3D<T>,
    pub v_textures: Coords2D<T>,
    pub triangles: Vec<Triangle>,
}

#[derive(Copy, Clone, Debug)]
pub struct BoundingBox<T: PartialOrd + Copy> {
    pub min_x: T,
    pub min_y: T,
    pub min_z: T,
    pub max_x: T,
    pub max_y: T,
    pub max_z: T,
}

impl <T: PartialOrd + Copy> Coords3D<T> {
    pub fn add_vector(&mut self, x: T, y: T, z: T){
        self.xs.push(x);
        self.ys.push(y);
        self.zs.push(z);
    }
}

impl <T: PartialOrd + Copy> Coords2D<T> {
    pub fn add_vector(&mut self, u: T, v: T) {
        self.us.push(u);
        self.vs.push(v);
    }
}

impl<T: PartialOrd + Copy> Mesh<T> {

    fn bounding_box(&self) -> BoundingBox<T>{

        let (min_x, max_x) = get_vec_min_max(&self.v_positions.xs);
        let (min_y, max_y) = get_vec_min_max(&self.v_positions.ys);
        let (min_z, max_z) = get_vec_min_max(&self.v_positions.zs);

        return BoundingBox { min_x, min_y, min_z, max_x, max_y, max_z }
    }
}

fn get_vec_min_max<T: PartialOrd + Copy>(elements: &Vec<T>) -> (T, T) {
    let mut min = elements[0];
    let mut max = elements[0];
    for element in elements{
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
