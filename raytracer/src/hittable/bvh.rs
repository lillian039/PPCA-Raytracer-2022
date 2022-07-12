use super::{
    super::basic_tools,
    aabb::{box_compare, AABB},
};
use basic_tools::ray::Ray;
use std::{cmp::Ordering::Less, sync::Arc};

use super::hittable_origin::{random_int, HitRecord, Hittable};
#[derive(Clone)]
pub struct BVHNode {
    pub left: Option<Arc<dyn Hittable>>,
    pub right: Option<Arc<dyn Hittable>>,
    pub bounding_box: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.left.as_ref().unwrap().hit(r, t_min, t_max, rec);
        let t_maxr = if hit_left { rec.t } else { t_max };
        let hit_right = self.right.as_ref().unwrap().hit(r, t_min, t_maxr, rec);
        hit_left || hit_right
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bounding_box;
        true
    }
}

impl BVHNode {
    pub fn new(
        src_objects: Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let mut objects = src_objects.clone();
        let axis = random_int(0, 2);
        let tmp_left;
        let tmp_right;
        let object_span = end - start;
        if object_span == 1 {
            tmp_left = Some(objects[start].clone());
            tmp_right = Some(objects[start].clone());
        } else if object_span == 2 {
            if box_compare(&objects[start], &objects[start + 1], axis) == Less {
                tmp_left = Some(objects[start].clone());
                tmp_right = Some(objects[start + 1].clone());
            } else {
                tmp_left = Some(objects[start + 1].clone());
                tmp_right = Some(objects[start].clone());
            }
        } else {
            objects[start..end].sort_by(|a, b| box_compare(a, b, axis));
            let mid = start + object_span / 2;
            tmp_left = Some(Arc::new(BVHNode::new(
                objects.clone(),
                start,
                mid,
                time0,
                time1,
            )));
            tmp_right = Some(Arc::new(BVHNode::new(objects, mid, end, time0, time1)));
        }
        let mut box_left = AABB::default();
        let mut box_right = AABB::default();
        if !tmp_left
            .as_ref()
            .unwrap()
            .bounding_box(time0, time1, &mut box_left)
            || !tmp_right
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, &mut box_right)
        {
            println!("No bounding box in bvh_node constructor!!!");
        }
        let boxx = AABB::surrounding_box(box_left, box_right);
        Self {
            left: (tmp_left),
            right: (tmp_right),
            bounding_box: (boxx),
        }
    }
}
