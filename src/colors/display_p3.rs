use nalgebra::{Matrix3, Vector3};

use super::utils::{MatrixTRCTransform, ParametricCurveType3};
use super::{Color, CIEXYZ};

/// A color represented in the Apple Display P3 color space.
///
/// The Display P3 color space is similar to the DCI-P3 color space but uses Illuminant D65 as its
/// white point and uses the sRGB gamma transfer curve.
///
/// See https://en.wikipedia.org/wiki/DCI-P3#Display_P3 for more information.
#[derive(Clone, Copy, Debug)]
pub struct DisplayP3 {
    vec: Vector3<f64>,
}

impl Color for DisplayP3 {
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
        33759.0 / 65536.0, 19135.0 / 65536.0, 10296.0 / 65536.0,
        15807.0 / 65536.0, 45367.0 / 65536.0,  4363.0 / 65536.0,
          -69.0 / 65536.0,  2745.0 / 65536.0, 51385.0 / 65536.0,
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
