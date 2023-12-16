use crate::bounding_box;
use crate::linear_algebra;
use crate::mesh::{Mesh, Point3D, Triangle};
use crate::tga;

const LIGHT_DIRECTION: Point3D<f32> = Point3D {
    x: 0f32,
    y: 0f32,
    z: -1f32,
};
const OBSERVER_DISTANCE: f32 = 1f32;
const FOCAL_LENGTH: f32 = 1f32;
const WIDTH: u16 = 1920;
const HEIGHT: u16 = 1080;
const ZOOM: f32 = 0.15f32;
const WIDTH_RENORMALISATION: f32 = WIDTH as f32 / (ZOOM * 32f32);
const HEIGHT_RENORMALISATION: f32 = HEIGHT as f32 / (ZOOM * 18f32);

pub fn line(x0: u16, y0: u16, x1: u16, y1: u16, img: &mut tga::Image<tga::Rgb>, color: tga::Rgb) {
    let mut start_x: u16 = x0;
    let mut start_y: u16 = y0;
    let mut end_x: u16 = x1;
    let mut end_y: u16 = y1;

    let mut transposed: bool = false;
    if x0.abs_diff(x1) < y0.abs_diff(y1) {
        (start_x, start_y) = (start_y, start_x);
        (end_x, end_y) = (end_y, end_x);
        transposed = true;
    }
    if start_x > end_x {
        (start_x, end_x) = (end_x, start_x);
        (start_y, end_y) = (end_y, start_y);
    }
    let dx: i32 = (end_x - start_x).into();
    let derror2: i32 = (end_y.abs_diff(start_y) * 2).into();
    let mut error2: i32 = 0i32;
    let mut y: u16 = start_y;
    for x in start_x..end_x {
        if transposed {
            let _ = img.set(y, x, color);
        } else {
            let _ = img.set(x, y, color);
        }
        error2 += derror2;
        if error2 > dx {
            if end_y > start_y {
                y += 1;
            } else {
                y -= 1;
            }
            error2 -= dx * 2;
        }
    }
}

pub fn mesh(mesh: Mesh) -> tga::Image<tga::Rgb> {
    let mut mesh_img: tga::Image<tga::Rgb> = tga::Image::new(WIDTH, HEIGHT);

    let mut vertex_buffer_x = [0u16; 3];
    let mut vertex_buffer_y = [0u16; 3];

    let mut z_buffer = [f32::NEG_INFINITY; (WIDTH as u32 * HEIGHT as u32) as usize];

    let triangles = &mesh.triangles;

    triangles.into_iter().for_each(|triangle|
        raster_triangle(
            &triangle,
            &mesh,
            &mut mesh_img,
            &mut vertex_buffer_x,
            &mut vertex_buffer_y,
            &mut z_buffer,
        )
    );
    return mesh_img;
}

