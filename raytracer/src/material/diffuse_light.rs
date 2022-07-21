use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Point},
};
use super::super::hittable::hittable_origin::HitRecord;
use crate::material::metal::{Material, ScatterRecord};
use crate::texture::text::{SolidColor, Texture};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct DiffuseLight {
    pub emit: Option<Arc<dyn Texture>>,
    pub light_intensity: f64,
}

impl DiffuseLight {
    pub fn new(a: Arc<dyn Texture>, intensity: f64) -> Self {
        Self {
            emit: (Some(a)),
            light_intensity: intensity,
        }
    }

    pub fn new_col(c: Color, intensity: f64) -> Self {
        Self {
            emit: Some(Arc::new(SolidColor::new(&c))),
            light_intensity: intensity,
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn emit(&self, u: f64, v: f64, p: &Point, _r_in: &Ray, rec: &HitRecord) -> Color {
        if rec.front_face {
            return self.emit.as_ref().unwrap().value(u, v, p) * self.light_intensity;
        }
        Color::new(0.0, 0.0, 0.0)
    }
}
