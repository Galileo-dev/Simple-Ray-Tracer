use std::fs::File;

use std::io::{BufWriter, LineWriter};
use std::io::{Result, Write};

use crate::math::base::clamp;
use crate::math::vec3::{ColorRGB, Vector};

pub fn ppm_header(file: &mut BufWriter<File>, image_width: i32, image_height: i32) -> Result<()> {
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", image_width, image_height)?;
    writeln!(file, "255")?;

    Ok(())
}

pub fn save_color(
    file: &mut BufWriter<File>,
    color: ColorRGB,
    samples_per_pixel: i32,
) -> Result<()> {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    //Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    writeln!(
        file,
        "{} {} {}",
        255.999 * clamp(r, 0.0, 0.999),
        255.999 * clamp(g, 0.0, 0.999),
        255.999 * clamp(b, 0.0, 0.999)
    )
    .expect("Error Writing To File");
    Ok(())
}

pub fn write_color(color: ColorRGB, samples_per_pixel: i32) {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    //Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    println!(
        "{} {} {}",
        255.999 * clamp(r, 0.0, 0.999),
        255.999 * clamp(g, 0.0, 0.999),
        255.999 * clamp(b, 0.0, 0.999)
    )
}
