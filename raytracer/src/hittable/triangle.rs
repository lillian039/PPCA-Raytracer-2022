use super::super::basic_tools;
use super::super::material::metal::Material;
use super::aabb::AABB;
use super::hittable_origin::{HitRecord, Hittable};
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};
use std::sync::Arc;

pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub a1: f64,
    pub b1: f64,
    pub c1: f64,
    pub d1: f64,
    pub mp: Option<Arc<dyn Material>>,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point, mat: Arc<dyn Material>) -> Self {
        let fa = (b.y - a.y) * (c.z - a.z) - (c.y - a.y) * (b.z - a.z);
        let fb = (b.z - a.z) * (c.x - a.x) - (c.z - a.z) * (b.x - a.x);
        let fc = (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y);
        let fd = -(fa * a.x + fb * a.y + fc * a.z);
        //    println!("a: {} b:{} c:{} d:{} ", fa, fb, fc, fd);

        Self {
            a: (a),
            b: (b),
            c: (c),
            a1: fa,
            b1: fb,
            c1: fc,
            d1: fd,
            mp: (Some(mat)),
        }
    }
}

impl Hittable for Triangle {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        let mut xmin = f64::min(self.a.x, self.b.x);
        xmin = xmin.min(self.c.x);
        let mut ymin = f64::min(self.a.y, self.b.y);
        ymin = ymin.min(self.c.y);
        let mut zmin = f64::min(self.a.z, self.b.z);
        zmin = zmin.min(self.c.z);

        let mut xmax = f64::max(self.a.x, self.b.x);
        xmax = xmax.max(self.c.x);
        let mut ymax = f64::max(self.a.y, self.b.y);
        ymax = ymax.max(self.c.y);
        let mut zmax = f64::max(self.a.z, self.b.z);
        zmax = zmax.max(self.c.z);
        *output_box = AABB::new(Point::new(xmin, ymin, zmin), Point::new(xmax, ymax, zmax));
        true
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let k_right = -self.d1 - (self.a1 * r.point.x + self.b1 * r.point.y + self.c1 * r.point.z);
        let k_left = self.a1 * r.direct.x + self.b1 * r.direct.y + self.c1 * r.direct.z;
        let t = k_right / k_left;

        if t < t_min || t > t_max || t.is_nan() {
            return false;
        }

        let p = r.at(t);
        let pa = self.a - p;
        let pb = self.b - p;
        let pc = self.c - p;

        let dot1 = Vec3::dot(&Vec3::cross(pa, pb), &Vec3::cross(pb, pc));
        let dot2 = Vec3::dot(&Vec3::cross(pa, pb), &Vec3::cross(pc, pa));
        if dot1 < 0.00001 || dot2 < 0.00001 {
            return false;
        }

        let outward_normal = Vec3::unit_vector(Vec3::cross(self.a - self.b, self.a - self.c));
        rec.set_face_normal(r, &outward_normal);
        let a1 = self.a.x - self.b.x;
        let b1 = self.a.x - self.c.x;
        let c1 = self.a.x - p.x;
        let a2 = self.a.y - self.b.y;
        let b2 = self.a.y - self.c.y;
        let c2 = self.a.y - p.y;
        rec.u = (c1 * b2 - b1 * c2) / (a1 * b2 - b1 * a2);
        rec.v = (a1 * c2 - a2 * c1) / (a1 * b2 - b1 * a2);
        rec.t = t;
        rec.p = r.at(t);
        rec.mat_ptr = self.mp.clone();
        true
    }
}
