use super::super::basic_tools;
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direct, outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }

    pub fn new() -> Self {
        Self {
            p: (Point::new(0.0, 0.0, 0.0)),
            normal: (Vec3::new(0.0, 0.0, 0.0)),
            t: (0.0),
            front_face: (true),
        }
    }
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * PI / 180.0
}
