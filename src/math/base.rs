use super::constants::PI;

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

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}
