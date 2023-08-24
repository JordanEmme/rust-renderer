pub mod line_drawer;
pub mod mesh;
pub mod obj_importer;
pub mod tga;
pub mod bounding_box;

use std::fs::File;
use std::io::BufWriter;

const WHITE: tga::Rgb = tga::Rgb {
    r: 255,
    g: 255,
    b: 255,
};

const WIDTH: u16 = 1920;
const HEIGHT: u16 = 1080;

const OBJ_PATH: &str = "assets/input.obj";

fn main() {
    let mesh: mesh::Mesh = obj_importer::obj_to_mesh(OBJ_PATH);
    let mut bounding_box: bounding_box::BoundingBox = mesh.bounding_box();
    bounding_box.pad(10f32);
    let (width, height): (f32, f32) = camera_box(&bounding_box);

    let mut img: tga::Image<tga::Rgb> = tga::Image::<tga::Rgb>::new(WIDTH, HEIGHT);
    line_drawer::line(0, 0, 9, 9, &mut img, WHITE);

    //    let mut img = tga::Image::<tga::Rgb>::new(10, 10);
    //    let _ = img.set(0, 0, RED);

    let output_filename: &str = "output.tga";
    let mut writer: BufWriter<File> = BufWriter::new(File::create(output_filename).unwrap());
    img.write(&mut writer, true, false).unwrap();
}

fn camera_box(bounding_box: &bounding_box::BoundingBox) -> (f32, f32) {
    let mut width: f32 = bounding_box.max_x - bounding_box.min_x;
    let mut height: f32 = bounding_box.max_x - bounding_box.min_x;
    let ratio: f32 = (WIDTH as f32) / (HEIGHT as f32);
    let mesh_is_wide: bool = width / (height * ratio) > 1f32;

    if mesh_is_wide {
        height = width / ratio;
    } else {
        width = height * ratio;
    }

    // add some padding for aesthetics and avoid overflowing the image with
    // integer coordinates
    width *= 1.05f32;
    height *= 1.05f32;

    (width, height)
}
