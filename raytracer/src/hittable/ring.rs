use super::super::basic_tools;
use super::super::material::metal::Material;
use super::aabb::AABB;
use super::hittable_origin::{HitRecord, Hittable};
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};

#[derive(Clone, Default)]
pub struct Ring<M>
where
    M: Material,
{
    pub center: Point,
    pub rmin: f64,
    pub rmax: f64,
    pub mat: M,
}

impl<M: Material> Ring<M> {
    pub fn new(cen: Point, rmin: f64, rmax: f64, mat_ptr: M) -> Self {
        Self {
            center: (cen),
            rmin: (rmin),
            rmax: (rmax),
            mat: (mat_ptr),
        }
    }
}

//whether hit the shpere t is the time
impl<M: Material> Hittable for Ring<M> {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
        let t = (self.center.y - r.point.y) / r.direct.y;
        if t < t_min || t_max < t {
            return false;
        }
        let p = r.at(t);
        let condition = ((p.x - self.center.x).powi(2) + (p.z - self.center.z).powi(2)).sqrt();

        if condition < self.rmin || condition > self.rmax {
            return false;
        }

        rec.p = p;
        rec.t = t;
        rec.normal = Vec3::new(0.0, 1.0, 0.0);
        rec.mat_ptr = Some(&self.mat);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.rmax, 0.001, self.rmax),
            self.center + Vec3::new(self.rmax, 0.001, self.rmax),
        );
        true
    }
}
