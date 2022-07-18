use super::super::hittable::hittable_origin::{random_double, random_t};
use std::{
    f64::consts::PI,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub},
};
pub type Color = Vec3;
pub type Point = Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn random() -> Self {
        Self {
            x: (random_double()),
            y: (random_double()),
            z: (random_double()),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            x: (random_t(min, max)),
            y: (random_t(min, max)),
            z: (random_t(min, max)),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut p = Vec3::random_range(-1.0, 1.0);
        while p.length_squared() >= 1.0 {
            p = Vec3::random_range(-1.0, 1.0);
        }
        p
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit_vector(v: Vec3) -> Self {
        v / v.length()
    }

    pub fn dot(ls: &Self, rs: &Self) -> f64 {
        ls.x * rs.x + ls.y * rs.y + ls.z * rs.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Self {
        Self {
            x: (u.y * v.z - u.z * v.y),
            y: (u.z * v.x - u.x * v.z),
            z: (u.x * v.y - u.y * v.x),
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere;
        }
        -in_unit_sphere
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Self {
        v - n * Vec3::dot(&v, &n) * 2.0
    }

    pub fn refract(inray: Vec3, n: Vec3, eta_ratio: f64) -> Self {
        //eta_ratio means η/η'
        let mut cos_theta = Vec3::dot(&-inray, &n);
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        let r_out_perp = (inray + n * cos_theta) * eta_ratio;
        let r_out_parallel = n * (-(1.0 - r_out_perp.length_squared()).abs().sqrt());
        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Self {
        // for defocus blur
        let mut p = Vec3::new(random_t(-1.0, 1.0), random_t(-1.0, 1.0), 0.0);
        while p.length_squared() >= 1.0 {
            p = Vec3::new(random_t(-1.0, 1.0), random_t(-1.0, 1.0), 0.0);
        }
        p
    }

    pub fn random_cosine_direction() -> Vec3 {
        let r1 = random_double();
        let r2 = random_double();
        let z = (1.0 - r2).sqrt();

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();

        //println!("x:{} y:{} z:{} ", x, y, z);

        Vec3::new(x, y, z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: (self.x + other.x),
            y: (self.y + other.y),
            z: (self.z + other.z),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z),
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
