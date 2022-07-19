use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Vec3},
};
use super::super::hittable::hittable_origin::random_double;
use super::metal::{Material, ScatterRecord};
pub struct Dielectric {
    pub ir: f64, //index of refrection
}

impl Dielectric {
    pub fn new(index_of_ref: f64) -> Self {
        Self { ir: (index_of_ref) }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
        srec: &mut ScatterRecord,
    ) -> bool {
        srec.is_specular = true;
        srec.pdf_ptr = None;
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(r_in.direct);
        let cos_theta = min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };
        srec.specular_ray = Ray::new(rec.p, direction, r_in.time);
        true
    }
}
