use super::{
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Camera {
    pub origin: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * PI / 180.0
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        //fov: 视野 vfov:vertical field of view in degrees
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        let ori = lookfrom;
        let hori = u * viewport_width;
        let vert = v * viewport_height;
        Camera {
            origin: lookfrom,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: ori - hori / 2.0 - vert / 2.0 - w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin,
        )
    }
}
