use palette::{IntoColor, Lch};

use crate::colors::Myrgb;

use super::SemanticScheme;
use super::SemanticStyle;
use super::gamut;

fn wrap_hue(h: f32) -> f32 {
    h.rem_euclid(360.0)
}

fn from_lch(l: f32, c: f32, h: f32) -> Myrgb {
    let color = Lch::new(l.max(0.0).min(100.0), c.max(0.0), wrap_hue(h));
    let corrected = gamut::clamp_to_gamut(color);
    Myrgb(corrected.into_color())
}

fn ensure_contrast(bg: Myrgb, fg: Myrgb) -> Myrgb {
    use palette::color_difference::Wcag21RelativeContrast;
    if bg.0.has_min_contrast_text(fg.0) {
        return fg;
    }
    let bg_lch: Lch = bg.0.into_color();
    let fg_lch: Lch = fg.0.into_color();
    if bg_lch.l < 50.0 {
        from_lch(fg_lch.l.min(95.0), fg_lch.chroma, fg_lch.hue.into_inner())
    } else {
        from_lch(fg_lch.l.max(5.0), fg_lch.chroma, fg_lch.hue.into_inner())
    }
}

// ============================================================
// Surface hierarchy
// ============================================================

struct DarkSurfaces;
impl DarkSurfaces {
    const SURFACE_DIM: f32 = 6.0;
    const SURFACE: f32 = 12.0;
    const SURFACE_BRIGHT: f32 = 24.0;
    const SURFACE_CONTAINER_LOWEST: f32 = 4.0;
    const SURFACE_CONTAINER_LOW: f32 = 10.0;
    const SURFACE_CONTAINER: f32 = 17.0;
    const SURFACE_CONTAINER_HIGH: f32 = 22.0;
    const SURFACE_CONTAINER_HIGHEST: f32 = 27.0;
    const SURFACE_VARIANT: f32 = 22.0;
    const ON_SURFACE: f32 = 90.0;
    const ON_SURFACE_VARIANT: f32 = 80.0;
    const OUTLINE: f32 = 52.0;
    const OUTLINE_VARIANT: f32 = 35.0;
}

struct LightSurfaces;
impl LightSurfaces {
    const SURFACE_DIM: f32 = 87.0;
    const SURFACE: f32 = 95.0;
    const SURFACE_BRIGHT: f32 = 98.0;
    const SURFACE_CONTAINER_LOWEST: f32 = 100.0;
    const SURFACE_CONTAINER_LOW: f32 = 96.0;
    const SURFACE_CONTAINER: f32 = 93.0;
    const SURFACE_CONTAINER_HIGH: f32 = 92.0;
    const SURFACE_CONTAINER_HIGHEST: f32 = 90.0;
    const SURFACE_VARIANT: f32 = 85.0;
    const ON_SURFACE: f32 = 15.0;
    const ON_SURFACE_VARIANT: f32 = 25.0;
    const OUTLINE: f32 = 52.0;
    const OUTLINE_VARIANT: f32 = 65.0;
}

// ============================================================
// Error family
// ============================================================

const ERROR_HUE: f32 = 25.0;

struct DarkError;
impl DarkError {
    const ERROR: f32 = 80.0;
    const ON_ERROR: f32 = 20.0;
    const ERROR_CONTAINER: f32 = 30.0;
    const ON_ERROR_CONTAINER: f32 = 90.0;
}

struct LightError;
impl LightError {
    const ERROR: f32 = 40.0;
    const ON_ERROR: f32 = 90.0;
    const ERROR_CONTAINER: f32 = 80.0;
    const ON_ERROR_CONTAINER: f32 = 20.0;
}

// ============================================================
// Accent generation
//
// Dark theme model:
//   accent     = L=75  (light accent on dark background)
//   on_accent  = L=15  (dark foreground, readable on accent)
//   container  = L=30  (dark container)
//   on_cont    = L=90  (light foreground, readable on container)
//
// Light theme model:
//   accent     = L=40  (darker accent on light background)
//   on_accent  = L=90  (light foreground)
//   container  = L=80  (light container)
//   on_cont    = L=20  (dark foreground)
// ============================================================

