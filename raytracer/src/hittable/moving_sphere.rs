use super::super::basic_tools;
use super::super::material::metal::Material;
use super::{hittable_origin::{HitRecord, Hittable},aabb::AABB};
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};
use std::sync::Arc;
#[derive(Clone,Default)]
pub struct MovingSphere {
    pub center0: Point,
    pub center1: Point,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn Material>>,
}

impl MovingSphere {
    pub fn new(cen0: Point, cen1: Point, t0: f64, t1: f64, r: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center0: (cen0),
            center1: (cen1),
            time0: (t0),
            time1: (t1),
            radius: (r),
            mat_ptr: (Some(mat)),
        }
    }

    pub fn center(&self, time: f64) -> Point {
        self.center0
            + (self.center1 - self.center0) * (time - self.time0) / (self.time1 - self.time0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64,rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.point - self.center(r.time);
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
            front_face: bool::default(),
            mat_ptr: self.mat_ptr.clone(),
        };
        let outward_normal = (rec.p - self.center(r.time)) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        true
    }

    fn bounding_box(&self,time0:f64,time1:f64,output_box:&mut super::aabb::AABB)->bool {
        let box0=AABB::new(self.center(time0)-Vec3::new(self.radius, self.radius, self.radius),self.center(time0)+Vec3::new(self.radius, self.radius, self.radius));
        let box1=AABB::new(self.center(time1)-Vec3::new(self.radius, self.radius, self.radius),self.center(time1)+Vec3::new(self.radius, self.radius, self.radius));
        *output_box=AABB::surrounding_box(box0, box1);
        true
    }
}
