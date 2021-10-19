use std::fs::File;

use std::io::{Result, Write};

use crate::math::base::clamp;
use crate::math::vec3::{ColorRGB, Vector};

pub fn ppm_header(mut file: &File, image_width: i32, image_height: i32) -> Result<()> {
    let mut buff = Vec::new();
    writeln!(&mut buff, "P3")?;
    writeln!(&mut buff, "{} {}", image_width, image_height)?;
    writeln!(&mut buff, "255")?;
    file.write(&buff)?;

    Ok(())
}

pub fn save_color(mut file: &File, color: ColorRGB, samples_per_pixel: i32) -> Result<()> {
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
