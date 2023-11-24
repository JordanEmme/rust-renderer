pub fn space_point_in_proj_space(point: (f32, f32, f32), observer_distance: f32) -> (f32, f32) {
    let (x, y, z): (f32, f32, f32) = point;
    let t: f32 = 1. / (1. - (z / observer_distance));
    return (t * x, t * y);
}

