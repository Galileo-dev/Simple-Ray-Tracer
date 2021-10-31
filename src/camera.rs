#[allow(dead_code)]
// use raytracing::math_helper::{
//     color, cross, degrees_to_radians, dot, point, random_in_unit_disk, unit_vector, vec3,
//     write_color, Color, Point3, Vec3, Vector,
// };
use crate::math::base::degrees_to_radians;
use crate::math::rand::random_in_unit_disk;
use crate::math::vec3::{cross, unit_vector, Point3, Vec3, Vector};

use crate::{new_ray, Ray};

#[allow(dead_code)]
pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    vfov: f64,
    h: f64,
    lens_radius: f64,
    w: Vec3,
    v: Vec3,
    u: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(&vup, &w));
        let v = cross(&w, &u);

        let focal_length = 1.0;
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            vfov,
            h,
            lens_radius,
            w,
            v,
            u,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        return new_ray(
            &(self.origin + offset),
            &(self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset),
        );
    }
}
