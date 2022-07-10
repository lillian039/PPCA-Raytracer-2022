use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Vec3},
};
use super::metal::Material;

pub struct Dielectric {
    pub ir: f64, //index of refrection
}

impl Dielectric {
    pub fn new(index_of_ref: f64) -> Self {
        Self { ir: (index_of_ref) }
    }
}
pub fn min(ls: f64, rs: f64) -> f64 {
    if ls < rs {
        return ls;
    }
    rs
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &crate::hittable::hittable_origin::HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(r_in.direct);
        let cos_theta = min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };
        *scattered = Ray::new(rec.p, direction);
        true
    }
}
