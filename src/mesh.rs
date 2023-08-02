use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct Vector3<T: PartialOrd + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector2<T: PartialOrd + Copy> {
    pub u: T,
    pub v: T,
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

pub struct Mesh<T: PartialOrd + Copy> {
    pub v_positions: Vec<Vector3<T>>,
    pub v_normals: Vec<Vector3<T>>,
    pub v_uvs: Vec<Vector2<T>>,
    pub triangles: Vec<Triangle>,
}

impl<T: PartialOrd + Copy> Mesh<T> {

     fn bounding_box(&self) -> (Vector3<T>, Vector3<T>){

        let mut current_vertex = &self.v_positions[0];

        let mut min_x: T = current_vertex.x;
        let mut min_y: T = current_vertex.y;
        let mut min_z: T = current_vertex.z;
        let mut max_x: T = current_vertex.x;
        let mut max_y: T = current_vertex.y;
        let mut max_z: T = current_vertex.z;
        
        for i in 1..self.v_positions.len() {
           current_vertex = &self.v_positions[i];
           let x_min_comp = current_vertex.x.partial_cmp(&min_x);
           if assert_eq!(x_min_comp, Some(Ordering::Less)){
               min_x = current_vertex.x;
           }
           else{
               min_x = min_x;
           }
        }   
        let min: Vector3<T> = Vector3::<T>{ x : min_x, y : min_y, z : min_z };
        let max: Vector3<T> = Vector3::<T>{ x : max_x, y : max_y, z : max_z };
        (min, max)
    }
}
