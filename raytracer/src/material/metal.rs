use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Vec3},
};
use super::super::hittable::hittable_origin::HitRecord;
pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Metal {
    pub albebo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direct), rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albebo;
        Vec3::dot(&scattered.direct, &rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(al: Color, fuz: f64) -> Self {
        let mut fuzzz = 1.0;
        if fuz < fuzzz {
            fuzzz = fuz;
        }
        Self {
            albebo: al,
            fuzz: fuzzz,
        }
    }
}
