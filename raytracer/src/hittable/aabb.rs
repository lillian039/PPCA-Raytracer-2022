use super::super::basic_tools;
use basic_tools::{ray::Ray, vec3::Point};

#[derive(Clone,Copy,Default)]
pub struct AABB{
    pub minimum:Point,
    pub maximum:Point,
}

impl AABB {
    pub fn new(a:Point,b:Point)->Self{
        Self { minimum: (a), maximum: (b) }
    }
    
    pub fn hit(&self,r:&Ray,mut t_min:f64,mut t_max:f64)->bool{
        let maxm=[self.maximum.x,self.maximum.y,self.maximum.z];
        let minm=[self.minimum.x,self.minimum.y,self.minimum.z];
        let origin=[r.point.x,r.point.y,r.point.z];
        let direct=[r.direct.x,r.direct.y,r.direct.z];
        for a in 0..3{
            let invd=1.0/direct[a];
            let mut t0=(minm[a]-origin[a])*invd;
            let mut t1=(maxm[a]-origin[a])*invd;

            if invd < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min=f64::max(t0,t_min);
            t_max=f64::min(t1,t_max);

            if t_max <= t_min{
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0:AABB,box1:AABB)->Self{
        let small=Point::new(f64::min(box0.minimum.x,box1.minimum.x), f64::min(box0.minimum.y,box1.minimum.y), f64::min(box0.minimum.z,box1.minimum.z));
        let big=Point::new(f64::max(box0.maximum.x, box1.maximum.x), f64::max(box0.maximum.y, box1.maximum.y), f64::max(box0.maximum.z, box1.maximum.z));
        Self { minimum: (small), maximum: (big) }
    }
}

