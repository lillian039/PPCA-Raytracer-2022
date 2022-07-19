use super::super::material::{
    dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian,
};
use super::aabb::AABB;
use super::{
    super::basic_tools::{
        ray::Ray,
        vec3::{Color, Point, Vec3},
    },
    hittable_origin::random_int,
};
use super::{
    hittable_origin::{HitRecord, Hittable},
    sphere::Sphere,
    xy_rectangle::{Cube, FlipFace, RotateY, Translate, XYRectangle, XZRectangle, YZRectangle},
};

use std::sync::Arc;

#[derive(Clone, Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn new() -> Self {
        Self {
            objects: (Vec::new()),
        }
    }

    pub fn lights() -> HittableList {
        let mut lights = HittableList::default();
        let light = Arc::new(DiffuseLight::new_col(Color::new(15.0, 15.0, 15.0)));
        let lamp = Arc::new(XZRectangle::new(
            213.0,
            343.0,
            227.0,
            332.0,
            554.0,
            light.clone(),
        ));
        lights.add(lamp);
        let ball = Arc::new(Sphere::new(Point::new(190.0, 90.0, 190.0), 90.0, light));
        lights.add(ball);
        lights
    }

    pub fn cornell_box() -> HittableList {
        let mut objects = HittableList::default();

        let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::new_col(Color::new(15.0, 15.0, 15.0)));

        objects.add(Arc::new(YZRectangle::new(
            0.0, 555.0, 0.0, 555.0, 555.0, green,
        )));
        objects.add(Arc::new(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
        let lamp = Arc::new(XZRectangle::new(213.0, 343.0, 227.0, 332.0, 554.0, light));
        objects.add(Arc::new(FlipFace::new(lamp)));
        objects.add(Arc::new(XZRectangle::new(
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
            white.clone(),
        )));
        objects.add(Arc::new(XZRectangle::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));
        objects.add(Arc::new(XYRectangle::new(
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
            white.clone(),
        )));
        let box1 = Arc::new(Cube::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(165.0, 330.0, 165.0),
            white,
        ));
        let box1 = Arc::new(RotateY::new(box1, 15.0));
        let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

        objects.add(box1);

        let glass = Arc::new(Dielectric::new(1.5));
        let ball = Arc::new(Sphere::new(Point::new(190.0, 90.0, 190.0), 90.0, glass));
        objects.add(ball);

        objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::default();
        let mut closest_so_far = t_max;
        let mut hit_anything = false;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                *rec = tmp_rec.clone();
                closest_so_far = tmp_rec.t;
            }
        }
        hit_anything
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut super::aabb::AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut tmp_box = AABB::default();
        let mut first_box = true;

        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut tmp_box) {
                return false;
            }
            *output_box = if first_box {
                tmp_box
            } else {
                AABB::surrounding_box(*output_box, tmp_box)
            };
            first_box = false;
        }
        true
    }

    fn pdf_value(&self, o: &Point, v: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }

        sum
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        self.objects[random_int(0, int_size - 1) as usize].random(o)
    }
}
