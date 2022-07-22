use super::super::material::{
    dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
};
use super::aabb::AABB;
use super::triangle::Object;
use super::{
    super::basic_tools::{
        ray::Ray,
        vec3::{Color, Point, Vec3},
    },
    hittable_origin::random_int,
};
use super::{
    bvh::BVHNode,
    fog::ConstantMedium,
    hittable_origin::{random_t, HitRecord, Hittable},
    moving_sphere::MovingSphere,
    sphere::Sphere,
    triangle::Triangle,
    xy_rectangle::{Cube, FlipFace, RotateY, XYRectangle, XZRectangle, YZRectangle},
};

use crate::texture::text::{ImageTexture, NoiseTexture};
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

    pub fn final_scence() -> HittableList {
        let mut boxes1 = HittableList::default();
        let ground = Lambertian::new(Color::new(0.48, 0.83, 0.53));

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

        let light = DiffuseLight::new_col(Color::new(1.0, 1.0, 1.0), 7.0);
        let lamp = XZRectangle::new(123.0, 423.0, 147.0, 412.0, 554.0, light);
        objects.add(Arc::new(FlipFace::new(Arc::new(lamp))));

        //=== moving sphere ====
        let center1 = Point::new(400.0, 400.0, 200.0);
        let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
        let moving_sphere_material = Lambertian::new(Color::new(0.7, 0.3, 0.1));
        objects.add(Arc::new(MovingSphere::new(
            center1,
            center2,
            0.0,
            1.0,
            50.0,
            moving_sphere_material,
        )));

        //=== metal and glass ===
        let glass_mat = Dielectric::new(1.5);
        let metal_mat = Metal::new(Color::new(0.8, 0.8, 0.9), 1.0);
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
            Point::new(360.0, 150.0, 145.0),
            70.0,
            glass_mat.clone(),
        ));
        objects.add(boundary.clone());
        let smoke_ball = Arc::new(ConstantMedium::new_col(
            boundary,
            0.2,
            &Color::new(0.2, 0.4, 0.9),
        ));
        objects.add(smoke_ball);
        let boundary = Arc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 5000.0, glass_mat));
        let smoke_ball = Arc::new(ConstantMedium::new_col(
            boundary,
            0.0001,
            &Color::new(1.0, 1.0, 1.0),
        ));
        objects.add(smoke_ball);

        //=== earth ===
        let imagetx = ImageTexture::new(&String::from("earthmap.jpg"));
        let emat = DiffuseLight::new(imagetx, 1.0);
        let earth = Arc::new(Sphere::new(Point::new(400.0, 200.0, 400.0), 100.0, emat));
        objects.add(earth);

        //=== noise box ===
        let pertext = NoiseTexture::new(0.1);
        let perball = Arc::new(Sphere::new(
            Point::new(220.0, 280.0, 300.0),
            80.0,
            Lambertian::newp(pertext),
        ));
        objects.add(perball);

        //=== boxes contain many boxes
        /*  let mut boxes2 = HittableList::default();
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
        objects.add(many_balls); */

        objects
    }

    pub fn lights() -> HittableList {
        let mut lights = HittableList::default();
        let light = DiffuseLight::new_col(Color::new(1.0, 1.0, 1.0), 15.0);
        let lamp = Arc::new(XZRectangle::new(213.0, 343.0, 227.0, 332.0, 554.0, light));
        lights.add(lamp);
        /*   let ball = Arc::new(Sphere::new(Point::new(190.0, 90.0, 190.0), 90.0, light));
        lights.add(ball); */
        lights
    }

    pub fn lights_final_scence() -> HittableList {
        let mut lights = HittableList::default();
        let light = DiffuseLight::new_col(Color::new(1.0, 1.0, 1.0), 7.0);
        let lamp = Arc::new(XZRectangle::new(
            123.0,
            423.0,
            147.0,
            412.0,
            554.0,
            light.clone(),
        ));
        lights.add(lamp);
        lights.add(Arc::new(Sphere::new(
            Point::new(260.0, 150.0, 45.0),
            50.0,
            light,
        )));
        lights
    }
    pub fn cornell_box() -> HittableList {
        let mut objects = HittableList::default();
        /*   let emat = Arc::new(Lambertian::newp(Arc::new(ImageTexture::new(
            &String::from("earthmap.jpg"),
        )))); */

        let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
        let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
        let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
        let light = DiffuseLight::new_col(Color::new(1.0, 1.0, 1.0), 15.0);

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

        let p1 = Point::new(120.0, 0.0, 350.0);
        let p2 = Point::new(500.0, 0.0, 380.0);
        let p3 = Point::new(33.0, 0.0, 394.0);
        let p4 = Point::new(250.0, 300.0, 300.0);

        /*  let trian = Arc::new(Triangle::new(
            Point::new(50.0, 200.0, 250.0),
            Point::new(500.0, 20.0, 280.0),
            Point::new(33.0, 19.0, 294.0),
            white.clone(),
        ));
        objects.add(trian); */
        let mut trian = HittableList::default();

        let trian1 = Arc::new(Triangle::new(p1, p2, p4, white.clone()));
        let trian2 = Arc::new(Triangle::new(p3, p2, p4, white.clone()));
        let trian3 = Arc::new(Triangle::new(p1, p3, p4, white));
        trian.add(trian1);
        trian.add(trian2);
        trian.add(trian3);

        let pre = Arc::new(Object::new_hittable(&trian));
        let rota = Arc::new(RotateY::new(pre, 0.0));

        objects.add(rota);

        /*  let rectan = Arc::new(XYRectangle::new(165.0, 330.0, 165.0, 330.0, 550.0, white));
        objects.add(rectan); */
        /*  let box1 = Arc::new(Cube::new(
            Point::new(0.0, 0.0, 0.0),
            Point::new(165.0, 330.0, 165.0),
            white,
        ));
        let box1 = Arc::new(RotateY::new(box1, 15.0));
        let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

        objects.add(box1); */

        /*   let glass = Arc::new(Dielectric::new(1.5));
        let ball = Arc::new(Sphere::new(Point::new(190.0, 90.0, 190.0), 90.0, glass));
        objects.add(ball); */

        objects
    }
}

impl Hittable for HittableList {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord<'a>) -> bool {
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
