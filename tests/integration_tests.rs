extern crate color_schemes;

use color_schemes::colors::{get_d65_ciexyz, Color, DisplayP3, CIEXYY, CIEXYZ, SRGB};
use color_schemes::errors::Error;
use color_schemes::utils::contrast_ratio;
use nalgebra::Vector3;

macro_rules! assert_within_delta {
    ( $val1: expr, $val2: expr, $delta: expr ) => {
        assert!(
            ($val1 - $val2).abs() < $delta,
            "Expected {} to be within {} of {}",
            $val1,
            $delta,
            $val2
        );
    };
}

#[test]
fn test_convert_srgb_to_ciexyy() -> Result<(), Error> {
    let srgb = SRGB::from_hex("ff0000")?;
    let ciexyy: CIEXYY = srgb.to_color();
    let vec = ciexyy.to_vector3();
    let expected = Vector3::new(0.64, 0.33, 0.2126);
    assert_within_delta!((vec - expected).magnitude(), 0.0, 0.0001);
    Ok(())
}

#[test]
fn test_srgb_white() -> Result<(), Error> {
    let white: SRGB = get_d65_ciexyz().to_color();
    assert_eq!(white.to_hex()?, "ffffff");
    Ok(())
}

#[test]
fn test_contrast_ratio() -> Result<(), Error> {
    let color1 = SRGB::from_hex("996633")?;
    let color2 = SRGB::from_hex("0000ff")?;
    let ratio = contrast_ratio(color1, color2);
    assert_within_delta!(ratio, 1.7610, 0.0001);
    Ok(())
}

#[test]
fn test_ciexyz_to_ciexyz_is_identical() {
    let color1 = CIEXYZ::new(0.123, 0.456, 0.789);
    let color2 = color1.to_ciexyz();
    assert_eq!(color1.to_vector3(), color2.to_vector3());
}

#[test]
fn test_display_p3_white() -> Result<(), Error> {
    // Compare Display P3 white against known chromaticity
    // https://en.wikipedia.org/wiki/DCI-P3#Display_P3
    let display_p3_white = DisplayP3::from_hex("fff")?;
    let display_p3_white_in_ciexyz = display_p3_white.to_ciexyz();
    let display_p3_white_in_ciexyy: CIEXYY = display_p3_white_in_ciexyz.to_color();
    let vec = display_p3_white_in_ciexyy.to_vector3();
    let x = vec[0];
    let y = vec[1];
    assert_within_delta!(x, 0.3127, 0.0001);
    assert_within_delta!(y, 0.3290, 0.0001);

    Ok(())
}

#[test]
fn test_display_p3_green() -> Result<(), Error> {
    // Compare Display P3 green against known chromaticity
    // https://en.wikipedia.org/wiki/DCI-P3#Display_P3
    let display_p3_green = DisplayP3::from_hex("0f0")?;
    let display_p3_green_in_ciexyz = display_p3_green.to_ciexyz();
    let display_p3_green_in_ciexyy: CIEXYY = display_p3_green_in_ciexyz.to_color();
    let vec = display_p3_green_in_ciexyy.to_vector3();
    let x = vec[0];
    let y = vec[1];
    assert_within_delta!(x, 0.265, 0.0001);
    assert_within_delta!(y, 0.690, 0.0001);

    Ok(())
}
