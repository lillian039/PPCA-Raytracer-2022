use super::super::basic_tools::{
    ray::Ray,
    vec3::{Color, Point, Vec3},
};
use super::super::material::{
    dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
};
use super::aabb::AABB;
use super::{
    bvh::BVHNode,
    fog::ConstantMedium,
    hittable_origin::{random_double, random_t, HitRecord, Hittable},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    xy_rectangle::{Cube, RotateY, Translate, XYRectangle, XZRectangle, YZRectangle},
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
        objects.add(Arc::new(Sphere::new(Point::new(0.0, 2.0, 0.0), 2.0, mat)));
        let difflight = Arc::new(DiffuseLight::new_col(Color::new(4.0, 4.0, 4.0)));
        objects.add(Arc::new(XYRectangle::new(
            3.0,
            5.0,
            1.0,
            3.0,
            -2.0,
            difflight.clone(),
        )));
        objects.add(Arc::new(Sphere::new(
            Point::new(0.0, 7.0, 0.0),
            2.0,
            difflight,
        )));

        objects
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
        objects.add(Arc::new(XZRectangle::new(
            213.0, 343.0, 227.0, 332.0, 554.0, light,
        )));
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
            white.clone(),
        ));
        let box1 = Arc::new(RotateY::new(box1, 15.0));
        let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

        objects.add(box1);

        let box2 = Arc::new(Cube::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(165.0, 165.0, 165.0),
            white,
        ));
        let box2 = Arc::new(RotateY::new(box2, -18.0));
        let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
        objects.add(box2);

        objects
    }

    pub fn cornell_smoke() -> HittableList {
        let mut objects = HittableList::default();

        let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
        let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
        let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
        let light = Arc::new(DiffuseLight::new_col(Color::new(7.0, 7.0, 7.0)));

        objects.add(Arc::new(YZRectangle::new(
            0.0, 555.0, 0.0, 555.0, 555.0, green,
        )));
        objects.add(Arc::new(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
        objects.add(Arc::new(XZRectangle::new(
            113.0, 443.0, 127.0, 432.0, 554.0, light,
        )));
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
            white.clone(),
        ));
        let box1 = Arc::new(RotateY::new(box1, 15.0));
        let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
        let box1 = Arc::new(ConstantMedium::new_col(
            box1,
            0.01,
            Color::new(0.0, 0.0, 0.0),
        ));

        objects.add(box1);

        let box2 = Arc::new(Cube::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(165.0, 165.0, 165.0),
            white,
        ));
        let box2 = Arc::new(RotateY::new(box2, -18.0));
        let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
        let box2 = Arc::new(ConstantMedium::new_col(
            box2,
            0.01,
            Color::new(1.0, 1.0, 1.0),
        ));
        objects.add(box2);

        objects
    }

    pub fn final_scence() -> HittableList {
        let mut boxes1 = HittableList::default();
        let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = random_t(1.0, 101.0);
                let z1 = z0 + w;

                boxes1.add(Arc::new(Cube::new(
                    Point::new(x0, y0, z0),
                    Point::new(x1, y1, z1),
                    ground.clone(),
                )));
            }
        }

        let mut objects = HittableList::default();
        objects.add(Arc::new(BVHNode::new(
            boxes1.objects.clone(),
            0,
            boxes1.objects.len(),
            0.0,
            1.0,
        )));

        let light = Arc::new(DiffuseLight::new_col(Color::new(7.0, 7.0, 7.0)));
        objects.add(Arc::new(XZRectangle::new(
            123.0, 423.0, 147.0, 412.0, 554.0, light,
        )));

        //=== moving sphere ====
        let center1 = Point::new(400.0, 400.0, 200.0);
        let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
        let moving_sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
        objects.add(Arc::new(MovingSphere::new(
            center1,
            center2,
            0.0,
            1.0,
            50.0,
            moving_sphere_material,
        )));

        //=== metal and glass ===
        let glass_mat = Arc::new(Dielectric::new(1.5));
        let metal_mat = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0));
        objects.add(Arc::new(Sphere::new(
            Point::new(260.0, 150.0, 45.0),
            50.0,
            glass_mat.clone(),
        )));
        objects.add(Arc::new(Sphere::new(
            Point::new(0.0, 150.0, 145.0),
            50.0,
            metal_mat,
        )));

        //=== smoke ===
        let boundary = Arc::new(Sphere::new(
            Point::new(360.0, 150.0, 45.0),
            70.0,
            glass_mat.clone(),
        ));
        // objects.add(boundary.clone());
        let smoke_ball = Arc::new(ConstantMedium::new_col(
            boundary,
            0.2,
            Color::new(0.2, 0.4, 0.9),
        ));
        objects.add(smoke_ball);
        let boundary = Arc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 5000.0, glass_mat));
        let smoke_ball = Arc::new(ConstantMedium::new_col(
            boundary,
            0.0001,
            Color::new(1.0, 1.0, 1.0),
        ));
        objects.add(smoke_ball);

        //=== earth ===
        let emat = Arc::new(Lambertian::newp(Arc::new(ImageTexture::new(
            &String::from("earthmap.jpg"),
        ))));
        let earth = Arc::new(Sphere::new(Point::new(400.0, 200.0, 400.0), 100.0, emat));
        objects.add(earth);

        //=== noise box ===
        let pertext = Arc::new(NoiseTexture::new(0.1));
        let perball = Arc::new(Sphere::new(
            Point::new(220.0, 280.0, 300.0),
            80.0,
            Arc::new(Lambertian::newp(pertext)),
        ));
        objects.add(perball);

        //=== boxes contain many boxes
        let mut boxes2 = HittableList::default();
        let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
        let ns = 1000;
        for _j in 0..ns {
            boxes2.add(Arc::new(Sphere::new(
                Point::random_range(0.0, 165.0),
                10.0,
                white.clone(),
            )));
        }
        let many_balls = Arc::new(BVHNode::new(
            boxes2.objects.clone(),
            0,
            boxes2.objects.len(),
            0.0,
            1.0,
        ));
        let many_balls = Arc::new(RotateY::new(many_balls, 15.0));
        let many_balls = Arc::new(Translate::new(many_balls, Vec3::new(-100.0, 270.0, 395.0)));
        objects.add(many_balls);

        objects
    }

    pub fn planets() -> HittableList {
        let mut objects = HittableList::default();

        let light = Arc::new(DiffuseLight::new_col(Color::new(10.0, 10.0, 10.0)));
        objects.add(Arc::new(Sphere::new(
            Point::new(0.0, 10.0, 0.0),
            10.0,
            light,
        )));
        let glass_mat = Arc::new(Dielectric::new(1.5));
        let boundary = Arc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 5000.0, glass_mat));
        let smoke_ball = Arc::new(ConstantMedium::new_col(
            boundary,
            0.0001,
            Color::new(1.0, 1.0, 1.0),
        ));
        objects.add(smoke_ball);

        let light = Arc::new(DiffuseLight::new_col(Color::new(7.0, 7.0, 7.0)));
        objects.add(Arc::new(XZRectangle::new(
            100.0, -100.0, 100.0, -100.0, 200.0, light,
        )));

        let emat = Arc::new(Lambertian::newp(Arc::new(ImageTexture::new(
            &String::from("earthmap.jpg"),
        ))));
        let earth = Arc::new(Sphere::new(Point::new(200.0, 100.0, 0.0), 70.0, emat));
        objects.add(earth);
        let mermat = Arc::new(Lambertian::newp(Arc::new(ImageTexture::new(
            &String::from("mercury.jpg"),
        ))));
        let mercury = Arc::new(Sphere::new(Point::new(-150.0, 129.0, 0.0), 70.0, mermat));
        objects.add(mercury);
        let marsmat = Arc::new(Lambertian::newp(Arc::new(ImageTexture::new(
            &String::from("Mars.jpg"),
        ))));
        let mars = Arc::new(Sphere::new(Point::new(-25.0, 88.0, 0.0), 50.0, marsmat));
        objects.add(mars);
        let metal_mat = Arc::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.0));
        let ground = Arc::new(Sphere::new(
            Point::new(0.0, -10000.0, 0.0),
            10000.0,
            metal_mat,
        ));
        objects.add(ground);

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
