use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Point},
};
use super::super::hittable::hittable_origin::HitRecord;
use crate::material::metal::Material;
use crate::texture::text::{SolidColor, Texture};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct DiffuseLight {
    pub emit: Option<Arc<dyn Texture>>,
}

impl DiffuseLight {
    pub fn new(a: &Arc<dyn Texture>) -> Self {
        Self {
            emit: (Some(a.clone())),
        }
    }

    pub fn new_col(c: Color) -> Self {
        Self {
            emit: Some(Arc::new(SolidColor::new(&c))),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emit(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.emit.as_ref().unwrap().value(_u, _v, _p)
    }
}
