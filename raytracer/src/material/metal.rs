use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Point, Vec3},
};
use super::super::hittable::{hittable_origin::HitRecord, pdf::PDF};
use std::sync::Arc;
#[derive(Default)]
pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn PDF>>,
}
pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _src: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }

    fn emit(&self, _u: f64, _v: f64, _p: &Point, _r_in: &Ray, _rec: &HitRecord) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

#[derive(Clone, Copy, Default)]
pub struct ONB {
    pub axis: [Vec3; 3],
}

impl ONB {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }

    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u() * a + self.v() * b + self.w() * c
    }

    pub fn local_vec(&self, a: Vec3) -> Vec3 {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }

    pub fn build_from_w(&mut self, n: Vec3) {
        self.axis[2] = Vec3::unit_vector(n);
        let a = if (self.w().x).abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        self.axis[1] = Vec3::unit_vector(Vec3::cross(self.w(), a));
        self.axis[0] = Vec3::cross(self.w(), self.v());
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albebo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direct), rec.normal);
        srec.specular_ray = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            0.0,
        );
        srec.attenuation = self.albebo;
        srec.is_specular = true;
        srec.pdf_ptr = None;
        true
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
