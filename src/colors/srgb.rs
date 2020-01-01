use nalgebra::{Matrix3, Vector3};

use super::utils::{MatrixTRCTransform, ParametricCurveType3};
use super::{Color, CIEXYZ};

/// A color represented in the sRGB color space.
///
/// This color space is the standard on the web (https://www.w3.org/TR/css-color-3/#rgb-color), and
/// all hexadecimal colors seen on the web, such as in CSS, are generally specified in the sRGB
/// color space.
///
/// See https://en.wikipedia.org/wiki/SRGB for more information.
#[derive(Clone, Copy, Debug)]
pub struct SRGB {
    vec: Vector3<f64>,
}

impl Color for SRGB {
    fn from_ciexyz(ciexyz: &CIEXYZ) -> Self {
        let in_vec = ciexyz.to_vector3();
        let out_vec = matrix_trc_transform().to_device_space(in_vec);
        Self::from_vector3(out_vec)
    }

    fn to_ciexyz(&self) -> CIEXYZ {
        let in_vec = self.to_vector3();
        let out_vec = matrix_trc_transform().to_profile_connection_space(in_vec);
        CIEXYZ::from_vector3(out_vec)
    }

    fn from_vector3(vec: Vector3<f64>) -> Self {
        Self { vec }
    }

    fn to_vector3(&self) -> Vector3<f64> {
        self.vec
    }
}

const TONE_RESPONSE_CURVE: ParametricCurveType3 = ParametricCurveType3 {
    g: 157_286.0 / 65536.0,
    a: 62119.0 / 65536.0,
    b: 3417.0 / 65536.0,
    c: 5072.0 / 65536.0,
    d: 2651.0 / 65536.0,
};

#[rustfmt::skip]
fn transform_matrix() -> Matrix3<f64> {
    // Note: The columns each correspond to the red, green, and blue tristimulus values
    Matrix3::new(
        28578.0 / 65536.0, 25241.0 / 65536.0,  9376.0 / 65536.0,
        14581.0 / 65536.0, 46981.0 / 65536.0,  3972.0 / 65536.0,
          912.0 / 65536.0,  6362.0 / 65536.0, 46799.0 / 65536.0,
    )
}

#[rustfmt::skip]
fn chromatic_adaptation_matrix() -> Matrix3<f64> {
    Matrix3::new(
        68674.0 / 65536.0,  1502.0 / 65536.0, -3290.0 / 65536.0,
         1939.0 / 65536.0, 64912.0 / 65536.0, -1118.0 / 65536.0,
         -605.0 / 65536.0,   988.0 / 65536.0, 49262.0 / 65536.0,
    )
}

fn matrix_trc_transform() -> MatrixTRCTransform {
    MatrixTRCTransform {
        matrix: transform_matrix(),
        red_trc: TONE_RESPONSE_CURVE,
        green_trc: TONE_RESPONSE_CURVE,
        blue_trc: TONE_RESPONSE_CURVE,
        chromatic_adaptation_matrix: chromatic_adaptation_matrix(),
    }
}
