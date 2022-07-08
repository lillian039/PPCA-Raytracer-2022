use super::{
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Camera {
    pub origin: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(h: f64, w: f64, f: f64) -> Camera {
        let ori = Point::new(0.0, 0.0, 0.0);
        let hori = Vec3::new(w, 0.0, 0.0);
        let vert = Vec3::new(0.0, h, 0.0);
        Camera {
            origin: ori,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: ori - hori / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, f),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
