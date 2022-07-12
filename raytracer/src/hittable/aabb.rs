use super::{super::basic_tools, hittable_origin::Hittable};
use basic_tools::{ray::Ray, vec3::Point};
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Clone, Copy, Default)]
pub struct AABB {
    pub minimum: Point,
    pub maximum: Point,
}

impl AABB {
    pub fn new(a: Point, b: Point) -> Self {
        Self {
            minimum: (a),
            maximum: (b),
        }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let maxm = [self.maximum.x, self.maximum.y, self.maximum.z];
        let minm = [self.minimum.x, self.minimum.y, self.minimum.z];
        let origin = [r.point.x, r.point.y, r.point.z];
        let direct = [r.direct.x, r.direct.y, r.direct.z];
        for a in 0..3 {
            let invd = 1.0 / direct[a];
            let mut t0 = (minm[a] - origin[a]) * invd;
            let mut t1 = (maxm[a] - origin[a]) * invd;

            if invd < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> Self {
        let small = Point::new(
            f64::min(box0.minimum.x, box1.minimum.x),
            f64::min(box0.minimum.y, box1.minimum.y),
            f64::min(box0.minimum.z, box1.minimum.z),
        );
        let big = Point::new(
            f64::max(box0.maximum.x, box1.maximum.x),
            f64::max(box0.maximum.y, box1.maximum.y),
            f64::max(box0.maximum.z, box1.maximum.z),
        );
        Self {
            minimum: (small),
            maximum: (big),
        }
    }
}
pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: i32) -> Ordering {
    let mut box_a = AABB::default();
    let mut box_b = AABB::default();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        println!("No bounding box in bvh_node constructor!!!!!");
    }
    if axis == 0 {
        if box_a.minimum.x < box_b.minimum.x {
            return Ordering::Less;
        } else if box_a.minimum.x == box_b.minimum.x {
            return Ordering::Equal;
        }
    } else if axis == 1 {
        if box_a.minimum.y < box_b.minimum.y {
            return Ordering::Less;
        } else if box_a.minimum.y == box_b.minimum.y {
            return Ordering::Equal;
        }
    } else if axis == 2 {
        if box_a.minimum.z < box_b.minimum.z {
            return Ordering::Less;
        } else if box_a.minimum.z == box_b.minimum.z {
            return Ordering::Equal;
        }
    }
    Ordering::Greater
}
