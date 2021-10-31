use std::sync::Arc;

use crate::material::Material;
use crate::math::ray::Ray;
use crate::math::vec3::{dot, vec3, Point3, Vec3, Vector};

use super::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };

        let sqrt = discriminant.sqrt();

        let mut root = (-half_b - sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec: Option<HitRecord> = Some(HitRecord {
            p: ray.at(root),
            t: root,
            normal: vec3(0.0, 0.0, 0.0),
            material: &self.material,
            front_face: false,
        });
        let outward_normal: Vec3 = (rec.as_ref().unwrap().p - self.center) / self.radius;
        rec.as_mut().unwrap().set_face_normal(ray, &outward_normal);
        rec
    }
}
