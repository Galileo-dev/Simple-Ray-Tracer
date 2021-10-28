#[allow(dead_code)]
use std::fs::File;
use std::io::{Error, Write};

use std::sync::Arc;
use std::time::Instant;

// mod CONSTANTS;
// use CONSTANTS::{INFINITY, PI};

mod math;
use math::rand::random_f64;
use math::vec3::{color, point, unit_vector, vec3, ColorRGB, Point3, Vec3, Vector};

use math::constants::INFINITY;

mod save;

mod material;
use material::{Lambertian, Material};

mod shapes;
use shapes::{sphere, HitRecord};

mod camera;
use camera::Camera;

use save::ppm_header;

use crate::material::{Dielectric, Metal};
use crate::save::save_color;
use crate::shapes::HittableList;
fn main() -> Result<(), Error> {
    //?Create a new file for image
    let path = "Image.ppm";
    let file = File::create(path)?;

    //?Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 10;
    let max_depth = 50;

    //?World
    let mut world = random_scene();

    pub fn random_scene() -> HittableList {
        let mut world = HittableList { objects: vec![] };
        let ground_material = Arc::new(Lambertian {
            albedo: color(0.5, 0.5, 0.5),
        });
        world.add(Arc::new(sphere(
            point(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_f64(0.0, 1.0);
                let center = point(
                    a as f64 + 0.9 * random_f64(0.0, 1.0),
                    0.2,
                    b as f64 + 0.9 * random_f64(0.0, 1.0),
                );

                if (center - point(4.0, 0.2, 0.0)).length() > 0.9 {
                    let sphere_material: Arc<dyn Material>;
                    if choose_mat < 0.8 {
                        // diffuse
                        let albedo = ColorRGB::random(0.0, 1.0) * ColorRGB::random(0.0, 1.0);
                        sphere_material = Arc::new(Lambertian { albedo });
                        world.add(Arc::new(sphere(center, 0.2, sphere_material)));
                    } else if choose_mat < 0.95 {
                        // metal
                        let albedo = ColorRGB::random(0.5, 1.0);
                        let fuzz = random_f64(0.0, 0.5);
                        sphere_material = Arc::new(Metal {
                            albedo,
                            roughness: fuzz,
                        });
                        world.add(Arc::new(sphere(center, 0.2, sphere_material)));
                    } else {
                        // glass
                        sphere_material = Arc::new(Dielectric {
                            index_of_refraction: 1.5,
                        });
                        world.add(Arc::new(sphere(center, 0.2, sphere_material)));
                    }
                }
            }
        }

        let material1 = Arc::new(Dielectric {
            index_of_refraction: 1.5,
        });
        world.add(Arc::new(sphere(point(0.0, 1.0, 0.0), 1.0, material1)));

        let material2 = Arc::new(Lambertian {
            albedo: color(0.4, 0.2, 0.1),
        });
        world.add(Arc::new(sphere(point(-4.0, 1.0, 0.0), 1.0, material2)));

        let material3 = Arc::new(Metal {
            albedo: color(0.7, 0.6, 0.5),
            roughness: 0.0,
        });
        world.add(Arc::new(sphere(point(4.0, 1.0, 0.0), 1.0, material3)));

        return world;
    }

    //?Camera

    let lookfrom = point(13.0, 2.0, 3.0);
    let lookat = point(0.0, 0.0, 0.0);
    // let dist_to_focus = (lookfrom - lookat).length();
    let dist_to_focus = 10.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        dist_to_focus,
    );

    //timer
    let before = Instant::now();

    //?Choose A header
    ppm_header(&file, image_width, image_height).expect("PPM Header Creation Failed");

    //?Renderer
    for j in (0..image_height).rev() {
        print!(
            "\rScanLine {}/{}  {:?}%  Time Elapsed: {:.2?}s...",
            image_height - j,
            image_height,
            ((image_height as f64 - j as f64) / image_height as f64 * 100.0) as i32,
            before.elapsed().as_secs_f32()
        );
        std::io::stdout().flush()?;
        for i in 0..image_width {
            let mut pixel_color = color(0.0, 0.0, 0.0);
            // dont change sample to j otherwise it wont work!
            for _sample in 0..samples_per_pixel {
                let u = (i as f64 + random_f64(0.0, 1.0)) / (image_width as f64 - 1.0);
                let v = (j as f64 + random_f64(0.0, 1.0)) / (image_height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, &mut world, max_depth);
            }
            save_color(&file, pixel_color, samples_per_pixel).expect("Failed to save color");
        }
    }
    let elapsed = before.elapsed();
    eprintln!("Done in {:.2?}...", elapsed);

    Ok(())
}

fn ray_color(ray: Ray, world: &mut HittableList, depth: i32) -> ColorRGB {
    if depth <= 0 {
        return color(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, INFINITY) {
        Some(rec) => {
            let material = &rec.material;
            match material.scatter(ray, &rec) {
                Some((attenuation, scattered)) => {
                    return attenuation * ray_color(scattered, world, depth - 1);
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
