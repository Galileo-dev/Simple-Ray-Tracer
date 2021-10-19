use std::sync::Arc;

use crate::material::Material;
use crate::math::vec3::{dot, Point3, Vec3};
use crate::Ray;
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: &Vec3) {
        self.front_face = dot(ray.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub mod sphere;
use sphere::Sphere;

pub fn sphere(center: Point3, radius: f64, material: Box<dyn Material>) -> Sphere {
    Sphere {
        center,
        radius,
        material,
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object.clone());
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl HittableList {
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            match object.hit(ray, t_min, closest_so_far) {
                Some(rec) => {
                    hit_anything = true;
                    closest_so_far = rec.t;
                    temp_rec.replace(rec);
                }
                None => {}
            }
        }
        if hit_anything {
            temp_rec
        } else {
            None
        }
    }
}
