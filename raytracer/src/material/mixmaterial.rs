use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Point},
};
use super::super::hittable::hittable_origin::{random_double, HitRecord};
use super::metal::{Material, ScatterRecord};

#[derive(Clone)]
pub struct MixtureMaterial<M, N>
where
    M: Material,
    N: Material,
{
    pub m1: M,
    pub m2: N,
    pub rate: f64,
}
impl<M: Material, N: Material> MixtureMaterial<M, N> {
    pub fn new(mat1: M, mat2: N, r: f64) -> Self {
        Self {
            m1: (mat1),
            m2: (mat2),
            rate: r,
        }
    }
}
impl<M: Material, N: Material> Material for MixtureMaterial<M, N> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let c = random_double();
        if c < self.rate {
            return self.m1.scatter(r_in, rec, srec);
        }
        self.m2.scatter(r_in, rec, srec)
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let c = random_double();
        if c < self.rate {
            return self.m1.scattering_pdf(r_in, rec, scattered);
        }
        self.m2.scattering_pdf(r_in, rec, scattered)
    }

    fn emit(&self, u: f64, v: f64, p: &Point, r_in: &Ray, rec: &HitRecord) -> Color {
        let c = random_double();
        if c < self.rate {
            return self.m1.emit(u, v, p, r_in, rec);
        }
        self.m2.emit(u, v, p, r_in, rec)
    }
}
