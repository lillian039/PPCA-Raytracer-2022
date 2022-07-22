use crate::basic_tools::camera::degrees_to_radians;

use super::super::basic_tools;
use super::super::material::metal::Material;
use super::aabb::AABB;
use super::hittable_list::HittableList;
use super::hittable_origin::{random_t, HitRecord, Hittable};
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};
use std::f64::INFINITY;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct XYRectangle<M>
where
    M: Material,
{
    pub mp: M,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl<M: Material> XYRectangle<M> {
    pub fn new(_x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64, mat: M) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        }
    }
}

impl<M: Material> Hittable for XYRectangle<M> {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point::new(self.x0, self.y0, self.k - 0.0001),
            Point::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }

    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
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
        rec.mat_ptr = Some(&self.mp);
        rec.p = r.at(t);
        true
    }
}

#[derive(Clone, Default)]
pub struct XZRectangle<M>
where
    M: Material,
{
    pub mp: M,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}
impl<M: Material> XZRectangle<M> {
    pub fn new(_x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64, mat: M) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }
}

impl<M: Material> Hittable for XZRectangle<M> {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point::new(self.x0, self.k - 0.0001, self.z0),
            Point::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }

    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
        let t = (self.k - r.point.y) / r.direct.y;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.point.x + t * r.direct.x;
        let z = r.point.z + t * r.direct.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Some(&self.mp);
        rec.p = r.at(t);
        true
    }

    fn pdf_value(&self, o: &Point, v: &Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(&Ray::new(*o, *v, 0.0), 0.001, INFINITY, &mut rec) {
            return 0.0;
        }

        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squard = rec.t * rec.t * v.length_squared();
        let cosine = (Vec3::dot(v, &rec.normal) / v.length()).abs();

        distance_squard / (cosine * area)
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let random_point = Point::new(
            random_t(self.x0, self.x1),
            self.k,
            random_t(self.z0, self.z1),
        );
        random_point - *o
    }
}

#[derive(Clone, Default)]
pub struct YZRectangle<M>
where
    M: Material,
{
    pub mp: M,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}
impl<M: Material> YZRectangle<M> {
    pub fn new(_y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64, mat: M) -> Self {
        Self {
            mp: mat,
            y0: _y0,
            y1: _y1,
            z0: _z0,
            z1: _z1,
            k: _k,
        }
    }
}
impl<M: Material> Hittable for YZRectangle<M> {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point::new(self.k - 0.0001, self.y0, self.z0),
            Point::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }

    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
        let t = (self.k - r.point.x) / r.direct.x;
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.point.y + t * r.direct.y;
        let z = r.point.z + t * r.direct.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Some(&self.mp);
        rec.p = r.at(t);
        true
    }
}

#[derive(Clone, Default)]
pub struct Cube {
    pub box_min: Point,
    pub box_max: Point,
    pub slides: HittableList,
}

impl Cube {
    pub fn new<M>(p0: Point, p1: Point, ptr: M) -> Self
    where
        M: Material + Clone + 'static,
    {
        let mut slide = HittableList::default();
        slide.add(Arc::new(XYRectangle::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        slide.add(Arc::new(XYRectangle::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));
        slide.add(Arc::new(XZRectangle::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        slide.add(Arc::new(XZRectangle::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));
        slide.add(Arc::new(YZRectangle::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        slide.add(Arc::new(YZRectangle::new(
            p0.y, p1.y, p0.z, p1.z, p0.x, ptr,
        )));
        Self {
            box_min: (p0),
            box_max: (p1),
            slides: (slide),
        }
    }
}

impl Hittable for Cube {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.box_min, self.box_max);
        true
    }

    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
        self.slides.hit(r, t_min, t_max, rec)
    }
}

#[derive(Clone, Default)]
pub struct Translate {
    pub ptr: Option<Arc<dyn Hittable>>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            ptr: (Some(p)),
            offset: (displacement),
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if !self
            .ptr
            .as_ref()
            .unwrap()
            .bounding_box(time0, time1, output_box)
        {
            return false;
        }
        *output_box = AABB::new(
            output_box.minimum + self.offset,
            output_box.maximum + self.offset,
        );
        true
    }

    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
        let moved_r = Ray::new(r.point - self.offset, r.direct, r.time);
        if !self.ptr.as_ref().unwrap().hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        rec.set_face_normal(&moved_r, &rec.normal.clone());
        true
    }
}

#[derive(Clone, Default)]
pub struct RotateY {
    pub ptr: Option<Arc<dyn Hittable>>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub has_box: bool,
    pub bbox: AABB,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let radius = degrees_to_radians(angle);
        let mut bbox = AABB::default();
        let hasbox = p.bounding_box(0.0, 1.0, &mut bbox);
        let mut min = Point::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point::new(-INFINITY, -INFINITY, -INFINITY);
        let cos_theta = radius.cos();
        let sin_theta = radius.sin();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.maximum.x + (1 - i) as f64 * bbox.minimum.x;
                    let y = j as f64 * bbox.maximum.y + (1 - j) as f64 * bbox.minimum.y;
                    let z = k as f64 * bbox.maximum.z + (1 - k) as f64 * bbox.minimum.z;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min.x = f64::min(min.x, tester.x);
                    max.x = f64::max(max.x, tester.x);

                    min.y = f64::min(min.y, tester.y);
                    max.y = f64::max(max.y, tester.y);

                    min.z = f64::min(min.z, tester.z);
                    max.z = f64::max(max.z, tester.z);
                }
            }
        }

        Self {
            ptr: (Some(p)),
            sin_theta: (sin_theta),
            cos_theta: (cos_theta),
            has_box: (hasbox),
            bbox: (AABB::new(min, max)),
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.has_box
    }

    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
        let mut origin = r.point;
        let mut direction = r.direct;
        origin.x = self.cos_theta * r.point.x - self.sin_theta * r.point.z;
        origin.z = self.sin_theta * r.point.x + self.cos_theta * r.point.z;

        direction.x = self.cos_theta * r.direct.x - self.sin_theta * r.direct.z;
        direction.z = self.sin_theta * r.direct.x + self.cos_theta * r.direct.z;

        let rotated_r = Ray::new(origin, direction, r.time);

        if !self
            .ptr
            .as_ref()
            .unwrap()
            .hit(&rotated_r, t_min, t_max, rec)
        {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

        normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }
}

pub struct FlipFace {
    pub ptr: Option<Arc<dyn Hittable>>,
}

impl FlipFace {
    pub fn new(p: Arc<dyn Hittable>) -> Self {
        Self { ptr: Some(p) }
    }
}
impl Hittable for FlipFace {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.ptr
            .as_ref()
            .unwrap()
            .bounding_box(time0, time1, output_box)
    }

    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
        if !self.ptr.as_ref().unwrap().hit(r, t_min, t_max, rec) {
            return false;
        }
        rec.front_face = !rec.front_face;
        true
    }
}
