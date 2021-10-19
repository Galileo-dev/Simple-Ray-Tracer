use std::fmt;

//use rand::{prelude::ThreadRng, Rng};

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{impl_binary_operations, impl_op_assign, impl_unary_operations};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait Vector {
    fn x(&self) -> f64;

    fn y(&self) -> f64;

    fn z(&self) -> f64;

    fn length(self) -> f64;

    fn length_squared(self) -> f64;

    fn random(min: f64, max: f64) -> Vec3;

    fn near_zero(self) -> bool;
}

impl Vector for Vec3 {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn random(min: f64, max: f64) -> Vec3 {
        vec3(
            random_f64(min, max),
            random_f64(min, max),
            random_f64(min, max),
        )
    }

    fn near_zero(self) -> bool {
        let s = 1e-8;
        return (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s);
    }
}

pub type Color = Vec3;
pub type Point = Vec3;

pub fn color(x: f64, y: f64, z: f64) -> Color {
    Color { x: x, y: y, z: z }
}

pub fn point(x: f64, y: f64, z: f64) -> Point {
    Point { x: x, y: y, z: z }
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x: x, y: y, z: z }
}

// Utility Functions

//Todo(): Make this more Efficient
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        // In the same hemisphere as the normal
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

pub fn write_color(color: Color) {
    println!(
        "{} {} {}",
        255.999 * color.x(),
        255.999 * color.y(),
        255.999 * color.z()
    )
}

pub fn unit_vector(vec3: Vec3) -> Vec3 {
    return vec3 / vec3.length();
}

pub fn dot(r: Vec3, other: Vec3) -> f64 {
    return r.x * other.x + r.y * other.y + r.z * other.z;
}

pub fn cross(r: Vec3, other: Vec3) -> Vec3 {
    return Vec3 {
        x: r.y * other.z - r.z * other.y,
        y: r.z * other.x - r.x * other.z,
        z: r.x * other.y - r.y * other.x,
    };
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    // Return a real number
    return rand::f64_in_range(min, max);
    //Todo(): Make more efficent
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    return x;
}

pub fn min(val1: f64, val2: f64) -> f64 {
    if val1 < val2 {
        return val1;
    } else {
        return val2;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = vec3(random_f64(-1.0, 1.0), random_f64(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

// MACROS

impl_binary_operations!(Vec3 Add add +);
impl_op_assign!(Vec3 AddAssign add_assign +);

impl_binary_operations!(Vec3 Sub sub -);
impl_op_assign!(Vec3 SubAssign sub_assign -);
impl_unary_operations!(Vec3 Neg neg -);

impl_binary_operations!(Vec3 Mul mul *);
impl_op_assign!(Vec3 MulAssign mul_assign *);

impl_binary_operations!(Vec3 Div div /);
impl_op_assign!(Vec3 DivAssign div_assign /);
