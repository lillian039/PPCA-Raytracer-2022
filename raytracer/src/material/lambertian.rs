use std::{f64::consts::PI, sync::Arc};

use crate::texture::text::{SolidColor, Texture};

use super::super::hittable::hittable_origin::HitRecord;
use super::metal::Material;
use super::{
    super::basic_tools::{
        ray::Ray,
        vec3::{Color, Vec3},
    },
    metal::ONB,
};

pub struct Lambertian {
    pub albedo: Option<Arc<dyn Texture>>, //反射率
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        alb: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        let mut uvw = ONB::default();
        uvw.build_from_w(rec.normal);

        let direction = uvw.local_vec(Vec3::random_cosine_direction());
        *scattered = Ray::new(rec.p, Vec3::unit_vector(direction), r_in.time);
        *alb = self.albedo.as_ref().unwrap().value(rec.u, rec.v, &rec.p);
        *pdf = Vec3::dot(&uvw.w(), &scattered.direct) / PI;
        true
        //====sphere===
        /*let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, Vec3::unit_vector(scatter_direction), r_in.time);
        *alb = self.albedo.as_ref().unwrap().value(rec.u, rec.v, &rec.p);
        *pdf = Vec3::dot(&rec.normal, &scattered.direct) / PI;
        true*/

        //====hemisphere====
        /* let direction = Vec3::random_in_hemisphere(&rec.normal);
        *scattered = Ray::new(rec.p, Vec3::unit_vector(direction), r_in.time);
        *alb = self.albedo.as_ref().unwrap().value(rec.u, rec.v, &rec.p);
        *pdf = 0.5 / PI;
        true */
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let mut cosine = Vec3::dot(&rec.normal, &Vec3::unit_vector(scattered.direct));
        cosine = if cosine < 0.0 { 0.0 } else { cosine / PI };
        cosine
    }
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self {
            albedo: Some(Arc::new(SolidColor::new(&a))),
        }
    }
    pub fn newp(a: Arc<dyn Texture>) -> Self {
        Self { albedo: (Some(a)) }
    }
}
