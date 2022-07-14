use super::super::basic_tools;
use super::super::material::metal::Material;
use super::aabb::AABB;
use super::hittable_origin::{HitRecord, Hittable};
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct XYRectangle {
    pub mp: Option<Arc<dyn Material>>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl XYRectangle {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            mp: Some(mat),
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        }
    }
}

impl Hittable for XYRectangle {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point::new(self.x0, self.y0, self.k - 0.0001),
            Point::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.point.z) / r.direct.z;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.point.x + t * r.direct.x;
        let y = r.point.y + t * r.direct.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }
}
