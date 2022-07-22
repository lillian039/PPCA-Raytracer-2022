use crate::texture::text::{SolidColor, Texture};

use super::super::basic_tools;
use super::super::material::metal::{Material, ScatterRecord};
use crate::hittable::hittable_origin::HitRecord;
use basic_tools::{ray::Ray, vec3::Color, vec3::Vec3};
#[derive(Clone, Default)]
pub struct Isotropic<T>
where
    T: Texture,
{
    pub albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new_p(a: T) -> Self {
        Self { albedo: (a) }
    }
}

impl Isotropic<SolidColor> {
    pub fn new(c: Color) -> Self {
        Self {
            albedo: (SolidColor::new(&c)),
        }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.specular_ray = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time);
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.is_specular = true;

        true
    }
}
