use crate::hittable::hittable_origin::random_t;

use super::{
    ray::Ray,
    vec3::{Point, Vec3},
};

#[derive(Clone, Copy)]
pub struct Camera {
    pub origin: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * PI / 180.0
}

#[allow(clippy::too_many_arguments)]
impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> Camera {
        //fov: 视野 vfov:vertical field of view in degrees
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w_ = Vec3::unit_vector(lookfrom - lookat);
        let u_ = Vec3::unit_vector(Vec3::cross(vup, w_));
        let v_ = Vec3::cross(w_, u_);
        let ori = lookfrom;
        let hori = u_ * viewport_width * focus_dist;
        let vert = v_ * viewport_height * focus_dist;
        Camera {
            origin: lookfrom,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: ori - hori / 2.0 - vert / 2.0 - w_ * focus_dist,
            u: u_,
            v: v_,
            w: w_,
            lens_radius: aperture / 2.0,
            time0: t0,
            time1: t1,
        }
    }

    pub fn new_random_scence() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let lookfrom = Point::new(13.0, 2.0, 3.0);
        let lookat = Point::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.1;
        Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            aspect_ratio,
            aperture,
            dist_to_focus,
            0.0,
            1.0,
        )
    }

    pub fn new_two_sphere() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let lookfrom = Point::new(13.0, 2.0, 3.0);
        let lookat = Point::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.0;
        Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            aspect_ratio,
            aperture,
            dist_to_focus,
            0.0,
            1.0,
        )
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            random_t(self.time0, self.time1),
        )
    }
}
