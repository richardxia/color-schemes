use nalgebra::Vector3;

use super::color::Color;

/// A color represented in the CIE 1931 XYZ color space.
///
/// This color space is treated as the canonical color space, and all other color spaces must
/// define transformation functions to and from this one.
///
/// Y is chosen such that it represents luminance. Z is chosen to roughly represent the blue
/// (short) cone response. X is chosen to have non-negative values for non-imaginary colors. X, Y,
/// and Z are normalized such that the brightest white has Y = 1.0.
///
/// See https://en.wikipedia.org/wiki/CIE_1931_color_space for more information.
#[derive(Clone, Copy, Debug)]
pub struct CIEXYZ {
    vec: Vector3<f64>, // Components are X, Y, and Z
}

impl CIEXYZ {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        CIEXYZ {
            vec: Vector3::new(x, y, z),
        }
    }

    pub fn get_luminance(&self) -> f64 {
        self.vec[1]
    }
}

impl Color for CIEXYZ {
    fn from_ciexyz(ciexyz: &CIEXYZ) -> Self {
        *ciexyz
    }

    fn to_ciexyz(&self) -> CIEXYZ {
        *self
    }

    fn from_vector3(vec: Vector3<f64>) -> Self {
        CIEXYZ { vec }
    }

    fn to_vector3(&self) -> Vector3<f64> {
        self.vec
    }
}

pub fn get_d65_ciexyz() -> CIEXYZ {
    CIEXYZ::from_vector3(Vector3::new(0.95047, 1.0, 1.08883))
}
