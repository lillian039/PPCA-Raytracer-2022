use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Point},
};
use super::super::hittable::hittable_origin::HitRecord;
use crate::material::metal::{Material, ScatterRecord};
use crate::texture::text::{SolidColor, Texture};

#[derive(Clone, Default)]
pub struct DiffuseLight<T>
where
    T: Texture,
{
    pub emit: T,
    pub light_intensity: f64,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(a: T, intensity: f64) -> Self {
        Self {
            emit: a,
            light_intensity: intensity,
        }
    }
}
impl DiffuseLight<SolidColor> {
    pub fn new_col(c: Color, intensity: f64) -> Self {
        Self {
            emit: SolidColor::new(&c),
            light_intensity: intensity,
        }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn emit(&self, u: f64, v: f64, p: &Point, _r_in: &Ray, rec: &HitRecord) -> Color {
        if rec.front_face {
            return self.emit.value(u, v, p) * self.light_intensity;
        }
        Color::new(0.0, 0.0, 0.0)
    }
}
