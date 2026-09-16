use palette::convert::FromColorUnclamped;
use palette::{Lch, Srgb};

/// Check if an LCH color is within the sRGB gamut by converting to Srgb
/// using unclamped conversion and verifying all components are in [0, 1].
fn is_in_gamut(color: Lch) -> bool {
    let srgb: Srgb = Srgb::from_color_unclamped(color);
    srgb.red >= 0.0
        && srgb.red <= 1.0
        && srgb.green >= 0.0
        && srgb.green <= 1.0
        && srgb.blue >= 0.0
        && srgb.blue <= 1.0
}

/// Clamp an LCH color to sRGB gamut by reducing chroma while preserving
/// hue and lightness. Returns the corrected LCH color.
pub fn clamp_to_gamut(color: Lch) -> Lch {
    if is_in_gamut(color) {
        return color;
    }

    let l = color.l;
    let h = color.hue;
    let c = color.chroma;

    // Binary search for the maximum chroma that fits in gamut
    let mut lo = 0.0f32;
    let mut hi = c;

    for _ in 0..20 {
        let mid = (lo + hi) / 2.0;
        let test = Lch::new(l, mid, h);
        if is_in_gamut(test) {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    Lch::new(l, lo, h)
}
