#[derive(Copy, Clone, Debug)]
pub struct BoundingBox3D {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

impl BoundingBox3D {
    pub fn pad(&mut self, padding_percentage: f32) {
        let total_percentage: f32 = 1f32 + padding_percentage / 100f32;
        self.min_x *= total_percentage;
        self.min_y *= total_percentage;
        self.min_z *= total_percentage;
        self.max_x *= total_percentage;
        self.max_y *= total_percentage;
        self.max_z *= total_percentage;
    }
}


#[derive(Copy, Clone, Debug)]
pub struct BoundingBox2D {
    pub min_u: u16,
    pub min_v: u16,
    pub max_u: u16,
    pub max_v: u16,
}

impl BoundingBox2D{
    pub fn get_bounding_box(triangle_u: &[u16; 3], triangle_v: &[u16; 3]) -> Self {
        let mut min_u = 0u16;
        let mut min_v = 0u16;
        let mut max_u = u16::MAX;
        let mut max_v = u16::MAX;
        for i in 0..3usize {
            let u = triangle_u[i];
            let v = triangle_v[i];
            min_u = min_u.min(u);
            min_v = min_v.min(v);
            max_u = max_u.max(u);
            max_v = max_v.max(v);
        }
        return BoundingBox2D { min_u, min_v, max_u, max_v }
    }
}
