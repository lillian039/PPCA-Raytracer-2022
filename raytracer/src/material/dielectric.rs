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

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &crate::hittable::hittable_origin::HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let mut refraction_ratio = self.ir;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        }
        let unit_direction = Vec3::unit_vector(r_in.direct);
        let refracted = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        *scattered = Ray::new(rec.p, refracted);
        true
    }
}
