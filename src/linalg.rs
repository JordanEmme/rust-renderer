pub fn cross_product(u: (f32, f32, f32), v: (f32, f32, f32)) -> (f32, f32, f32) {
    return (
        u.1 * v.2 - u.2 * v.1,
        u.2 * v.0 - u.0 * v.2,
        u.0 * v.1 - u.1 * v.0,
        );
}

pub fn dot_product(u: (f32, f32, f32), v: (f32, f32, f32)) -> f32 {
    return u.0 * v.0 + u.1 * v.1 + u.2 * v.2;
}

pub fn norm(v: (f32, f32, f32)) -> f32 {
    return dot_product(v, v);
}

pub fn normalized(v: (f32, f32, f32)) -> (f32, f32, f32) {
    let norm: f32 = norm(v);
    if norm < 0.000001f32 {
        return (0f32, 0f32, 0f32);
    } else {
        return (v.0 / norm, v.1 / norm, v.2 / norm);
    }
}

pub fn get_plane_normal(
    u: (f32, f32, f32),
    v: (f32, f32, f32),
    w: (f32, f32, f32),
) -> (f32, f32, f32) {
    let vec1: (f32, f32, f32) = (v.0 - w.0, v.1 - w.1, v.2 - w.2);
    let vec2: (f32, f32, f32) = (v.0 - u.0, v.1 - u.1, v.2 - u.2);
    return cross_product(vec1, vec2);
}

// Assuming a, b, and c appear in a direct/clockwise order
pub fn triangle_area(a: (u16, u16), b: (u16, u16), c:(u16, u16)) -> f32 {
   let v_ca: (f32, f32, f32) = ((a.0 - c.0) as f32, (a.1 - c.1) as f32, 0f32);
   let v_cb: (f32, f32, f32) = ((b.0 - c.0) as f32, (b.1 - c.1) as f32, 0f32);
   let cross: (f32, f32, f32) = cross_product(v_ca, v_cb);
   return cross.2 / 2f32;
}

pub fn point_barycentric_coord_in_rast_triangle(point: (u16, u16), a: (u16, u16), b: (u16, u16), c: (u16, u16)) -> (f32, f32, f32) {
   let abc_area: f32 = triangle_area(a, b, c);
   let norm_pab_area: f32 = triangle_area(point, a, c) / abc_area;
   let norm_pbc_area: f32 = triangle_area(point, b, c) / abc_area;
   let norm_pca_area: f32 = triangle_area(point, a, c) / abc_area;
   return (norm_pbc_area, norm_pca_area, norm_pab_area);
}

pub fn point_is_in_rast_triangle(point: (u16, u16), a: (u16, u16), b: (u16, u16), c: (u16, u16)) -> bool {
    let p_bary_coords = point_barycentric_coord_in_rast_triangle(point, a, b, c);
    return p_bary_coords.0 >= 0f32 && p_bary_coords.1 >= 0f32 && p_bary_coords.2 >= 0f32;
}
