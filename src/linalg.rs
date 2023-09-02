pub fn cross_product(u: (f32, f32, f32), v: (f32, f32, f32)) -> (f32, f32, f32) {
    let mut w: (f32, f32, f32) = (0f32, 0f32, 0f32);
    for i in 0..3 {
        let j = (i + 1).modulo(3);
        let k = (i + 2).modulo(3);
        w.i = u.j * v.k - u.k * v.j;
    }
    return w;
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
