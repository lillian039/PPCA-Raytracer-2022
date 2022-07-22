use crate::texture::text::{SolidColor, Texture};

use super::super::basic_tools;
use super::super::material::isotropic::Isotropic;
use super::aabb::AABB;
use super::hittable_origin::{random_double, HitRecord, Hittable};
use basic_tools::{ray::Ray, vec3::Color, vec3::Vec3};
use std::f64::INFINITY;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct ConstantMedium<T>
where
    T: Texture,
{
    pub boundary: Option<Arc<dyn Hittable>>,
    pub phase_function: Isotropic<T>,
    pub neg_inv_density: f64,
}

impl<T: Texture> ConstantMedium<T> {
    pub fn new(b: Arc<dyn Hittable>, d: f64, a: T) -> Self {
        Self {
            boundary: (Some(b)),
            phase_function: Isotropic::new_p(a),
            neg_inv_density: (-1.0 / d),
        }
    }
}

impl ConstantMedium<SolidColor> {
    pub fn new_col(b: Arc<dyn Hittable>, d: f64, c: &Color) -> Self {
        Self {
            boundary: (Some(b)),
            phase_function: Isotropic::new(*c),
            neg_inv_density: (-1.0 / d),
        }
    }
}

impl<T: Texture> Hittable for ConstantMedium<T> {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary
            .as_ref()
            .unwrap()
            .bounding_box(time0, time1, output_box)
    }

    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self
            .boundary
            .as_ref()
            .unwrap()
            .hit(r, -INFINITY, INFINITY, &mut rec1)
        {
            return false;
        }
        if !self
            .boundary
            .as_ref()
            .unwrap()
            .hit(r, rec1.t + 0.0001, INFINITY, &mut rec2)
        {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direct.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().log2();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3::new(0.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat_ptr = Some(&self.phase_function);

        true
    }
}
