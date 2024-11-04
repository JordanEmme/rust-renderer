use crate::mesh::Point3D;

pub fn cross_product(u: Point3D<f32>, v: Point3D<f32>) -> Point3D<f32> {
    return Point3D {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    };
}

pub fn dot_product(u: Point3D<f32>, v: Point3D<f32>) -> f32 {
    return u.x * v.x + u.y * v.y + u.z * v.z;
}

pub fn norm(v: Point3D<f32>) -> f32 {
    return dot_product(v, v).sqrt();
}

pub fn normalized(v: Point3D<f32>) -> Point3D<f32> {
    let norm: f32 = norm(v);
    return if norm < 0.000001f32 {
        Point3D {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    } else {
        Point3D {
            x: v.x / norm,
            y: v.y / norm,
            z: v.z / norm,
        }
    };
}

pub fn get_plane_normal(u: Point3D<f32>, v: Point3D<f32>, w: Point3D<f32>) -> Point3D<f32> {
    let vec1 = Point3D {
        x: v.x - w.x,
        y: v.y - w.y,
        z: v.z - w.z,
    };
    let vec2 = Point3D {
        x: v.x - u.x,
        y: v.y - u.y,
        z: v.z - u.z,
    };
    return normalized(cross_product(vec1, vec2));
}

// Assuming a, b, and c appear in a direct/clockwise order
pub fn triangle_area(a: (u16, u16), b: (u16, u16), c: (u16, u16)) -> f32 {
    let v_ca = Point3D {
        x: a.0 as f32 - c.0 as f32,
        y: a.1 as f32 - c.1 as f32,
        z: 0f32,
    };
    let v_cb = Point3D {
        x: b.0 as f32 - c.0 as f32,
        y: b.1 as f32 - c.1 as f32,
        z: 0f32,
    };
    let cross = cross_product(v_ca, v_cb);
    return cross.z / 2f32;
}

pub fn point_barycentric_coord_in_rast_triangle(
    point: (u16, u16),
    a: (u16, u16),
    b: (u16, u16),
    c: (u16, u16),
) -> Point3D<f32> {
    let abc_area: f32 = triangle_area(a, b, c);
    let norm_pab_area: f32 = triangle_area(point, a, b) / abc_area;
    let norm_pbc_area: f32 = triangle_area(point, b, c) / abc_area;
    let norm_pca_area: f32 = triangle_area(point, c, a) / abc_area;
    return Point3D {
        x: norm_pbc_area,
        y: norm_pca_area,
        z: norm_pab_area,
    };
}

pub fn point_is_in_rast_triangle(barycentric_coordinates: &Point3D<f32>) -> bool {
    return barycentric_coordinates.x >= 0f32
        && barycentric_coordinates.y >= 0f32
        && barycentric_coordinates.z >= 0f32;
}

pub fn barycentric_interpolation(
    barycentric_coords: &Point3D<f32>,
    vertex0_normal: &Point3D<f32>,
    vertex1_normal: &Point3D<f32>,
    vertex2_normal: &Point3D<f32>,
) -> Point3D<f32> {
    return Point3D {
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
}
