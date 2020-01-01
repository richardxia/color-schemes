use nalgebra::Vector3;

use super::{Color, CIEXYZ};

/// A color represented in the CIE 1931 xyY color space.
///
/// This color space is derived from a simple transformation of the CIE 1931 XYZ color space, where
/// Y is equivalent to the Y from XYZ space (luminance), but x and y are normalized by the quantity
/// (X + Y + Z).
///
/// The resulting x and y values are then used in the CIE xy chromaticity diagram/color space,
/// which has a number of useful properties.
///
/// See
/// https://en.wikipedia.org/wiki/CIE_1931_color_space#CIE_xy_chromaticity_diagram_and_the_CIE_xyY_color_space
/// for more information.
#[derive(Clone, Copy, Debug)]
pub struct CIEXYY {
    vec: Vector3<f64>, // Components are x, y, and Y
}

impl Color for CIEXYY {
    #[allow(non_snake_case)]
    fn from_ciexyz(ciexyz: &CIEXYZ) -> Self {
        let ciexyz_vec = ciexyz.to_vector3();
        let X = ciexyz_vec[0];
        let Y = ciexyz_vec[1];
        let Z = ciexyz_vec[2];
        let denom = X + Y + Z;
        Self::from_vector3(Vector3::new(X / denom, Y / denom, Y))
    }

    #[allow(non_snake_case)]
    fn to_ciexyz(&self) -> CIEXYZ {
        let v = self.to_vector3();
        let x = v[0];
        let y = v[1];
        let Y = v[2];
        let z = 1.0 - x - y;
        CIEXYZ::from_vector3(Vector3::new(Y / y * x, Y, Y / y * z))
    }

    fn from_vector3(vec: Vector3<f64>) -> Self {
        CIEXYY { vec }
    }

    fn to_vector3(&self) -> Vector3<f64> {
        self.vec
    }
}
