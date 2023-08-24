#[derive(Copy, Clone, Debug)]
pub struct BoundingBox {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

impl BoundingBox {
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


