//! Utility functions for converting between encodings of colors agnostic of the color space.
use std::fmt::Debug;

use nalgebra::Vector3;

use super::errors::Error;
use super::errors::Error::HexadecimalParseError;
use super::nalgebra_helpers::Vector3OptionExt;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Real(f64);

/// An encoding of a Real number in a byte (256 discrete values).
///
/// The encoding is such that Byte(0) = Real(0.0) and Byte(255) = Real(1.0)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Byte(u8);

impl Byte {
    fn to_real(self) -> Real {
        let Byte(x) = self;
        Real((x as f64) / 255.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct HexByte(char, char); // Big-endian; e.g. HexByte('1', '0') == 16

impl HexByte {
    fn to_byte(self) -> Option<Byte> {
        let Self(c0, c1) = self;
        let i0 = c0.to_digit(16)?;
        let i1 = c1.to_digit(16)?;
        Some(Byte((i0 * 16 + i1) as u8))
    }
}

fn real_to_byte(real: Real) -> Byte {
    let Real(raw_value) = real;
    let byte = (raw_value * 255.0).round() as i32;
    let clamped_byte = if byte < 0 {
        0
    } else if byte > 255 {
        255
    } else {
        byte as u8
    };
    Byte(clamped_byte)
}

pub fn parse_hex_string(hex_string: &str) -> Result<Vector3<f64>, Error> {
    let chars: Vec<char> = hex_string.chars().collect();
    let vec_hex_bytes = if chars.len() == 6 {
        Vector3::new(
            HexByte(chars[0], chars[1]),
            HexByte(chars[2], chars[3]),
            HexByte(chars[4], chars[5]),
        )
    } else if chars.len() == 3 {
        // Three-digit shorthand notation
        // See https://www.w3.org/TR/css-color-3/#rgb-color
        Vector3::new(
            HexByte(chars[0], chars[0]),
            HexByte(chars[1], chars[1]),
            HexByte(chars[2], chars[2]),
        )
    } else {
        return Err(HexadecimalParseError(format!(
            "A web hexadecimal color must be either 6 or 3 digits long, not {}",
            chars.len()
        )));
    };

    let vec_opt_bytes = vec_hex_bytes.map(|x| x.to_byte());
    let vec_bytes = vec_opt_bytes.sequence_option().ok_or_else(|| {
        HexadecimalParseError("A hexadecimal color may only contain hexadecimal digits".to_string())
    })?;
    Ok(vec_bytes.map(|b| b.to_real().0))
}

pub fn to_hex_string(vec: Vector3<f64>) -> String {
    let byte_vec = vec.map(Real).map(real_to_byte).map(|x| x.0);
    format!(
        "{:0>2x}{:0>2x}{:0>2x}",
        byte_vec[0], byte_vec[1], byte_vec[2]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn test_real_to_byte_0() {
        assert_eq!(real_to_byte(Real(0.0)), Byte(0))
    }

    #[test]
    fn test_real_to_byte_1() {
        assert_eq!(real_to_byte(Real(1.0)), Byte(255))
    }

    #[test]
    fn test_real_to_byte_rounding() {
        assert_eq!(real_to_byte(Real(0.9999)), Byte(255))
    }

    #[test]
    fn test_parse_hex_string_success() {
        assert_eq!(
            parse_hex_string("ff0080"),
            Ok(Vector3::new(1.0, 0.0, (0x80 as f64) / 255.0,))
        )
    }

    #[test]
    fn test_parse_hex_string_shorthand() {
        assert_eq!(
            parse_hex_string("123"),
            Ok(Vector3::new(
                (0x11 as f64) / 255.0,
                (0x22 as f64) / 255.0,
                (0x33 as f64) / 255.0
            ))
        )
    }

    #[test]
    fn test_parse_hex_string_invalid_length() {
        assert!(parse_hex_string("1234").is_err())
    }

    #[test]
    fn test_parse_hex_string_invalid_char() {
        assert!(parse_hex_string("xyz").is_err())
    }

    #[test]
    fn test_to_hex_string_success_light() {
        assert_eq!(to_hex_string(Vector3::new(1.0, 1.0, 1.0)), "ffffff")
    }

    #[test]
    fn test_to_hex_string_success_dark() {
        assert_eq!(to_hex_string(Vector3::new(0.0, 0.0, 0.0)), "000000")
    }

    #[test]
    fn test_from_and_to_hex_string() -> Result<(), Error> {
        use rand::{Rng, SeedableRng};
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(1337);

        for _ in 0..100 {
            let string = &format!("{:0>6x}", rng.gen::<u64>())[0..6];
            assert_eq!(
                to_hex_string(parse_hex_string(&string)?),
                string.to_string()
            );
        }
        Ok(())
    }
}
