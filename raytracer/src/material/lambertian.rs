use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Vec3},
};
use super::super::hittable::hittable_origin::HitRecord;
use super::metal::Material;
use std::sync::Arc;
pub struct Lambertian {
    pub albedo: Color, //反射率
}

impl Material for Lambertian
 {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &Arc<HitRecord<Material>>,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Lambertian {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            albedo: Color::new(x, y, z),
        }
    }
}
