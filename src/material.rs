use crate::math::base::min;
use crate::math::rand::{random_f64, random_in_unit_sphere, random_unit_vector};
use crate::math::vec3::{color, dot, unit_vector, ColorRGB, Vec3, Vector};

use crate::{new_ray, Ray};

use super::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(ColorRGB, Ray)>;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    pub albedo: ColorRGB,
}

impl Material for Lambertian {
    #[allow(unused_variables)]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(ColorRGB, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = new_ray(&rec.p, &scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    pub albedo: ColorRGB,
    pub roughness: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(ColorRGB, Ray)> {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        let scattered = new_ray(
            &rec.p,
            &(reflected + self.roughness * random_in_unit_sphere()),
        );
        let attenuation = self.albedo;

        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(ColorRGB, Ray)> {
        //let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        // let scattered = new_ray(rec.p, reflected + self.roughness * random_in_unit_sphere());
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let attenuation = color(1.0, 1.0, 1.0);
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f64(0.0, 1.0) {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        let scattered = new_ray(&rec.p, &direction);
        return Some((attenuation, scattered));
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return v - 2.0 * dot(&v, &n) * n;
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = min(dot(&-uv, &n), 1.0);
    let r_out_perp = etai_over_etat * (uv + (cos_theta * n));
    let r_out_parallel = -(((1.0 - r_out_perp.length_squared()).abs()).sqrt()) * n;
    return r_out_perp + r_out_parallel;
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
