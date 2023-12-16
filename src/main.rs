mod bounding_box;
pub mod drawers;
mod linear_algebra;
pub mod mesh;
pub mod obj_importer;
pub mod tga;

use std::fs::File;
use std::io::BufWriter;

const OBJ_PATH: &str = "assets/input.obj";

fn main() {
    let mesh: mesh::Mesh = obj_importer::obj_to_mesh(OBJ_PATH);
    let mesh_img: tga::Image<tga::Rgb> = drawers::mesh(mesh);
    let output_filename: &str = "output.tga";
    let mut writer: BufWriter<File> = BufWriter::new(File::create(output_filename).unwrap());
    mesh_img.write(&mut writer, true, false).unwrap();
}
