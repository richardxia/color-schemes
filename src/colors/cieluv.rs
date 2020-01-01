use nalgebra::Vector3;

use super::{get_d65_ciexyz, Color, CIEXYZ};

/// A color represented in the CIE 1976 L*, u*, v* color space.
///
/// This color space is derived from the CIE 1931 XYZ color space, where it has the property of
/// being more perceptually uniform.
///
/// See https://en.wikipedia.org/wiki/CIELUV for more information.
///
/// Unlike the Wikipedia article, we normalize L to [0, 1].
#[derive(Clone, Copy, Debug)]
pub struct CIELUV {
    vec: Vector3<f64>, // Components are L, u, and v
}

#[allow(non_snake_case)]
fn uv_chromaticities(color: &CIEXYZ) -> (f64, f64) {
    let color_vec = color.to_vector3();
    let X = color_vec[0];
    let Y = color_vec[1];
    let Z = color_vec[2];
    (
        (4.0 * X) / (X + 15.0 * Y + 3.0 * Z),
        (9.0 * Y) / (X + 15.0 * Y + 3.0 * Z),
    )
}

impl Color for CIELUV {
    #[allow(non_snake_case)]
    fn from_ciexyz(ciexyz: &CIEXYZ) -> Self {
        let ciexyz_vec = ciexyz.to_vector3();
        let Y = ciexyz_vec[1];

        let white: CIEXYZ = get_d65_ciexyz();
        let white_Y = white.to_vector3()[1];

        let normalized_Y = Y / white_Y;

        let l = if normalized_Y <= (6.0 / 29.0 as f64).powf(3.0) {
            ((29.0 / 3.0 as f64).powf(3.0) * normalized_Y) / 100.0
        } else {
            (116.0 * normalized_Y.powf(1.0 / 3.0) - 16.0) / 100.0
        };
        let (u_prime, v_prime) = uv_chromaticities(ciexyz);
        let (u_prime_white, v_prime_white) = uv_chromaticities(&white);

        let u = 13.0 * l * (u_prime - u_prime_white);
        let v = 13.0 * l * (v_prime - v_prime_white);

        Self::from_vector3(Vector3::new(l, u, v))
    }

    #[allow(non_snake_case)]
    fn to_ciexyz(&self) -> CIEXYZ {
        let v = self.to_vector3();
        let l = v[0];
        let u = v[1];
        let v = v[2];

        let white: CIEXYZ = get_d65_ciexyz();
        let white_Y = white.to_vector3()[1];
        let (u_prime_white, v_prime_white) = uv_chromaticities(&white);

        let u_prime = u / (13.0 * l) + u_prime_white;
        let v_prime = v / (13.0 * l) + v_prime_white;

        let Y = if (l * 100.0) <= 8.0 {
            white_Y * (l * 100.0) * (3.0 / 29.0 as f64).powf(3.0)
        } else {
            white_Y * (((l * 100.0) + 16.0) / 116.0).powf(3.0)
        };

        let X = Y * 9.0 * u_prime / (4.0 * v_prime);
        let Z = Y * (12.0 - 3.0 * u_prime - 20.0 * v_prime) / (4.0 * v_prime);

        CIEXYZ::from_vector3(Vector3::new(X, Y, Z))
    }

    fn from_vector3(vec: Vector3<f64>) -> Self {
        CIELUV { vec }
    }

    fn to_vector3(&self) -> Vector3<f64> {
        self.vec
    }
}
