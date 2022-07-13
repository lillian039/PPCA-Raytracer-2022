use std::vec;

use crate::basic_tools::vec3::{Point, Vec3};
use crate::hittable::hittable_origin::{random_double, random_int};

#[derive(Clone, Default)]
pub struct Perlin {
    // pub ranfloat: Vec<f64>,
    pub ranrec: Vec<Vec3>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>,
}
#[allow(clippy::needless_range_loop)]
impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = Vec::default();
        for _i in 0..256 {
            randfloat.push(Vec3::unit_vector(Vec3::random_range(-1.0, 1.0)));
        }
        Self {
            ranrec: (randfloat),
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
        for i in (1..255).rev() {
            let target = random_int(0, i as i32);
            p.swap(i, target as usize);
        }
        p
    }

    fn set_ranrec(&self, i: i32, j: i32, k: i32) -> Vec3 {
        self.ranrec[(self.perm_x[(i & 255) as usize]
            ^ self.perm_y[(j & 255) as usize]
            ^ self.perm_z[(k & 255) as usize]) as usize]
    }
    pub fn noise(&self, p: &Point) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        u = u * u * (3.0 - 2.0 * u);
        v = v * v * (3.0 - 2.0 * v);
        w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = vec![vec![vec![Vec3::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.set_ranrec(i + di as i32, j + dj as i32, k + dk as i32);
                }
            }
        }

        let mut accum = 0.0;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = di as f64;
                    let j = dj as f64;
                    let k = dk as f64;
                    let weight_v = Vec3::new(u - i, v - j, w - k);
                    accum += (i * u + (1.0 - i) * (1.0 - u))
                        * (j * v + (1.0 - j) * (1.0 - v))
                        * (k * w + (1.0 - k) * (1.0 - w))
                        * Vec3::dot(&c[di][dj][dk], &weight_v);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: &Point, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut wight = 1.0;

        for i in 0..depth {
            accum += wight * self.noise(&temp_p);
            wight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}
