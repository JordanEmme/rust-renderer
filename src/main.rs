pub mod bounding_box;
pub mod drawers;
pub mod linalg;
pub mod mesh;
pub mod obj_importer;
pub mod tga;

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
    let bounding_box: bounding_box::BoundingBox = mesh.bounding_box();
    // let mut bounding_box: bounding_box::BoundingBox = mesh.bounding_box();
    // bounding_box.pad(10f32);
    let (camera_min, camera_max): ((f32, f32), (f32, f32)) = camera_box(&bounding_box);

    let mesh_img: tga::Image<tga::Rgb> = create_mesh_wireframe(mesh, camera_min, camera_max);

    let output_filename: &str = "output.tga";
    let mut writer: BufWriter<File> = BufWriter::new(File::create(output_filename).unwrap());
    mesh_img.write(&mut writer, true, false).unwrap();
}

fn camera_box(bounding_box: &bounding_box::BoundingBox) -> ((f32, f32), (f32, f32)) {
    let mut width: f32 = bounding_box.max_x - bounding_box.min_x;
    let mut height: f32 = bounding_box.max_y - bounding_box.min_y;
    let ratio: f32 = (WIDTH as f32) / (HEIGHT as f32);
    let mesh_is_wide: bool = width / (height * ratio) > 1f32;

    if mesh_is_wide {
        height = width / ratio;
    } else {
        width = height * ratio;
    }
    let box_centre: (f32, f32) = (
        (bounding_box.max_x + bounding_box.min_x) / 2f32,
        (bounding_box.max_y + bounding_box.min_y) / 2f32,
    );
    let camera_min: (f32, f32) = (box_centre.0 - width / 2f32, box_centre.1 - height / 2f32);
    let camera_max: (f32, f32) = (box_centre.0 + width / 2f32, box_centre.1 + height / 2f32);
    (camera_min, camera_max)
}

fn create_mesh_wireframe(
    mesh: mesh::Mesh,
    camera_min: (f32, f32),
    camera_max: (f32, f32),
) -> tga::Image<tga::Rgb> {
    let mut mesh_img: tga::Image<tga::Rgb> = tga::Image::new(WIDTH, HEIGHT);

    let mut vertex_buffer_x = [0u16; 3];
    let mut vertex_buffer_y = [0u16; 3];

    let min_x: f32 = camera_min.0;
    let min_y: f32 = camera_min.1;
    let box_width: f32 = camera_max.0 - min_x;
    let box_height: f32 = camera_max.1 - min_y;
    let width_renorm: f32 = (WIDTH - 1) as f32 / box_width;
    let height_renorm: f32 = (HEIGHT - 1) as f32 / box_height;
    mesh.triangles.into_iter().for_each(|triangle| {
        for i in 0..3usize {
            vertex_buffer_x[i] =
                (width_renorm * (mesh.v_positions.xs[triangle.vertices[i]] - min_x)).floor() as u16;
            vertex_buffer_y[i] = (height_renorm
                * (mesh.v_positions.ys[triangle.vertices[i]] - min_y))
                .floor() as u16;
        }
        for i in 0..3usize {
            drawers::line(
                vertex_buffer_x[i],
                vertex_buffer_y[i],
                vertex_buffer_x[(i + 1).rem_euclid(3)],
                vertex_buffer_y[(i + 1).rem_euclid(3)],
                &mut mesh_img,
                WHITE,
            );
        }
    });

    return mesh_img;
}
