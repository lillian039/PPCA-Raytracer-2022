use super::super::material::metal::Material;
use super::{super::basic_tools, aabb::AABB};
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};
use rand::Rng;
#[derive(Clone, Default)]
pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: Option<&'a dyn Material>,
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direct, outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool;

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;

    fn pdf_value(&self, _o: &Point, _v: &Vec3) -> f64 {
        1.0
    }

    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_t(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn random_int(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
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
