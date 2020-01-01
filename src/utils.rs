use super::colors::Color;

fn contrast_ratio_luminance(lighter: f64, darker: f64) -> f64 {
    (lighter + 0.05) / (darker + 0.05)
}

pub fn contrast_ratio<C>(c1: C, c2: C) -> f64
where
    C: Color,
{
    let xyz1 = c1.to_ciexyz();
    let xyz2 = c2.to_ciexyz();

    let lum1 = xyz1.get_luminance();
    let lum2 = xyz2.get_luminance();

    let (lighter, darker) = if lum1 > lum2 {
        (lum1, lum2)
    } else {
        (lum2, lum1)
    };

    contrast_ratio_luminance(lighter, darker)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::CIEXYZ;

    #[test]
    fn test_contrast_ratio_luminance_equal() {
        assert_eq!(contrast_ratio_luminance(0.1, 0.1), 1.0)
    }

    #[test]
    fn test_contrast_ratio_luminance_extreme() {
        assert_eq!(contrast_ratio_luminance(1.0, 0.0), 21.0)
    }

    #[test]
    fn test_contrast_ratio_success() {
        let white = CIEXYZ::new(0.95047, 1.0, 1.08883);
        let black = CIEXYZ::new(0.0, 0.0, 0.0);
        assert_eq!(contrast_ratio(white, black), 21.0);
    }

    #[test]
    fn test_contrast_ratio_success_flipped() {
        let white = CIEXYZ::new(0.95047, 1.0, 1.08883);
        let black = CIEXYZ::new(0.0, 0.0, 0.0);
        assert_eq!(contrast_ratio(black, white), 21.0);
    }
}
