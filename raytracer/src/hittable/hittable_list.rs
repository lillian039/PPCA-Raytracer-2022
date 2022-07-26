use super::super::material::{
    dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    mixmaterial::MixtureMaterial,
};
use super::aabb::AABB;
use super::hittable_origin::random_double;
use super::ring::Ring;
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
    // fog::ConstantMedium,
    hittable_origin::{random_t, HitRecord, Hittable},
    sphere::Sphere,
    xy_rectangle::{
        Cube, FlipFace, RotateX, RotateY, Translate, XYRectangle, XZRectangle, YZRectangle,
    },
};

use crate::texture::text::ImageTexture;
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

    pub fn whale_lights() -> HittableList {
        let mut lights = HittableList::default();
        let light = DiffuseLight::new_col(Color::new(1.0, 1.0, 1.0), 20.0);
        let lamp = Arc::new(XZRectangle::new(113.0, 443.0, 127.0, 432.0, 1299.0, light));
        lights.add(lamp);
        lights
    }

    pub fn whale() -> HittableList {
        let mut objects = HittableList::default();

        let back = ImageTexture::new(&String::from("pinkblue.png"));
        let pink = Lambertian::newp(back);
        //  let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
        let light = DiffuseLight::new_col(Color::new(1.0, 1.0, 1.0), 20.0);
        let mut boxes1 = HittableList::default();
        let aluminum = Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.);

        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.0;
                let x0 = -1000.0 + i as f64 * w;
                let z0 = -1000.0 + j as f64 * w;
                let y0 = -20.0;
                let x1 = x0 + w;
                let y1 = random_t(1.0, 101.0);
                let z1 = z0 + w;

                boxes1.add(Arc::new(Cube::new(
                    Point::new(x0, y0, z0),
                    Point::new(x1, y1, z1),
                    aluminum.clone(),
                )));
            }
        }
        objects.add(Arc::new(BVHNode::new(
            boxes1.objects.clone(),
            0,
            boxes1.objects.len(),
            0.0,
            1.0,
        )));

        let glass = Dielectric::new(1.5);

        let cloud = Arc::new(Object::new(
            &String::from("obj/cloud.obj"),
            glass.clone(),
            0.6,
        ));
        let cloud = Arc::new(BVHNode::new(
            cloud.surface.clone().objects,
            0,
            cloud.surface.objects.len(),
            0.0,
            1.0,
        ));

        let move_obj = Arc::new(Translate::new(cloud.clone(), Vec3::new(0.0, 300.0, 300.0)));
        objects.add(move_obj);

        let move_obj = Arc::new(Translate::new(cloud, Vec3::new(500.0, 600.0, 400.0)));
        objects.add(move_obj);

        objects.add(Arc::new(YZRectangle::new(
            0.0,
            1300.0,
            -800.0,
            1355.0,
            1355.0,
            pink.clone(),
        )));
        objects.add(Arc::new(YZRectangle::new(
            0.0,
            1300.0,
            -800.0,
            1355.0,
            -800.0,
            pink.clone(),
        )));
        let lamp = Arc::new(XZRectangle::new(113.0, 443.0, 127.0, 432.0, 1299.0, light));
        objects.add(Arc::new(FlipFace::new(lamp)));

        objects.add(Arc::new(XZRectangle::new(
            -800.0,
            1355.0,
            -800.0,
            1355.0,
            1300.0,
            pink.clone(),
        )));
        objects.add(Arc::new(XYRectangle::new(
            -800.0, 1355.0, 0.0, 1300.0, 1355.0, pink,
        )));
        let light2 = DiffuseLight::new_col(Color::new(1.0, 1.0, 1.0), 1.0);
        let blue = MixtureMaterial::new(light2, glass.clone(), 0.5);

        let obj = Arc::new(Object::new(&String::from("obj/whale.obj"), blue, 800.0));
        let bvh_obj = Arc::new(BVHNode::new(
            obj.surface.clone().objects,
            0,
            obj.surface.objects.len(),
            0.0,
            1.0,
        ));
        let move_obj = Arc::new(RotateY::new(bvh_obj, 180.0));
        let move_obj = Arc::new(Translate::new(move_obj, Vec3::new(300.0, 350.0, 400.0)));

        objects.add(move_obj);

        let center = Vec3::new(300.0, 300.0, 400.0);

        let mut planets_ring = HittableList::default();

        for _i in 0..300 {
            let r = random_t(420.0, 520.0);
            let theta = random_t(0.0, 360.0);
            let high = random_t(0.0, 50.0) - 25.0;
            let posi = Vec3::new(
                center.x + theta.cos() * r,
                center.y + theta.sin() * r,
                center.z + high,
            );
            let rball = random_t(7.0, 12.0);
            let choice = random_double();
            if choice < 0.3 {
                let mat = glass.clone();
                let ball = Arc::new(Sphere::new(posi, rball, mat));
                planets_ring.add(ball);
            } else if choice < 0.7 {
                let albedo = Color::random_range(0.7, 1.0);
                let fuzz = random_t(0.0, 0.5);
                let mat = Metal::new(albedo, fuzz);
                let ball = Arc::new(Sphere::new(posi, rball, mat));
                planets_ring.add(ball);
            } else {
                let col = Color::random_range(0.7, 1.0);
                let mat = DiffuseLight::new_col(col, 0.7);
                let ball = Arc::new(Sphere::new(posi, rball * 0.6, mat));
                planets_ring.add(ball);
            }
        }

        let planets_ring = Arc::new(BVHNode::new(
            planets_ring.clone().objects,
            0,
            planets_ring.objects.len(),
            0.0,
            1.0,
        ));

        //let planets_ring = Arc::new(RotateX::new(planets_ring, 20.0));

        objects.add(planets_ring);

        let emat = DiffuseLight::new(ImageTexture::new(&String::from("earthmap.jpg")), 1.0);
        let earth = Arc::new(Sphere::new(Point::new(200.0, 350.0, 150.0), 50.0, emat));
        objects.add(earth);

        let mermat = DiffuseLight::new(ImageTexture::new(&String::from("mercury.jpg")), 1.2);
        let mercury = Arc::new(Sphere::new(Point::new(360.0, 400.0, 500.0), 50.0, mermat));
        objects.add(mercury);

        let venusmat = DiffuseLight::new(ImageTexture::new(&String::from("venus.jpg")), 1.2);
        let venus = Arc::new(Sphere::new(Point::new(250.0, 488.0, 250.0), 50.0, venusmat));
        objects.add(venus);

        let jupitermat = Lambertian::newp(ImageTexture::new(&String::from("Jupiter.jpg")));
        let light2 = DiffuseLight::new(ImageTexture::new(&String::from("Jupiter.jpg")), 1.0);
        let jupitermat = MixtureMaterial::new(jupitermat, light2, 0.95);
        let jupiter = Arc::new(Sphere::new(
            Point::new(600.0, 300.0, 450.0),
            70.0,
            jupitermat,
        ));
        let jupiter = Arc::new(RotateX::new(jupiter, 20.0));
        objects.add(jupiter);

        let saturnmat = Lambertian::newp(ImageTexture::new(&String::from("Saturn.jpg")));
        let saturn = Arc::new(Sphere::new(
            Point::new(-50.0, 350.0, 350.0),
            60.0,
            saturnmat,
        ));
        let saturn = Arc::new(RotateX::new(saturn, 20.0));
        objects.add(saturn);
        let light =
            DiffuseLight::new_col(Color::new(245.0 / 255.0, 222.0 / 255.0, 179.0 / 255.0), 1.0);
        let transcu = MixtureMaterial::new(light, glass, 0.6);
        let saturn_ring = Arc::new(Ring::new(
            Point::new(-50.0, 350.0, 350.0),
            100.0,
            105.0,
            transcu.clone(),
        ));
        let saturn_ring = Arc::new(RotateX::new(saturn_ring, 20.0));
        objects.add(saturn_ring);
        let saturn_ring = Arc::new(Ring::new(
            Point::new(-50.0, 350.0, 350.0),
            109.0,
            120.0,
            transcu,
        ));
        let saturn_ring = Arc::new(RotateX::new(saturn_ring, 20.0));
        objects.add(saturn_ring);
        objects
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
        let lamp = Arc::new(XZRectangle::new(
            213.0,
            343.0,
            227.0,
            332.0,
            554.0,
            light.clone(),
        ));
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
            0.0, 555.0, 0.0, 555.0, 555.0, white,
        )));

        /*  let obj = Arc::new(Object::new_texture(
            &String::from("obj/patrick.obj"),
            200.0,
            &String::from("obj/Char_Patrick.png"),
        ));
        let bvh_obj = Arc::new(BVHNode::new(
            obj.surface.clone().objects,
            0,
            obj.surface.objects.len(),
            0.0,
            1.0,
        ));
        let move_obj = Arc::new(RotateY::new(bvh_obj, 180.0));
        let move_obj = Arc::new(RotateX::new(move_obj, -30.0));
        let move_obj = Arc::new(Translate::new(move_obj, Vec3::new(200.0, 0.0, 300.0)));

        objects.add(move_obj); */

        let ring = Ring::new(Vec3::new(300.0, 200.0, 400.0), 50.0, 54.0, light.clone());
        let ring = Arc::new(RotateX::new(Arc::new(ring), 30.0));
        objects.add(ring);
        let ring = Ring::new(Vec3::new(300.0, 200.0, 400.0), 58.0, 60.0, light);
        let ring = Arc::new(RotateX::new(Arc::new(ring), 30.0));
        objects.add(ring);
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
