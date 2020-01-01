use nalgebra::{Matrix3, Vector3};

/// ICC parametricCurveType 3
///
/// parametricCurveType 3 is a piecewise, continuous function of the form:
///
/// y = (a*x + b)^g  for (x >= d)
/// y = c * x        for (x < d)
///
/// This function assumes that the input x is defined along the interval [0, 1]. This function is
/// used to implement the tone response curve, which applies a nonlinear transformation to each
/// component of a color. This function is used to from the device color space to the profile
/// connection space (CIE XYZ). The inverse function should be used when mapping in the opposite
/// direction.
pub struct ParametricCurveType3 {
    pub g: f64,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl ParametricCurveType3 {
    /// Apply the parametric curve to a value.
    pub fn apply(&self, x: f64) -> f64 {
        let ParametricCurveType3 { g, a, b, c, d } = *self;
        if x >= d {
            (a * x + b).powf(g)
        } else {
            c * x
        }
    }

    /// Apply the inverse function.
    ///
    /// x = (y^(1/g) - b)/a  for (y >= c*d)
    /// x = y/c              for (y < c*d)
    pub fn apply_inverse(&self, y: f64) -> f64 {
        let ParametricCurveType3 { g, a, b, c, d } = *self;
        if y >= c * d {
            (y.powf(1.0 / g) - b) / a
        } else {
            y / c
        }
    }
}

pub struct MatrixTRCTransform {
    pub matrix: Matrix3<f64>, // Mapping from device space to PCS XYZ, sans chromatic adaptation or tone response curves
    pub red_trc: ParametricCurveType3,
    pub green_trc: ParametricCurveType3,
    pub blue_trc: ParametricCurveType3,
    pub chromatic_adaptation_matrix: Matrix3<f64>, // Chromatically adapt white point in PCS XYZ
}

impl MatrixTRCTransform {
    /// Transform from device space to PCS XYZ
    pub fn to_profile_connection_space(&self, v: Vector3<f64>) -> Vector3<f64> {
        let chromatic_adaptation = self
            .chromatic_adaptation_matrix
            .try_inverse()
            .expect("Matrix is not invertible.");

        chromatic_adaptation * self.matrix * self.apply_trc(v)
    }

    /// Transform from PCS XYZ to device space
    pub fn to_device_space(&self, v: Vector3<f64>) -> Vector3<f64> {
        let linear_transform = self
            .matrix
            .try_inverse()
            .expect("Matrix is not invertible.");
        self.apply_trc_inverse(linear_transform * self.chromatic_adaptation_matrix * v)
    }

    fn apply_trc(&self, v: Vector3<f64>) -> Vector3<f64> {
        let r = v[0];
        let g = v[1];
        let b = v[2];
        Vector3::new(
            self.red_trc.apply(r),
            self.green_trc.apply(g),
            self.blue_trc.apply(b),
        )
    }

    fn apply_trc_inverse(&self, v: Vector3<f64>) -> Vector3<f64> {
        let r = v[0];
        let g = v[1];
        let b = v[2];
        Vector3::new(
            self.red_trc.apply_inverse(r),
            self.green_trc.apply_inverse(g),
            self.blue_trc.apply_inverse(b),
        )
    }
}
