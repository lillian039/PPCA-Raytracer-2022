use super::vec3::Vec3;
pub struct Ray {
    pub point: Vec3,
    pub direct: Vec3,
}

impl Ray {
    pub fn new(p: Vec3, d: Vec3) -> Self {
        Self {
            point: p,
            direct: d,
        }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.point + self.direct * t
    }
}
