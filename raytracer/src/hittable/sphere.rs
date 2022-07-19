use super::super::basic_tools;
use super::super::material::metal::{Material, ONB};
use super::aabb::AABB;
use super::hittable_origin::{HitRecord, Hittable};
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};
use std::f64::consts::PI;
use std::f64::INFINITY;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn new(cen: Point, r: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            center: (cen),
            radius: (r),
            mat_ptr: Some(mat_ptr),
        }
    }
    pub fn get_sphere_uv(p: &Point, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
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
        *rec = HitRecord {
            p: r.at(root),
            normal: Vec3::default(),
            t: root,
            u: 1.0,
            v: 1.0,
            front_face: bool::default(),
            mat_ptr: self.mat_ptr.clone(),
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }

    fn pdf_value(&self, o: &Point, v: &Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(&Ray::new(*o, *v, 0.0), 0.001, INFINITY, &mut rec) {
            return 0.0;
        }

        let cos_thea_max = (1.0 - self.radius.powi(2) / (self.center - *o).length_squared()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_thea_max);
        1.0 / solid_angle
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let direction = self.center - *o;
        let distance_squared = direction.length_squared();
        let mut uvw = ONB::default();
        uvw.build_from_w(direction);
        uvw.local_vec(Vec3::random_to_sphere(self.radius, distance_squared))
    }
}
