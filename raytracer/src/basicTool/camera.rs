use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    viewport_hight: f64,
    viewport_width: f64,
    focal_length: f64,

    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(h: f64, w: f64, f: f64) -> Camera {
        let ori = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let hori = Vec3 {
            x: w,
            y: 0.0,
            z: 0.0,
        };
        let vert = Vec3 {
            x: 0.0,
            y: h,
            z: 0.0,
        };
        Camera {
            viewport_hight: h,
            viewport_width: w,
            focal_length: f,
            origin: ori,
            horizontal: hori,
            vertical: vert,
            lower_left_corner: ori
                - hori / 2.0
                - vert / 2.0
                - Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: f,
                },
        }
    }
}
