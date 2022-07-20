use super::super::basic_tools;
use super::super::material::metal::Material;
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};
use rand::Rng;
#[derive(Clone)]
pub struct HitRecord<M>
where M:Material {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: M,
}

impl <M> HitRecord <M>
where M:Material
{
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direct, outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}
pub trait Hittable <M>
where M:Material{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<M>>;
}

pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_t(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
