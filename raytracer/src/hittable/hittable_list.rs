use super::super::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
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

    pub fn random_scene() -> HittableList {
        let mut world = HittableList::default();
        let ground_material = Arc::new(Lambertian::new(0.5, 0.5, 0.5));
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
                        let sphere_material =
                            Arc::new(Lambertian::new(albedo.x, albedo.y, albedo.z));
                        let center2 = center + Vec3::new(0.0, random_t(0.0, 0.5), 0.0);
                        world.add(Arc::new(MovingSphere::new(
                            center,
                            center2,
                            0.0,
                            1.0,
                            0.2,
                            sphere_material,
                        )));
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
        let material_2 = Arc::new(Lambertian::new(0.4, 0.2, 0.1));
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
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
                // rec.clone_from(&temp_rec);
            }
        }
        rec
    }
}
