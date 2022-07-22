use super::super::basic_tools;
use super::super::material::metal::Material;
use super::aabb::AABB;
use super::hittable_list::HittableList;
use super::hittable_origin::{HitRecord, Hittable};
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};
use std::sync::Arc;
pub struct Triangle<M>
where
    M: Material,
{
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub a1: f64,
    pub b1: f64,
    pub c1: f64,
    pub d1: f64,
    pub mp: M,
    pub minimum: Point,
    pub maximum: Point,
}

fn min_three(x: f64, y: f64, z: f64) -> f64 {
    let mut min_three = f64::min(x, y);
    min_three = min_three.min(z);
    min_three
}

fn max_three(x: f64, y: f64, z: f64) -> f64 {
    let mut max_three = f64::max(x, y);
    max_three = max_three.max(z);
    max_three
}
impl<M: Material> Triangle<M> {
    pub fn new(a: Point, b: Point, c: Point, mat: M) -> Self {
        let fa = (b.y - a.y) * (c.z - a.z) - (c.y - a.y) * (b.z - a.z);
        let fb = (b.z - a.z) * (c.x - a.x) - (c.z - a.z) * (b.x - a.x);
        let fc = (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y);
        let fd = -(fa * a.x + fb * a.y + fc * a.z);
        //    println!("a: {} b:{} c:{} d:{} ", fa, fb, fc, fd);
        let xmin = min_three(a.x, b.x, c.x);
        let ymin = min_three(a.y, b.y, c.y);
        let zmin = min_three(a.z, b.z, c.z);

        let xmax = max_three(a.x, b.x, c.x);
        let ymax = max_three(a.y, b.y, c.y);
        let zmax = max_three(a.z, b.z, c.z);

        Self {
            a: (a),
            b: (b),
            c: (c),
            a1: fa,
            b1: fb,
            c1: fc,
            d1: fd,
            mp: mat,
            minimum: Point::new(xmin, ymin, zmin),
            maximum: Point::new(xmax, ymax, zmax),
        }
    }
}

impl<M: Material> Hittable for Triangle<M> {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.minimum, self.maximum);
        true
    }

    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
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
        rec.mat_ptr = Some(&self.mp);
        true
    }
}

pub struct Object {
    pub surface: HittableList,
}

impl Object {
    pub fn new_hittable(surfaces: &HittableList) -> Self {
        Self {
            surface: (surfaces.clone()),
        }
    }

    pub fn new<M>(filename: &String, mat: M, scale: f64) -> Self
    where
        M: Material + Clone + 'static,
    {
        print!("?");
        let mut points = Vec::default();
        let pathname = String::from("obj/") + filename;
        let cornell_box = tobj::load_obj(
            pathname,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        );
        assert!(cornell_box.is_ok());
        let (models, materials) = cornell_box.expect("Failed to load OBJ file");
        println!("!");

        let mut new_object = HittableList::default();
        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;
            for v in 0..mesh.positions.len() / 3 {
                let x = mesh.positions[3 * v] as f64 * scale;
                let y = mesh.positions[3 * v + 1] as f64 * scale;
                let z = mesh.positions[3 * v + 2] as f64 * scale;
                let p = Point::new(x, y, z);
                points.push(p);
            }
        }
        for i in 0..points.len() / 3 {
            let p1 = points[3 * i];
            let p2 = points[3 * i + 1];
            let p3 = points[3 * i + 2];
            let trian = Triangle::new(p1, p2, p3, mat.clone());
            new_object.add(Arc::new(trian));
        }
        Self {
            surface: (new_object),
        }
    }
}

impl Hittable for Object {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.surface.bounding_box(time0, time1, output_box);
        true
    }

    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
        self.surface.hit(r, t_min, t_max, rec)
    }
}
