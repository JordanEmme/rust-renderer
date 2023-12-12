pub mod bounding_box;
pub mod drawers;
pub mod linalg;
pub mod mesh;
pub mod obj_importer;
pub mod tga;

use std::fs::File;
use std::io::BufWriter;

use crate::linalg::point_barycentric_coord_in_rast_triangle;
use bounding_box::BoundingBox2D;

const WHITE: tga::Rgb = tga::Rgb {
    r: 255,
    g: 255,
    b: 255,
};

const LIGHT_DIRECTION: (f32, f32, f32) = (0f32, 0f32, -1f32);
const OBSERVER_DISTANCE: f32 = 1f32;
const FOCAL_LENGTH: f32 = 5f32;
const WIDTH: u16 = 1920;
const HEIGHT: u16 = 1080;

const OBJ_PATH: &str = "assets/input.obj";

fn main() {
    let mesh: mesh::Mesh = obj_importer::obj_to_mesh(OBJ_PATH);
    let bounding_box: bounding_box::BoundingBox3D = mesh.bounding_box();
    // let mut bounding_box: bounding_box::BoundingBox = mesh.bounding_box();
    // bounding_box.pad(10f32);
    let mesh_img: tga::Image<tga::Rgb> = draw_mesh(mesh);

    let output_filename: &str = "output.tga";
    let mut writer: BufWriter<File> = BufWriter::new(File::create(output_filename).unwrap());
    mesh_img.write(&mut writer, true, false).unwrap();
}

fn camera_box(bounding_box: &bounding_box::BoundingBox3D) -> ((f32, f32), (f32, f32)) {
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

fn draw_mesh_wireframe(
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

fn draw_mesh(mesh: mesh::Mesh) -> tga::Image<tga::Rgb> {
    let mut mesh_img: tga::Image<tga::Rgb> = tga::Image::new(WIDTH, HEIGHT);

    let mut vertex_buffer_x = [0u16; 3];
    let mut vertex_buffer_y = [0u16; 3];

    let zoom: f32 = 0.15f32;

    let width_renorm: f32 = WIDTH as f32 / (zoom * 32f32);
    let height_renorm: f32 = HEIGHT as f32 / (zoom * 18f32);

    mesh.triangles.into_iter().for_each(|triangle| {
        for i in 0..3usize {
            let (x, y): (f32, f32) = mesh
                .v_positions
                .get_at_orthonormal_projection(triangle.vertices[i]);
            vertex_buffer_x[i] = (width_renorm * (x + zoom * 16f32)).floor() as u16;
            vertex_buffer_y[i] = (height_renorm * (y + zoom * 9f32)).floor() as u16;
        }

        let tga_bounding_box: BoundingBox2D =
            BoundingBox2D::get_bounding_box(&vertex_buffer_x, &vertex_buffer_y);
        let triangle_normal = linalg::get_plane_normal(
            mesh.v_positions.get_at(triangle.vertices[0]),
            mesh.v_positions.get_at(triangle.vertices[1]),
            mesh.v_positions.get_at(triangle.vertices[2]),
        );
        let mut intensity: f32 = -linalg::dot_product(triangle_normal, LIGHT_DIRECTION);
        if intensity >= 0f32 {
            for u in tga_bounding_box.min_u..=tga_bounding_box.max_u {
                for v in tga_bounding_box.min_v..=tga_bounding_box.max_v {
                    let barycentric_coords = point_barycentric_coord_in_rast_triangle(
                        (u, v),
                        (vertex_buffer_x[0], vertex_buffer_y[0]),
                        (vertex_buffer_x[1], vertex_buffer_y[1]),
                        (vertex_buffer_x[2], vertex_buffer_y[2]),
                    );
                    let vertex0_normal = mesh.v_normals.get_at(triangle.normals[0]);
                    let vertex1_normal = mesh.v_normals.get_at(triangle.normals[1]);
                    let vertex2_normal = mesh.v_normals.get_at(triangle.normals[2]);

                    let normal_at_point: (f32, f32, f32) = (
                        barycentric_coords.0 * vertex0_normal.0
                            + barycentric_coords.1 * vertex1_normal.0
                            + barycentric_coords.2 * vertex2_normal.0,
                        barycentric_coords.0 * vertex0_normal.1
                            + barycentric_coords.1 * vertex1_normal.1
                            + barycentric_coords.2 * vertex2_normal.1,
                        barycentric_coords.0 * vertex0_normal.2
                            + barycentric_coords.1 * vertex1_normal.2
                            + barycentric_coords.2 * vertex2_normal.2,
                    );

                    intensity = -linalg::dot_product(normal_at_point, LIGHT_DIRECTION);

                    let shade: u8 = (intensity * 255f32).max(0f32) as u8;
                    let colour = tga::Rgb {
                        r: shade,
                        g: shade,
                        b: shade,
                    };
                    if linalg::point_is_in_rast_triangle(
                        (u, v),
                        (vertex_buffer_x[0], vertex_buffer_y[0]),
                        (vertex_buffer_x[1], vertex_buffer_y[1]),
                        (vertex_buffer_x[2], vertex_buffer_y[2]),
                    ) {
                        let _ = mesh_img.set(u, v, colour);
                    }
                }
            }
        }
    });
    return mesh_img;
}
