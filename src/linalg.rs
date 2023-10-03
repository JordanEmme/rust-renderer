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

pub fn point_is_in_triangle(point: (u16, u16), a: (u16, u16), b: (u16, u16), c: (u16, u16)) -> bool {
    let v_a: (f32, f32, f32) = ((a.0 - point.0) as f32, (a.1 - point.1) as f32, 0f32);
    let v_b: (f32, f32, f32) = ((b.0 - point.0) as f32, (b.1 - point.1) as f32, 0f32);
    let v_c: (f32, f32, f32) = ((c.0 - point.0) as f32, (c.1 - point.1) as f32, 0f32);
    let cross1: (f32, f32, f32) = cross_product(v_a, v_b);
    let cross2: (f32, f32, f32) = cross_product(v_b, v_c);
    return cross1.2 * cross2.2 >= 0f32;
}