pub fn generate(seed: Myrgb, style: SemanticStyle) -> SemanticScheme {
    let lch: Lch = seed.0.into_color();
    let h = lch.hue.into_inner();
    let c = lch.chroma;

    let surface_c = c * 0.10;

    match style {
        SemanticStyle::Dark => {
            // --- Primary family ---
            let primary_h = h;
            let primary_c = c;
            let primary = from_lch(75.0, primary_c, primary_h);
            let on_primary = from_lch(28.0, primary_c * 0.40, primary_h);
            let primary_container = from_lch(35.0, primary_c * 0.8, primary_h);
            let on_primary_container = from_lch(90.0, primary_c * 0.3, primary_h);

            // --- Secondary family ---
            let secondary_h = wrap_hue(h + 10.0);
            let secondary_c = c * 0.70;
            let secondary = from_lch(72.0, secondary_c, secondary_h);
            let on_secondary = from_lch(28.0, secondary_c * 0.40, secondary_h);
            let secondary_container = from_lch(30.0, secondary_c * 0.5, secondary_h);
            let on_secondary_container = from_lch(90.0, secondary_c * 0.3, secondary_h);

            // --- Tertiary family ---
            let tertiary_h = wrap_hue(h - 10.0);
            let tertiary_c = c * 0.55;
            let tertiary = from_lch(70.0, tertiary_c, tertiary_h);
            let on_tertiary = from_lch(30.0, tertiary_c * 0.40, tertiary_h);
            let tertiary_container = from_lch(30.0, tertiary_c * 0.5, tertiary_h);
            let on_tertiary_container = from_lch(90.0, tertiary_c * 0.3, tertiary_h);

            // --- Background ---
            let background = from_lch(10.0, surface_c, h);
            let on_background = from_lch(90.0, surface_c, h);

            // --- Surface family ---
            let surface_dim = from_lch(DarkSurfaces::SURFACE_DIM, surface_c, h);
            let surface = from_lch(DarkSurfaces::SURFACE, surface_c, h);
            let surface_bright = from_lch(DarkSurfaces::SURFACE_BRIGHT, surface_c, h);
            let surface_container_lowest = from_lch(DarkSurfaces::SURFACE_CONTAINER_LOWEST, surface_c * 0.5, h);
            let surface_container_low = from_lch(DarkSurfaces::SURFACE_CONTAINER_LOW, surface_c * 0.5, h);
            let surface_container = from_lch(DarkSurfaces::SURFACE_CONTAINER, surface_c, h);
            let surface_container_high = from_lch(DarkSurfaces::SURFACE_CONTAINER_HIGH, surface_c, h);
            let surface_container_highest = from_lch(DarkSurfaces::SURFACE_CONTAINER_HIGHEST, surface_c, h);
            let surface_variant = from_lch(DarkSurfaces::SURFACE_VARIANT, surface_c * 1.5, h);
            let on_surface = from_lch(DarkSurfaces::ON_SURFACE, surface_c, h);
            let on_surface_variant = from_lch(DarkSurfaces::ON_SURFACE_VARIANT, surface_c, h);
            let outline = from_lch(DarkSurfaces::OUTLINE, surface_c * 0.5, h);
            let outline_variant = from_lch(DarkSurfaces::OUTLINE_VARIANT, surface_c * 0.5, h);

            // --- Error family ---
            let error = from_lch(DarkError::ERROR, 60.0, ERROR_HUE);
            let on_error = from_lch(DarkError::ON_ERROR, 25.0, ERROR_HUE);
            let error_container = from_lch(DarkError::ERROR_CONTAINER, 55.0, ERROR_HUE);
            let on_error_container = from_lch(DarkError::ON_ERROR_CONTAINER, 20.0, ERROR_HUE);

            // --- Inverse ---
            let inverse_surface = from_lch(90.0, surface_c, h);
            let inverse_on_surface = from_lch(20.0, surface_c, h);
            let inverse_primary = from_lch(40.0, primary_c, primary_h);

            // --- Fixed accent ---
            let primary_fixed = from_lch(80.0, primary_c * 0.8, primary_h);
            let primary_fixed_dim = from_lch(70.0, primary_c * 0.6, primary_h);
            let on_primary_fixed = from_lch(10.0, primary_c * 0.3, primary_h);
            let on_primary_fixed_variant = from_lch(30.0, primary_c * 0.4, primary_h);

            let secondary_fixed = from_lch(80.0, secondary_c * 0.8, secondary_h);
            let secondary_fixed_dim = from_lch(70.0, secondary_c * 0.6, secondary_h);
            let on_secondary_fixed = from_lch(10.0, secondary_c * 0.3, secondary_h);
            let on_secondary_fixed_variant = from_lch(30.0, secondary_c * 0.4, secondary_h);

            let tertiary_fixed = from_lch(80.0, tertiary_c * 0.8, tertiary_h);
            let tertiary_fixed_dim = from_lch(70.0, tertiary_c * 0.6, tertiary_h);
            let on_tertiary_fixed = from_lch(10.0, tertiary_c * 0.3, tertiary_h);
            let on_tertiary_fixed_variant = from_lch(30.0, tertiary_c * 0.4, tertiary_h);

            let scrim = from_lch(0.0, 0.0, h);
            let surface_tint = primary;

            // Apply contrast safety net
            let on_primary = ensure_contrast(primary, on_primary);
            let on_primary_container = ensure_contrast(primary_container, on_primary_container);
            let on_secondary = ensure_contrast(secondary, on_secondary);
            let on_secondary_container = ensure_contrast(secondary_container, on_secondary_container);
            let on_tertiary = ensure_contrast(tertiary, on_tertiary);
            let on_tertiary_container = ensure_contrast(tertiary_container, on_tertiary_container);
            let on_error = ensure_contrast(error, on_error);
            let on_error_container = ensure_contrast(error_container, on_error_container);
            let on_background = ensure_contrast(background, on_background);
            let on_surface = ensure_contrast(surface, on_surface);
            let on_surface_variant = ensure_contrast(surface_variant, on_surface_variant);

            SemanticScheme {
                primary, on_primary, primary_container, on_primary_container,
                secondary, on_secondary, secondary_container, on_secondary_container,
                tertiary, on_tertiary, tertiary_container, on_tertiary_container,
                background, on_background,
                surface, on_surface, surface_variant, on_surface_variant,
                outline, outline_variant,
                error, on_error, error_container, on_error_container,
                surface_dim, surface_bright,
                surface_container_lowest, surface_container_low,
                surface_container, surface_container_high, surface_container_highest,
                inverse_surface, inverse_on_surface, inverse_primary,
                primary_fixed, primary_fixed_dim, on_primary_fixed, on_primary_fixed_variant,
                secondary_fixed, secondary_fixed_dim, on_secondary_fixed, on_secondary_fixed_variant,
                tertiary_fixed, tertiary_fixed_dim, on_tertiary_fixed, on_tertiary_fixed_variant,
                scrim, surface_tint,
            }
        }
        SemanticStyle::Light => {
            // --- Primary family ---
            let primary_h = h;
            let primary_c = c;
            let primary = from_lch(40.0, primary_c, primary_h);
            let on_primary = from_lch(90.0, primary_c * 0.30, primary_h);
            let primary_container = from_lch(80.0, primary_c * 0.6, primary_h);
            let on_primary_container = from_lch(20.0, primary_c * 0.3, primary_h);

            // --- Secondary family ---
            let secondary_h = wrap_hue(h + 10.0);
            let secondary_c = c * 0.70;
            let secondary = from_lch(38.0, secondary_c, secondary_h);
            let on_secondary = from_lch(90.0, secondary_c * 0.30, secondary_h);
            let secondary_container = from_lch(80.0, secondary_c * 0.5, secondary_h);
            let on_secondary_container = from_lch(20.0, secondary_c * 0.3, secondary_h);

            // --- Tertiary family ---
            let tertiary_h = wrap_hue(h - 10.0);
            let tertiary_c = c * 0.55;
            let tertiary = from_lch(36.0, tertiary_c, tertiary_h);
            let on_tertiary = from_lch(90.0, tertiary_c * 0.30, tertiary_h);
            let tertiary_container = from_lch(80.0, tertiary_c * 0.5, tertiary_h);
            let on_tertiary_container = from_lch(20.0, tertiary_c * 0.3, tertiary_h);

            // --- Background ---
            let background = from_lch(95.0, surface_c, h);
            let on_background = from_lch(15.0, surface_c, h);

            // --- Surface family ---
            let surface_dim = from_lch(LightSurfaces::SURFACE_DIM, surface_c, h);
            let surface = from_lch(LightSurfaces::SURFACE, surface_c, h);
            let surface_bright = from_lch(LightSurfaces::SURFACE_BRIGHT, surface_c, h);
            let surface_container_lowest = from_lch(LightSurfaces::SURFACE_CONTAINER_LOWEST, surface_c * 0.5, h);
            let surface_container_low = from_lch(LightSurfaces::SURFACE_CONTAINER_LOW, surface_c * 0.5, h);
            let surface_container = from_lch(LightSurfaces::SURFACE_CONTAINER, surface_c, h);
            let surface_container_high = from_lch(LightSurfaces::SURFACE_CONTAINER_HIGH, surface_c, h);
            let surface_container_highest = from_lch(LightSurfaces::SURFACE_CONTAINER_HIGHEST, surface_c, h);
            let surface_variant = from_lch(LightSurfaces::SURFACE_VARIANT, surface_c * 1.5, h);
            let on_surface = from_lch(LightSurfaces::ON_SURFACE, surface_c, h);
            let on_surface_variant = from_lch(LightSurfaces::ON_SURFACE_VARIANT, surface_c, h);
            let outline = from_lch(LightSurfaces::OUTLINE, surface_c * 0.5, h);
            let outline_variant = from_lch(LightSurfaces::OUTLINE_VARIANT, surface_c * 0.5, h);

            // --- Error family ---
            let error = from_lch(LightError::ERROR, 60.0, ERROR_HUE);
            let on_error = from_lch(LightError::ON_ERROR, 25.0, ERROR_HUE);
            let error_container = from_lch(LightError::ERROR_CONTAINER, 55.0, ERROR_HUE);
            let on_error_container = from_lch(LightError::ON_ERROR_CONTAINER, 20.0, ERROR_HUE);

            let inverse_surface = from_lch(20.0, surface_c, h);
            let inverse_on_surface = from_lch(90.0, surface_c, h);
            let inverse_primary = from_lch(60.0, primary_c, primary_h);

            let primary_fixed = from_lch(80.0, primary_c * 0.8, primary_h);
            let primary_fixed_dim = from_lch(70.0, primary_c * 0.6, primary_h);
            let on_primary_fixed = from_lch(10.0, primary_c * 0.3, primary_h);
            let on_primary_fixed_variant = from_lch(30.0, primary_c * 0.4, primary_h);

            let secondary_fixed = from_lch(80.0, secondary_c * 0.8, secondary_h);
            let secondary_fixed_dim = from_lch(70.0, secondary_c * 0.6, secondary_h);
            let on_secondary_fixed = from_lch(10.0, secondary_c * 0.3, secondary_h);
            let on_secondary_fixed_variant = from_lch(30.0, secondary_c * 0.4, secondary_h);

            let tertiary_fixed = from_lch(80.0, tertiary_c * 0.8, tertiary_h);
            let tertiary_fixed_dim = from_lch(70.0, tertiary_c * 0.6, tertiary_h);
            let on_tertiary_fixed = from_lch(10.0, tertiary_c * 0.3, tertiary_h);
            let on_tertiary_fixed_variant = from_lch(30.0, tertiary_c * 0.4, tertiary_h);

            let scrim = from_lch(0.0, 0.0, h);
            let surface_tint = primary;

            let on_primary = ensure_contrast(primary, on_primary);
            let on_primary_container = ensure_contrast(primary_container, on_primary_container);
            let on_secondary = ensure_contrast(secondary, on_secondary);
            let on_secondary_container = ensure_contrast(secondary_container, on_secondary_container);
            let on_tertiary = ensure_contrast(tertiary, on_tertiary);
            let on_tertiary_container = ensure_contrast(tertiary_container, on_tertiary_container);
            let on_error = ensure_contrast(error, on_error);
            let on_error_container = ensure_contrast(error_container, on_error_container);
            let on_background = ensure_contrast(background, on_background);
            let on_surface = ensure_contrast(surface, on_surface);
            let on_surface_variant = ensure_contrast(surface_variant, on_surface_variant);

            SemanticScheme {
                primary, on_primary, primary_container, on_primary_container,
                secondary, on_secondary, secondary_container, on_secondary_container,
                tertiary, on_tertiary, tertiary_container, on_tertiary_container,
                background, on_background,
                surface, on_surface, surface_variant, on_surface_variant,
                outline, outline_variant,
                error, on_error, error_container, on_error_container,
                surface_dim, surface_bright,
                surface_container_lowest, surface_container_low,
                surface_container, surface_container_high, surface_container_highest,
                inverse_surface, inverse_on_surface, inverse_primary,
                primary_fixed, primary_fixed_dim, on_primary_fixed, on_primary_fixed_variant,
                secondary_fixed, secondary_fixed_dim, on_secondary_fixed, on_secondary_fixed_variant,
                tertiary_fixed, tertiary_fixed_dim, on_tertiary_fixed, on_tertiary_fixed_variant,
                scrim, surface_tint,
            }
        }
    }
}
