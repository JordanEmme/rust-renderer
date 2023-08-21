use std::{fs, str};
use super::mesh;

const OBJ_PATH: &str = "assets/input.obj";

pub fn obj_to_mesh() -> mesh::Mesh<f32>{
    let obj_content = fs::read_to_string(OBJ_PATH)
        .expect("Cannot read from the obj file.");
    
    let mut v_positions: mesh::Coords3D<f32> = mesh::Coords3D{xs: Vec::new(), ys: Vec::new(), zs: Vec::new()};
    let mut v_normals: mesh::Coords3D<f32> = mesh::Coords3D{xs: Vec::new(), ys: Vec::new(), zs: Vec::new()};
    let mut v_textures: mesh::Coords2D<f32> = mesh::Coords2D{us: Vec::new(), vs: Vec::new()};
    let mut triangles: Vec<mesh::Triangle> = Vec::new();
    
    for line in obj_content.lines(){
        let mut split: str::SplitWhitespace<'_> = line.split_whitespace();
        let indicator: Option<&str> = split.next();
        match indicator{
            Some("v") => add_3d_coords(&mut v_positions, &mut split),
            Some("vn") => add_3d_coords(&mut v_normals, &mut split),
            Some("vt") => add_2d_coords(&mut v_textures, &mut split),
            Some("f") => add_triangle(&mut triangles, &mut split),
            _ => break,
        } 
    }

    mesh::Mesh{v_positions, v_normals, v_textures, triangles}
}

fn add_3d_coords(to_add_to: &mut mesh::Coords3D<f32>, string_num_iterator: &mut str::SplitWhitespace<'_>){
    for i in 0..3usize {
        let str_num_opt: Option<&str> = string_num_iterator.next();
        match str_num_opt{
            Some(str_num) => {
                let coord: f32 = str_num.parse::<f32>().unwrap();
                match i {
                    0 => to_add_to.xs.push(coord),
                    1 => to_add_to.ys.push(coord),
                    2 => to_add_to.zs.push(coord),
                    _ => {},
                }
            },
            None => {},
        }
    }   
}

fn add_2d_coords(to_add_to: &mut mesh::Coords2D<f32>, string_num_iterator: &mut str::SplitWhitespace<'_>){
    for i in 0..2usize {
        let str_num_opt: Option<&str> = string_num_iterator.next();
        match str_num_opt{
            Some(str_num) => {
                let coord: f32 = str_num.parse::<f32>().unwrap();
                match i {
                    0 => to_add_to.us.push(coord),
                    1 => to_add_to.vs.push(coord),
                    _ => {},
                }
            },
            None => {},
        }
    }   
}

fn add_triangle(triangles: &mut Vec<mesh::Triangle>, string_num_iterator: &mut str::SplitWhitespace<'_>){
    let mut vertices: [usize; 3] = [0usize; 3];    
    let mut normals: [usize; 3] = [0usize; 3];    
    let mut textures: [usize; 3] = [0usize; 3];

    for i in 0..3usize {
        let coords_concat_opt: Option<&str> = string_num_iterator.next();
        match coords_concat_opt {
            Some(coords_concat) => {
                let coords: Vec<&str> = coords_concat.split("/").collect::<Vec<&str>>();
                for j in 0..3usize {
                    let coord: usize = coords[j].parse::<usize>().unwrap();
                    match j {
                        0 => {vertices[i] = coord},
                        1 => {textures[i] = coord},
                        2 => {normals[i] = coord},
                        _ => {},
                    }
                }
            },
            _ => {},
        }
    }
    let tri: mesh::Triangle = mesh::Triangle{vertices, normals, textures};
    triangles.push(tri);
}
