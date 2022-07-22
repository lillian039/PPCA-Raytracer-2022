use std::{f64::consts::PI, sync::Arc};

use crate::texture::text::{SolidColor, Texture};

use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Vec3},
};
use super::super::hittable::{hittable_origin::HitRecord, pdf::CosinePDF};
use super::metal::{Material, ScatterRecord};

pub struct Lambertian<T>
where
    T: Texture,
{
    pub albedo: T, //反射率
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = false;
        srec.attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(CosinePDF::new(rec.normal)));
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let mut cosine = Vec3::dot(&rec.normal, &Vec3::unit_vector(scattered.direct));
        cosine = if cosine < 0.0 { 0.0 } else { cosine / PI };
        cosine
    }
}

impl Lambertian<SolidColor> {
    pub fn new(a: Color) -> Self {
        Self {
            albedo: SolidColor::new(&a),
        }
    }
}
impl<T: Texture> Lambertian<T> {
    pub fn newp(a: T) -> Self {
        Self { albedo: (a) }
    }
}
