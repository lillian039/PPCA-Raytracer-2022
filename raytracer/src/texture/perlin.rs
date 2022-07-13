use crate::basic_tools::vec3::Point;
use crate::hittable::hittable_origin::{random_double, random_int};

#[derive(Clone, Default)]
pub struct Perlin {
    pub ranfloat: Vec<f64>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
}
impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = Vec::default();
        for _i in 0..256 {
            randfloat.push(random_double());
        }
        Self {
            ranfloat: (randfloat),
            perm_x: (Perlin::perlin_generate_perm()),
            perm_y: (Perlin::perlin_generate_perm()),
            perm_z: (Perlin::perlin_generate_perm()),
        }
    }
    pub fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::default();
        for i in 0..256 {
            p.push(i);
        }
        for i in (1..256).rev() {
            let target = random_int(0, i as i32);
            p.swap(i, target as usize);
        }
        p
    }

    pub fn noise(&self, p: &Point) -> f64 {
        let i = ((4.0 * p.x).abs() as usize) & 255;
        let j = ((4.0 * p.y).abs() as usize) & 255;
        let k = ((4.0 * p.z).abs() as usize) & 255;
        self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }
}
