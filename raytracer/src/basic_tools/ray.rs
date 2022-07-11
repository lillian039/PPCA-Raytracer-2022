use super::vec3::Vec3;
pub struct Ray {
    pub point: Vec3,
    pub direct: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(p: Vec3, d: Vec3, t: f64) -> Self {
        Self {
            point: p,
            direct: d,
            time: t,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.point + self.direct * t
    }
}
