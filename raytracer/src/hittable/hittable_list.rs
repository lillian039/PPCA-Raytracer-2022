use super::super::material::{
    dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
};
use super::aabb::AABB;
use super::{
    super::basic_tools::{
        ray::Ray,
        vec3::{Color, Point, Vec3},
    },
    hittable_origin::random_t,
};
use super::{
    hittable_origin::{random_double, HitRecord, Hittable},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    xy_rectangle::XYRectangle,
};

use crate::texture::text::{CheckerTexture, ImageTexture, NoiseTexture};
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

    pub fn random_scene() -> HittableList {
        let mut world = HittableList::default();
        let checker = Arc::new(CheckerTexture::new_col(
            &Color::new(0.2, 0.3, 0.1),
            &Color::new(0.9, 0.9, 0.9),
        ));
        let ground_material = Arc::new(Lambertian::newp(checker));
        world.add(Arc::new(Sphere::new(
            Point::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_double();
                let center = Point::new(
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                );

                if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let albedo = Color::random() * Color::random();
                        let sphere_material = Arc::new(Lambertian::new(albedo));
                        /* world.add(Arc::new(Sphere::new(center, 0.2, sphere_material))); */
                        let center2 = center + Vec3::new(0.0, random_t(0.0, 0.5), 0.0);
                        world.add(Arc::new(MovingSphere::new(
                            center,
                            center2,
                            0.0,
                            1.0,
                            0.2,
                            sphere_material,
                        )))
                    } else if choose_mat < 0.95 {
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = random_t(0.0, 0.5);
                        let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                    } else {
                        let sphere_material = Arc::new(Dielectric::new(1.5));
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
        let material_1 = Arc::new(Dielectric::new(1.5));
        world.add(Arc::new(Sphere::new(
            Point::new(0.0, 1.0, 0.0),
            1.0,
            material_1,
        )));
        let material_2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        world.add(Arc::new(Sphere::new(
            Point::new(-4.0, 1.0, 0.0),
            1.0,
            material_2,
        )));
        let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(Arc::new(Sphere::new(
            Point::new(4.0, 1.0, 0.0),
            1.0,
            material_3,
        )));
        world
    }

    pub fn two_sphere() -> HittableList {
        let mut objects = HittableList::default();
        let checker = Arc::new(CheckerTexture::new_col(
            &Color::new(0.2, 0.3, 0.1),
            &Color::new(0.9, 0.9, 0.9),
        ));
        let sphere_material = Arc::new(Lambertian::newp(checker));
        objects.add(Arc::new(Sphere::new(
            Point::new(0.0, -10.0, 0.0),
            10.0,
            sphere_material.clone(),
        )));
        objects.add(Arc::new(Sphere::new(
            Point::new(0.0, 10.0, 0.0),
            10.0,
            sphere_material,
        )));
        objects
    }

    pub fn two_perlin_sphere() -> HittableList {
        let mut objects = HittableList::default();
        let pertext = Arc::new(NoiseTexture::new(4.0));
        let permater = Arc::new(Lambertian::newp(pertext));
        objects.add(Arc::new(Sphere::new(
            Point::new(0.0, -1000.0, 0.0),
            1000.0,
            permater.clone(),
        )));
        objects.add(Arc::new(Sphere::new(
            Point::new(0.0, 2.0, 0.0),
            2.0,
            permater,
        )));
        objects
    }

    pub fn earth() -> HittableList {
        let earth_texture = Arc::new(ImageTexture::new(&String::from("mercury.jpg")));
        let earth_surface = Arc::new(Lambertian::newp(earth_texture));
        let globe = Arc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, earth_surface));

        let mut world = HittableList::default();
        world.add(globe);
        world
    }

    pub fn simple_light() -> HittableList {
        let mut objects = HittableList::default();
        let pertext = Arc::new(NoiseTexture::new(4.0));
        let mat = Arc::new(Lambertian::newp(pertext));
        objects.add(Arc::new(Sphere::new(
            Point::new(0.0, -1000.0, 0.0),
            1000.0,
            mat.clone(),
        )));
        objects.add(Arc::new(Sphere::new(
            Point::new(0.0, 2.0, 0.0),
            2.0,
            mat,
        )));
        let difflight = Arc::new(DiffuseLight::new_col(Color::new(4.0, 4.0, 4.0)));
        objects.add(Arc::new(XYRectangle::new(
            3.0, 5.0, 1.0, 3.0, -2.0, difflight,
        )));

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
}
