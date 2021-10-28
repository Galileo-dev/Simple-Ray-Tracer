use super::rand::random_f64;

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

pub type ColorRGB = Vec3;
pub type Point3 = Vec3;

pub fn color(x: f64, y: f64, z: f64) -> ColorRGB {
    ColorRGB { x: x, y: y, z: z }
}

pub fn point(x: f64, y: f64, z: f64) -> Point3 {
    Point3 { x: x, y: y, z: z }
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x: x, y: y, z: z }
}

pub fn unit_vector(vec3: Vec3) -> Vec3 {
    return vec3 / vec3.length();
}

pub fn dot(r: &Vec3, other: &Vec3) -> f64 {
    return r.x * other.x + r.y * other.y + r.z * other.z;
}

pub fn cross(r: &Vec3, other: &Vec3) -> Vec3 {
    return Vec3 {
        x: r.y * other.z - r.z * other.y,
        y: r.z * other.x - r.x * other.z,
        z: r.x * other.y - r.y * other.x,
    };
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
