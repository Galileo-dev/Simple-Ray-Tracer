use crate::shapes::HittableList;

use super::{
    constants::INFINITY,
    vec3::{color, unit_vector, ColorRGB, Point3, Vec3, Vector},
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    point: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn origin(self) -> Point3 {
        self.point
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    pub fn at(self, t: f64) -> Point3 {
        self.point + self.direction * t
    }
}

pub fn new_ray(point: &Point3, direction: &Vec3) -> Ray {
    Ray {
        point: *point,
        direction: *direction,
    }
}

pub fn ray_color(ray: &Ray, world: &mut HittableList, depth: i32) -> ColorRGB {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, INFINITY) {
        Some(rec) => {
            let material = &rec.material;
            match material.scatter(ray, &rec) {
                Some((attenuation, scattered)) => {
                    return attenuation * ray_color(&scattered, world, depth - 1);
                }

                None => {}
            }
            return color(0.0, 0.0, 0.0);
        }

        None => {}
    }
    let unit_direction = unit_vector(ray.direction());
    let t = (unit_direction.y() + 1.0) * 0.5;
    return ColorRGB {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * (1.0 - t)
        + ColorRGB {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * t;
}
