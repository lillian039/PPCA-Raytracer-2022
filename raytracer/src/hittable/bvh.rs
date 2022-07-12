use super::{super::basic_tools, aabb::AABB};
use super::super::material::metal::Material;
use basic_tools::{ray::Ray, vec3::Point, vec3::Vec3};
use rand::Rng;
use std::sync::Arc;

use super::{hittable_origin::{Hittable,HitRecord},hittable_list::HittableList};
#[derive(Clone)]
pub struct BVHNode{
    pub left:Option<Arc<dyn Hittable>>,
    pub right:Option<Arc<dyn Hittable>>,
    pub bounding_box:AABB,
}

/*impl BVHNode {
    pub fn new(list:HittableList,time0:f64,time1:f64)->Self{

    }
    
}*/

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64,rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(r, t_min, t_max){
            return false;
        }
        let hit_left=self.left.as_ref().unwrap().hit(r, t_min, t_max, rec);
        let t_maxr= if hit_left{rec.t} else {t_max};
        let hit_right= self.right.as_ref().unwrap().hit(r, t_min, t_maxr, rec);
        hit_left||hit_right
    }
    fn bounding_box(&self,time0:f64,time1:f64,output_box:&mut AABB)->bool {
        *output_box=self.bounding_box;
        true
    }
    
}