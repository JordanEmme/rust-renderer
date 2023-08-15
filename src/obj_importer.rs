use std::env;
use std::fs;
use std::str;
use super::mesh;

const OBJ_PATH: String = String::from("assets/input.obj");

pub fn obj_to_mesh() -> mesh::Mesh<f32>{
    let obj_content = fs::read_to_string(OBJ_PATH)
        .expect("Cannot read from the obj file.");
    
    let mut v_positions: Vec<mesh::Vector3<f32>> = Vec::new();
    let mut v_uvs: Vec<mesh::Vector2<f32>> = Vec::new();
    let mut v_normals: Vec<mesh::Vector3<f32>> = Vec::new();
    let mut triangles: Vec<mesh::Triangle> = Vec::new();
    
    for line in obj_content.lines(){
        let mut split = line.split_whitespace();
        let indicator = split.next();
        match indicator{
            Some("v") => println!("v"),
            Some("vn") => println!("normal"),
            Some("vt") => println!("uv"),
            Some("f") => println!("face"),
            _ => break,
        } 
    }

    mesh::Mesh{v_positions, v_normals, v_uvs, triangles}
}

fn add_vertex_position(){}

fn add_vertex_texture(){}

fn add_vertex_normal(){}

fn add_triangle(){}