fn raster_triangle(
    triangle: &Triangle,
    mesh: &Mesh,
    mesh_img: &mut tga::Image<tga::Rgb>,
    vertex_buffer_x: &mut [u16; 3],
    vertex_buffer_y: &mut [u16; 3],
    z_buffer: &mut [f32],
) {
    for i in 0..3usize {
        let proj = mesh
            .v_positions
            .get_at_orthonormal_projection(triangle.vertices[i]);
        vertex_buffer_x[i] = (WIDTH_RENORMALISATION * (proj.x + ZOOM * 16f32)).floor() as u16;
        vertex_buffer_y[i] = (HEIGHT_RENORMALISATION * (proj.y + ZOOM * 9f32)).floor() as u16;
    }

    let tga_bounding_box: bounding_box::BoundingBox2D =
        bounding_box::BoundingBox2D::get_bounding_box(&vertex_buffer_x, &vertex_buffer_y);
    let triangle_normal = linear_algebra::get_plane_normal(
        mesh.v_positions.get_at(triangle.vertices[0]),
        mesh.v_positions.get_at(triangle.vertices[1]),
        mesh.v_positions.get_at(triangle.vertices[2]),
    );
    let mut intensity: f32 = -linear_algebra::dot_product(triangle_normal, LIGHT_DIRECTION);
    if intensity >= 0f32 {
        for u in tga_bounding_box.min_u..=tga_bounding_box.max_u {
            for v in tga_bounding_box.min_v..=tga_bounding_box.max_v {
                let barycentric_coords = linear_algebra::point_barycentric_coord_in_rast_triangle(
                    (u, v),
                    (vertex_buffer_x[0], vertex_buffer_y[0]),
                    (vertex_buffer_x[1], vertex_buffer_y[1]),
                    (vertex_buffer_x[2], vertex_buffer_y[2]),
                );
                let vertex0_normal = mesh.v_normals.get_at(triangle.normals[0]);
                let vertex1_normal = mesh.v_normals.get_at(triangle.normals[1]);
                let vertex2_normal = mesh.v_normals.get_at(triangle.normals[2]);

                let normal_at_point = Point3D {
                    x: barycentric_coords.x * vertex0_normal.x
                        + barycentric_coords.y * vertex1_normal.x
                        + barycentric_coords.z * vertex2_normal.x,
                    y: barycentric_coords.x * vertex0_normal.y
                        + barycentric_coords.y * vertex1_normal.y
                        + barycentric_coords.z * vertex2_normal.y,
                    z: barycentric_coords.x * vertex0_normal.z
                        + barycentric_coords.y * vertex1_normal.z
                        + barycentric_coords.z * vertex2_normal.z,
                };

                intensity = -linear_algebra::dot_product(normal_at_point, LIGHT_DIRECTION);

                let shade: u8 = (intensity * 255f32).max(0f32) as u8;
                let colour = tga::Rgb {
                    r: shade,
                    g: shade,
                    b: shade,
                };

                let vertex0_z = mesh.v_positions.zs[triangle.vertices[0]];
                let vertex1_z = mesh.v_positions.zs[triangle.vertices[1]];
                let vertex2_z = mesh.v_positions.zs[triangle.vertices[2]];

                let z = barycentric_coords.x * vertex0_z
                    + barycentric_coords.y * vertex1_z
                    + barycentric_coords.z * vertex2_z;
                let z_offset: usize = (v as u32 * WIDTH as u32 + u as u32) as usize;

                if z_buffer[z_offset] < z
                    && linear_algebra::point_is_in_rast_triangle(&barycentric_coords)
                {
                    let _ = mesh_img.set(u, v, colour);
                    z_buffer[z_offset] = z;
                }
            }
        }
    }
}

pub fn wireframe(mesh: Mesh) -> tga::Image<tga::Rgb> {
    const WHITE: tga::Rgb = tga::Rgb {
        r: 255,
        g: 255,
        b: 255,
    };

    let mut bounding_box: bounding_box::BoundingBox3D = mesh.bounding_box();
    bounding_box.pad(10f32);
    let (camera_min, _) = camera_box(&bounding_box);

    let mut mesh_img: tga::Image<tga::Rgb> = tga::Image::new(WIDTH, HEIGHT);

    let mut vertex_buffer_x = [0u16; 3];
    let mut vertex_buffer_y = [0u16; 3];

    let min_x: f32 = camera_min.0;
    let min_y: f32 = camera_min.1;
    mesh.triangles.into_iter().for_each(|triangle| {
        for i in 0..3usize {
            vertex_buffer_x[i] = (WIDTH_RENORMALISATION
                * (mesh.v_positions.xs[triangle.vertices[i]] - min_x))
                .floor() as u16;
            vertex_buffer_y[i] = (HEIGHT_RENORMALISATION
                * (mesh.v_positions.ys[triangle.vertices[i]] - min_y))
                .floor() as u16;
        }
        for i in 0..3usize {
            line(
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
