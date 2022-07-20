use crate::material::metal::Material;

use super::super::basic_tools::ray::Ray;
use super::hittable_origin::{HitRecord, Hittable};
use std::sync::Arc;
#[derive(Clone, Default)]
pub struct HittableList<M:Material> 
where M:Material{
    pub objects: Vec<Arc<dyn Hittable<M>>>,
}

impl <M:Material> HittableList <M>{
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: dyn Hittable<M>) {
        self.objects.push(object);
    }

    pub fn new() -> Self {
        Self {
            objects: (Vec::new()),
        }
    }
}

impl <M> Hittable<M> for HittableList<M>
where M:Material {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<M>> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
                // rec.clone_from(&temp_rec);
            }
        }
        rec
    }
}
