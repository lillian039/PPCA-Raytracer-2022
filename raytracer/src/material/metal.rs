use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Vec3},
};
use super::super::hittable::hittable_origin::HitRecord;
use std::sync::Arc;
pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: Arc<HitRecord<dyn Material>>,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Metal {
    pub albebo: Color,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord<Arc<dyn Material>>,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direct), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albebo;
        Vec3::dot(&scattered.direct, &rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            albebo: (Color::new(x, y, z)),
        }
    }
}
