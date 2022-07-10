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
    pub fn new(vfov: f64, aspect_ratio: f64) -> Camera {
        //fov: 视野 vfov:vertical field of view in degrees
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let ori = Point::new(0.0, 0.0, 0.0);
        let hori = Vec3::new(viewport_width, 0.0, 0.0);
        let vert = Vec3::new(0.0, viewport_height, 0.0);
        Camera {
            origin: ori,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: ori - hori / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
