use std::fmt::Debug;

use nalgebra::Vector3;

use super::ciexyz::CIEXYZ;
use crate::encodings::{parse_hex_string, to_hex_string};
use crate::errors::Error;

/// A representation of a color in an arbitrary color space.
///
/// Concrete implementations of Color are color spaces, while instances of Color are concrete
/// colors within a color space.
///
/// An instance of a Color must be convertible back and forth between its native color space and
/// the CIE 1931 XYZ color space.
///
/// Colors are typically a 3-tuple of numbers, e.g. RGB, XYZ, etc.
pub trait Color: Debug + Sized + Copy {
    /// Convert from a CIE 1931 XYZ color to a color in this color space.
    fn from_ciexyz(ciexyz: &CIEXYZ) -> Self;

    /// Convert this color to a CIE 1931 XYZ color.
    fn to_ciexyz(&self) -> CIEXYZ;

    /// Lift an arbitrary 3-tuple of numbers into a color within this color space.
    ///
    /// Useful for implementing mathematical transformations between two color spaces.
    fn from_vector3(vec: Vector3<f64>) -> Self;

    /// Return an arbitrary 3-tuple of numbers from a color within this color space.
    ///
    /// Useful for implementing mathematical transformations between two color spaces.
    fn to_vector3(&self) -> Vector3<f64>;

    /// Convert from any other Color to this color.
    fn from_color<C>(color: &C) -> Self
    where
        C: Color,
    {
        Self::from_ciexyz(&color.to_ciexyz())
    }

    /// Convert this color to any other color.
    fn to_color<C>(&self) -> C
    where
        C: Color,
    {
        Color::from_ciexyz(&self.to_ciexyz())
    }

    /// Interpret a hexadecimal color string in this color space.
    fn from_hex(hex: &str) -> Result<Self, Error> {
        let hex_vec = parse_hex_string(hex)?;
        Ok(Self::from_vector3(hex_vec))
    }

    /// Encode color as hexadecimal string
    fn to_hex(&self) -> Result<String, Error> {
        Ok(to_hex_string(self.to_vector3()))
    }
}
