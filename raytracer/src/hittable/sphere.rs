use super::super::basic_tools;
use super::hittable_origin::{HitRecord, Hittable};
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(cen: Point, r: f64) -> Self {
        Self {
            center: (cen),
            radius: (r),
        }
    }
}
//whether hit the shpere t is the time
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.point - self.center;
        let a = r.direct.length_squared();
        let half_b = Vec3::dot(&oc, &r.direct);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        true
    }
}
