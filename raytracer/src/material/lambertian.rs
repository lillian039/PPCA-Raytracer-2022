use std::sync::Arc;

use crate::texture::text::{SolidColor, Texture};

use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Vec3},
};
use super::super::hittable::hittable_origin::HitRecord;
use super::metal::Material;

pub struct Lambertian {
    pub albedo: Option<Arc<dyn Texture>>, //反射率
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, r_in.time);
        *attenuation = self.albedo.as_ref().unwrap().value(rec.u, rec.v, &rec.p);
        true
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
