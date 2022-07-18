use crate::material::metal::ONB;

use super::super::basic_tools;
use super::hittable_origin::{random_double, Hittable};
use basic_tools::{vec3::Point, vec3::Vec3};
use std::f64::consts::PI;
use std::sync::Arc;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double();
    let r2 = random_double();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    Vec3::new(x, y, z)
}

#[derive(Clone, Copy, Default)]
pub struct CosinePDF {
    pub uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        let mut uvw = ONB::default();
        uvw.build_from_w(w);
        Self { uvw: (uvw) }
    }
}

impl PDF for CosinePDF {
    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(random_cosine_direction())
    }

    fn value(&self, direction: &Vec3) -> f64 {
        let mut cosine = Vec3::dot(&Vec3::unit_vector(*direction), &self.uvw.w());
        cosine = if cosine < 0.0 { 0.0 } else { cosine / PI };
        cosine
    }
}

#[derive(Clone, Default)]
pub struct HittablePDF {
    pub o: Point,
    pub ptr: Option<Arc<dyn Hittable>>,
}

impl HittablePDF {
    pub fn new(p: Arc<dyn Hittable>, origin: Point) -> Self {
        Self {
            o: (origin),
            ptr: (Some(p)),
        }
    }
}

impl PDF for HittablePDF {
    fn generate(&self) -> Vec3 {
        self.ptr.as_ref().unwrap().random(&self.o)
    }

    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.as_ref().unwrap().pdf_value(&self.o, &direction)
    }
}

#[derive(Clone, Default)]
pub struct MixturePDF {
    pub p: [Option<Arc<dyn PDF>>; 2],
}

impl MixturePDF {
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> Self {
        Self {
            p: ([Some(p0), Some(p1)]),
        }
    }
}

impl PDF for MixturePDF {
    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            return self.p[0].as_ref().unwrap().generate();
        }
        self.p[1].as_ref().unwrap().generate()
    }

    fn value(&self, direction: &Vec3) -> f64 {
        self.p[0].as_ref().unwrap().value(direction) * 0.5
            + self.p[1].as_ref().unwrap().value(direction) * 0.5
    }
}
