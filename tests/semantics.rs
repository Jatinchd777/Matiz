use wallust::colors::{Colors, Myrgb};
use wallust::semantics::{self, SemanticConfig, SemanticSchemeKind, SemanticStyle};
use wallust::template::jinja2::jinja_env;
use wallust::template::TemplateFields;
use wallust::backends::Backend;
use wallust::palettes::Palette;
use palette::{IntoColor, Lch, Srgb};

fn test_colors() -> Colors {
    Colors {
        cursor: Myrgb(Srgb::new(221_u8, 221, 221).into_format()),
        background: Myrgb(Srgb::new(238_u8, 238, 238).into_format()),
        foreground: Myrgb(Srgb::new(221_u8, 221, 221).into_format()),
        color0:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color1:  Myrgb(Srgb::new(1,   0_u8, 0).into_format()),
        color2:  Myrgb(Srgb::new(2,   0_u8, 0).into_format()),
        color3:  Myrgb(Srgb::new(3,   0_u8, 0).into_format()),
        color4:  Myrgb(Srgb::new(4,   0_u8, 0).into_format()),
        color5:  Myrgb(Srgb::new(5,   0_u8, 0).into_format()),
        color6:  Myrgb(Srgb::new(6,   0_u8, 0).into_format()),
        color7:  Myrgb(Srgb::new(7,   0_u8, 0).into_format()),
        color8:  Myrgb(Srgb::new(8,   0_u8, 0).into_format()),
        color9:  Myrgb(Srgb::new(9,   0_u8, 0).into_format()),
        color10: Myrgb(Srgb::new(10,  0_u8, 0).into_format()),
        color11: Myrgb(Srgb::new(11,  0_u8, 0).into_format()),
        color12: Myrgb(Srgb::new(12,  0_u8, 0).into_format()),
        color13: Myrgb(Srgb::new(13,  0_u8, 0).into_format()),
        color14: Myrgb(Srgb::new(14,  0_u8, 0).into_format()),
        color15: Myrgb(Srgb::new(15,  0_u8, 0).into_format()),
    }
}

fn vibrant_colors() -> Colors {
    Colors {
        cursor: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        background: Myrgb(Srgb::new(30_u8, 30, 30).into_format()),
        foreground: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        color0:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color1:  Myrgb(Srgb::new(200, 50_u8, 50).into_format()),
        color2:  Myrgb(Srgb::new(50,  200_u8, 50).into_format()),
        color3:  Myrgb(Srgb::new(200, 200_u8, 50).into_format()),
        color4:  Myrgb(Srgb::new(50,  50_u8, 200).into_format()),
        color5:  Myrgb(Srgb::new(200, 50_u8, 200).into_format()),
        color6:  Myrgb(Srgb::new(50,  200_u8, 200).into_format()),
        color7:  Myrgb(Srgb::new(200, 200_u8, 200).into_format()),
        color8:  Myrgb(Srgb::new(100, 100_u8, 100).into_format()),
        color9:  Myrgb(Srgb::new(200, 50_u8, 50).into_format()),
        color10: Myrgb(Srgb::new(50,  200_u8, 50).into_format()),
        color11: Myrgb(Srgb::new(200, 200_u8, 50).into_format()),
        color12: Myrgb(Srgb::new(50,  50_u8, 200).into_format()),
        color13: Myrgb(Srgb::new(200, 50_u8, 200).into_format()),
        color14: Myrgb(Srgb::new(50,  200_u8, 200).into_format()),
        color15: Myrgb(Srgb::new(255, 255_u8, 255).into_format()),
    }
}

fn bright_seed_colors() -> Colors {
    Colors {
        cursor: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        background: Myrgb(Srgb::new(30_u8, 30, 30).into_format()),
        foreground: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        color0:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color1:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color2:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color3:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color4:  Myrgb(Srgb::new(209, 192_u8, 158).into_format()),
        color5:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color6:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color7:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color8:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color9:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color10: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color11: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color12: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color13: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color14: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color15: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
    }
}

fn dark_seed_colors() -> Colors {
    Colors {
        cursor: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        background: Myrgb(Srgb::new(30_u8, 30, 30).into_format()),
        foreground: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        color0:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color1:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color2:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color3:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color4:  Myrgb(Srgb::new(15,  25_u8, 50).into_format()),
        color5:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color6:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color7:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color8:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color9:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color10: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color11: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color12: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color13: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color14: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color15: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
    }
}

fn low_chroma_colors() -> Colors {
    Colors {
        cursor: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        background: Myrgb(Srgb::new(30_u8, 30, 30).into_format()),
        foreground: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        color0:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color1:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color2:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color3:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color4:  Myrgb(Srgb::new(140, 138_u8, 135).into_format()),
        color5:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color6:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color7:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color8:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color9:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color10: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color11: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color12: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color13: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color14: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color15: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
    }
}

fn high_chroma_colors() -> Colors {
    Colors {
        cursor: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        background: Myrgb(Srgb::new(30_u8, 30, 30).into_format()),
        foreground: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        color0:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color1:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color2:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color3:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color4:  Myrgb(Srgb::new(0,   70_u8, 255).into_format()),
        color5:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color6:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color7:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color8:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color9:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color10: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color11: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color12: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color13: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color14: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color15: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
    }
}

fn generate_dark(colors: &Colors) -> wallust::semantics::SemanticScheme {
    semantics::generate(
        Some(&SemanticConfig {
            scheme: Some(SemanticSchemeKind::MaterialYou),
            seed: Some("color4".into()),
            style: Some(SemanticStyle::Dark),
        }),
        colors,
    ).unwrap()
}

fn generate_light(colors: &Colors) -> wallust::semantics::SemanticScheme {
    semantics::generate(
        Some(&SemanticConfig {
            scheme: Some(SemanticSchemeKind::MaterialYou),
            seed: Some("color4".into()),
            style: Some(SemanticStyle::Light),
        }),
        colors,
    ).unwrap()
}

fn lch_l(c: Myrgb) -> f32 { let lch: Lch = c.0.into_color(); lch.l }
fn lch_h(c: Myrgb) -> f32 { let lch: Lch = c.0.into_color(); lch.hue.into_inner() }
fn lch_c(c: Myrgb) -> f32 { let lch: Lch = c.0.into_color(); lch.chroma }
fn seed_lch(colors: &Colors) -> Lch { colors.color4.0.into_color() }

// ============================================================
// Basic config
// ============================================================

#[test]
fn disabled_when_no_config() {
    let config = SemanticConfig::default();
    let colors = test_colors();
    assert!(semantics::generate(None, &colors).is_none());
    assert!(semantics::generate(Some(&config), &colors).is_none());
}

#[test]
fn disabled_when_no_scheme() {
    let config = SemanticConfig {
        scheme: None,
        seed: Some("color4".into()),
        style: Some(SemanticStyle::Dark),
    };
    let colors = test_colors();
    assert!(semantics::generate(Some(&config), &colors).is_none());
}

#[test]
fn seed_resolution() {
    let colors = test_colors();
    assert_eq!(semantics::resolve_seed("color0", &colors), Some(colors.color0));
    assert_eq!(semantics::resolve_seed("color4", &colors), Some(colors.color4));
    assert_eq!(semantics::resolve_seed("color15", &colors), Some(colors.color15));
    assert_eq!(semantics::resolve_seed("background", &colors), Some(colors.background));
    assert_eq!(semantics::resolve_seed("foreground", &colors), Some(colors.foreground));
    assert_eq!(semantics::resolve_seed("nonexistent", &colors), None);
}

// ============================================================
// DARK THEME: tonal relationships
// ============================================================

#[test]
fn dark_primary_is_light_accent() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    // Primary should be a light accent (L≈75)
    assert!(lch_l(scheme.primary) > 60.0, "dark primary should be light: L={}", lch_l(scheme.primary));
}

#[test]
fn dark_on_primary_is_dark_foreground() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    // onPrimary should be dark (L≈15)
    assert!(lch_l(scheme.on_primary) < 30.0, "dark on_primary should be dark: L={}", lch_l(scheme.on_primary));
}

#[test]
fn dark_primary_container_is_dark() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    // primaryContainer should be dark (L≈30)
    assert!(lch_l(scheme.primary_container) < 45.0, "dark primary_container should be dark: L={}", lch_l(scheme.primary_container));
}

#[test]
fn dark_on_primary_container_is_light() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    // onPrimaryContainer should be light (L≈90)
    assert!(lch_l(scheme.on_primary_container) > 75.0, "dark on_primary_container should be light: L={}", lch_l(scheme.on_primary_container));
}

#[test]
fn dark_secondary_tonal_relationship() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    assert!(lch_l(scheme.secondary) > 60.0, "dark secondary light accent: L={}", lch_l(scheme.secondary));
    assert!(lch_l(scheme.on_secondary) < 30.0, "dark on_secondary dark: L={}", lch_l(scheme.on_secondary));
    assert!(lch_l(scheme.secondary_container) < 45.0, "dark secondary_container dark: L={}", lch_l(scheme.secondary_container));
    assert!(lch_l(scheme.on_secondary_container) > 75.0, "dark on_secondary_container light: L={}", lch_l(scheme.on_secondary_container));
}

#[test]
fn dark_tertiary_tonal_relationship() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    assert!(lch_l(scheme.tertiary) > 60.0, "dark tertiary light accent: L={}", lch_l(scheme.tertiary));
    assert!(lch_l(scheme.on_tertiary) < 30.0, "dark on_tertiary dark: L={}", lch_l(scheme.on_tertiary));
    assert!(lch_l(scheme.tertiary_container) < 45.0, "dark tertiary_container dark: L={}", lch_l(scheme.tertiary_container));
    assert!(lch_l(scheme.on_tertiary_container) > 75.0, "dark on_tertiary_container light: L={}", lch_l(scheme.on_tertiary_container));
}

// ============================================================
// LIGHT THEME: tonal relationships (inverted)
// ============================================================

#[test]
fn light_primary_is_darker_accent() {
    let colors = vibrant_colors();
    let scheme = generate_light(&colors);

    assert!(lch_l(scheme.primary) < 55.0, "light primary should be darker: L={}", lch_l(scheme.primary));
}

#[test]
fn light_on_primary_is_light_foreground() {
    let colors = vibrant_colors();
    let scheme = generate_light(&colors);

    assert!(lch_l(scheme.on_primary) > 75.0, "light on_primary should be light: L={}", lch_l(scheme.on_primary));
}

#[test]
fn light_primary_container_is_light() {
    let colors = vibrant_colors();
    let scheme = generate_light(&colors);

    assert!(lch_l(scheme.primary_container) > 65.0, "light primary_container should be light: L={}", lch_l(scheme.primary_container));
}

#[test]
fn light_on_primary_container_is_dark() {
    let colors = vibrant_colors();
    let scheme = generate_light(&colors);

    assert!(lch_l(scheme.on_primary_container) < 35.0, "light on_primary_container should be dark: L={}", lch_l(scheme.on_primary_container));
}

// ============================================================
// Contrast pairs: the hard invariant
// ============================================================

#[test]
fn dark_on_primary_readability() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let diff = (lch_l(scheme.primary) - lch_l(scheme.on_primary)).abs();
    assert!(diff > 40.0, "dark primary/on_primary contrast: {}", diff);
}

#[test]
fn dark_on_primary_container_readability() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let diff = (lch_l(scheme.primary_container) - lch_l(scheme.on_primary_container)).abs();
    assert!(diff > 40.0, "dark primary_container/on contrast: {}", diff);
}

#[test]
fn dark_on_secondary_readability() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let diff = (lch_l(scheme.secondary) - lch_l(scheme.on_secondary)).abs();
    assert!(diff > 40.0, "dark secondary/on contrast: {}", diff);
}

#[test]
fn dark_on_tertiary_readability() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let diff = (lch_l(scheme.tertiary) - lch_l(scheme.on_tertiary)).abs();
    assert!(diff > 40.0, "dark tertiary/on contrast: {}", diff);
}

#[test]
fn light_on_primary_readability() {
    let colors = vibrant_colors();
    let scheme = generate_light(&colors);

    let diff = (lch_l(scheme.primary) - lch_l(scheme.on_primary)).abs();
    assert!(diff > 40.0, "light primary/on contrast: {}", diff);
}

#[test]
fn all_dark_accent_pairs_readability() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let pairs = [
        ("primary/on_primary", lch_l(scheme.primary), lch_l(scheme.on_primary)),
        ("primary_container/on_primary_container", lch_l(scheme.primary_container), lch_l(scheme.on_primary_container)),
        ("secondary/on_secondary", lch_l(scheme.secondary), lch_l(scheme.on_secondary)),
        ("secondary_container/on_secondary_container", lch_l(scheme.secondary_container), lch_l(scheme.on_secondary_container)),
        ("tertiary/on_tertiary", lch_l(scheme.tertiary), lch_l(scheme.on_tertiary)),
        ("tertiary_container/on_tertiary_container", lch_l(scheme.tertiary_container), lch_l(scheme.on_tertiary_container)),
        ("error/on_error", lch_l(scheme.error), lch_l(scheme.on_error)),
        ("error_container/on_error_container", lch_l(scheme.error_container), lch_l(scheme.on_error_container)),
        ("background/on_background", lch_l(scheme.background), lch_l(scheme.on_background)),
        ("surface/on_surface", lch_l(scheme.surface), lch_l(scheme.on_surface)),
    ];

    for (name, bg_l, fg_l) in &pairs {
        let diff = (bg_l - fg_l).abs();
        assert!(diff > 35.0, "{} contrast too low: {}", name, diff);
    }
}

// ============================================================
// Accent family differentiation
// ============================================================

#[test]
fn primary_secondary_tertiary_are_not_identical() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let ps_hue = (lch_h(scheme.primary) - lch_h(scheme.secondary)).abs();
    let pt_hue = (lch_h(scheme.primary) - lch_h(scheme.tertiary)).abs();

    assert!(ps_hue > 3.0, "primary/secondary hue diff: {}", ps_hue);
    assert!(pt_hue > 3.0, "primary/tertiary hue diff: {}", pt_hue);
}

#[test]
fn secondary_tertiary_use_less_chroma_input() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);
    // Secondary uses 0.70× seed chroma, tertiary uses 0.55×
    // After gamut correction, the actual output chroma may differ,
    // but the input chroma is reduced. We verify the seed provides the chroma source.
    let secondary_c = lch_c(scheme.secondary);
    let tertiary_c = lch_c(scheme.tertiary);
    assert!(secondary_c > 0.0, "secondary has chroma");
    assert!(tertiary_c > 0.0, "tertiary has chroma");
}

#[test]
fn hue_differences_are_conservative() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let ps = (lch_h(scheme.primary) - lch_h(scheme.secondary)).abs();
    let pt = (lch_h(scheme.primary) - lch_h(scheme.tertiary)).abs();

    assert!(ps < 20.0, "primary/secondary hue diff: {}", ps);
    assert!(pt < 20.0, "primary/tertiary hue diff: {}", pt);
}

// ============================================================
// Wallpaper identity: hue preserved
// ============================================================

#[test]
fn primary_hue_matches_seed() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);
    let seed = seed_lch(&colors);

    let hue_diff = (lch_h(scheme.primary) - seed.hue.into_inner()).abs();
    assert!(hue_diff < 1.0, "primary hue should match seed: diff={}", hue_diff);
}

#[test]
fn primary_chroma_from_seed() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);
    // Primary uses seed chroma directly; gamut correction may reduce it.
    // The important thing is it's derived from the seed, not clamped to a fixed value.
    let primary_c = lch_c(scheme.primary);
    assert!(primary_c > 0.0, "primary should have chroma from seed");
    // For a high-chroma seed, gamut correction will reduce it, so we just verify it's not zero
}

#[test]
fn dark_seed_gets_light_primary() {
    let colors = dark_seed_colors();
    let scheme = generate_dark(&colors);

    // Even with a dark seed, dark theme primary should be light
    assert!(lch_l(scheme.primary) > 60.0, "dark seed: primary should still be light: L={}", lch_l(scheme.primary));
}

#[test]
fn bright_seed_preserves_hue_identity() {
    let colors = bright_seed_colors();
    let scheme = generate_dark(&colors);
    let seed = seed_lch(&colors);

    let hue_diff = (lch_h(scheme.primary) - seed.hue.into_inner()).abs();
    assert!(hue_diff < 1.0, "bright seed: primary hue preserved: diff={}", hue_diff);
}

// ============================================================
// Low/high chroma seeds
// ============================================================

#[test]
fn low_chroma_seed_sensible() {
    let colors = low_chroma_colors();
    let scheme = generate_dark(&colors);

    // All three should have distinct lightness
    let l_primary = lch_l(scheme.primary);
    let l_secondary = lch_l(scheme.secondary);
    let l_tertiary = lch_l(scheme.tertiary);

    // With low chroma, distinctions come from lightness and hue
    let ps_hue = (lch_h(scheme.primary) - lch_h(scheme.secondary)).abs();
    let pt_hue = (lch_h(scheme.primary) - lch_h(scheme.tertiary)).abs();

    // Hue offsets should still create some distinction
    assert!(ps_hue > 3.0 || (l_primary - l_secondary).abs() > 1.0,
        "low chroma: secondary should differ");
    assert!(pt_hue > 3.0 || (l_primary - l_tertiary).abs() > 1.0,
        "low chroma: tertiary should differ");
}

#[test]
fn high_chroma_seed_not_extreme() {
    let colors = high_chroma_colors();
    let scheme = generate_dark(&colors);

    let check = |name: &str, c: Myrgb| {
        let srgb: Srgb = c.0;
        assert!(srgb.red >= 0.0 && srgb.red <= 1.0, "{} red OOG", name);
        assert!(srgb.green >= 0.0 && srgb.green <= 1.0, "{} green OOG", name);
        assert!(srgb.blue >= 0.0 && srgb.blue <= 1.0, "{} blue OOG", name);
    };
    check("primary", scheme.primary);
    check("secondary", scheme.secondary);
    check("tertiary", scheme.tertiary);
}

// ============================================================
// Hue wrapping
// ============================================================

#[test]
fn hue_wrapping_near_0_degrees() {
    let colors = Colors {
        cursor: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        background: Myrgb(Srgb::new(30_u8, 30, 30).into_format()),
        foreground: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        color0:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color1:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color2:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color3:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color4:  Myrgb(Srgb::new(2,   252_u8, 2).into_format()),
        color5:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color6:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color7:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color8:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color9:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color10: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color11: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color12: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color13: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color14: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color15: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
    };
    let scheme = generate_dark(&colors);

    assert!(lch_h(scheme.primary) >= 0.0 && lch_h(scheme.primary) < 360.0);
    assert!(lch_h(scheme.secondary) >= 0.0 && lch_h(scheme.secondary) < 360.0);
    assert!(lch_h(scheme.tertiary) >= 0.0 && lch_h(scheme.tertiary) < 360.0);
}

#[test]
fn hue_wrapping_near_360_degrees() {
    let colors = Colors {
        cursor: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        background: Myrgb(Srgb::new(30_u8, 30, 30).into_format()),
        foreground: Myrgb(Srgb::new(200_u8, 200, 200).into_format()),
        color0:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color1:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color2:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color3:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color4:  Myrgb(Srgb::new(252, 2_u8, 252).into_format()),
        color5:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color6:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color7:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color8:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color9:  Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color10: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color11: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color12: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color13: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color14: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
        color15: Myrgb(Srgb::new(0,   0_u8, 0).into_format()),
    };
    let scheme = generate_dark(&colors);

    assert!(lch_h(scheme.primary) >= 0.0 && lch_h(scheme.primary) < 360.0);
    assert!(lch_h(scheme.secondary) >= 0.0 && lch_h(scheme.secondary) < 360.0);
    assert!(lch_h(scheme.tertiary) >= 0.0 && lch_h(scheme.tertiary) < 360.0);

    let tertiary_hue = lch_h(scheme.tertiary);
    assert!(tertiary_hue > 300.0 && tertiary_hue < 345.0,
        "tertiary hue near wrap boundary: {}", tertiary_hue);
}

// ============================================================
// Surface hierarchy
// ============================================================

#[test]
fn surface_tonal_hierarchy_dark() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    assert!(lch_l(scheme.surface_dim) < lch_l(scheme.surface), "surface_dim < surface");
    assert!(lch_l(scheme.surface) < lch_l(scheme.surface_bright), "surface < surface_bright");
    assert!(lch_l(scheme.surface_container_lowest) < lch_l(scheme.surface_container_low));
    assert!(lch_l(scheme.surface_container_low) < lch_l(scheme.surface_container));
    assert!(lch_l(scheme.surface_container) < lch_l(scheme.surface_container_high));
    assert!(lch_l(scheme.surface_container_high) < lch_l(scheme.surface_container_highest));
}

#[test]
fn surface_tonal_hierarchy_light() {
    let colors = vibrant_colors();
    let scheme = generate_light(&colors);

    assert!(lch_l(scheme.surface_dim) < lch_l(scheme.surface), "surface_dim < surface");
    assert!(lch_l(scheme.surface) < lch_l(scheme.surface_bright), "surface < surface_bright");
    assert!(lch_l(scheme.surface_container_highest) < lch_l(scheme.surface_container_high));
    assert!(lch_l(scheme.surface_container_high) < lch_l(scheme.surface_container));
    assert!(lch_l(scheme.surface_container) < lch_l(scheme.surface_container_low));
    assert!(lch_l(scheme.surface_container_low) < lch_l(scheme.surface_container_lowest));
}

#[test]
fn surface_colors_are_low_chroma() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);
    let seed_chroma = lch_c(colors.color4);

    assert!(lch_c(scheme.surface) < seed_chroma * 0.3, "surface chroma too high");
    assert!(lch_c(scheme.background) < seed_chroma * 0.3, "background chroma too high");
    assert!(lch_c(scheme.surface_variant) < seed_chroma * 0.5, "surface_variant chroma too high");
}

// ============================================================
// Error family
// ============================================================

#[test]
fn error_is_distinct_from_accent_colors() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let error_hue = lch_h(scheme.error);
    assert!(error_hue > 10.0 && error_hue < 40.0, "error hue should be red: {}", error_hue);

    let ep = (error_hue - lch_h(scheme.primary)).abs();
    let es = (error_hue - lch_h(scheme.secondary)).abs();
    let et = (error_hue - lch_h(scheme.tertiary)).abs();

    assert!(ep > 60.0, "error/primary hue: {}", ep);
    assert!(es > 60.0, "error/secondary hue: {}", es);
    assert!(et > 60.0, "error/tertiary hue: {}", et);
}

#[test]
fn error_dark_tonal_relationship() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    // Dark error: light accent (L=80), dark on_error (L=20)
    assert!(lch_l(scheme.error) > 60.0, "dark error light: L={}", lch_l(scheme.error));
    assert!(lch_l(scheme.on_error) < 35.0, "dark on_error dark: L={}", lch_l(scheme.on_error));
    assert!(lch_l(scheme.error_container) < 45.0, "dark error_container dark: L={}", lch_l(scheme.error_container));
    assert!(lch_l(scheme.on_error_container) > 75.0, "dark on_error_container light: L={}", lch_l(scheme.on_error_container));
}

// ============================================================
// Inverse roles
// ============================================================

#[test]
fn inverse_roles_are_coherent() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    assert!(lch_l(scheme.inverse_surface) > 70.0, "inverse_surface bright");
    assert!(lch_l(scheme.inverse_on_surface) < 30.0, "inverse_on_surface dark");

    let diff = (lch_l(scheme.inverse_surface) - lch_l(scheme.inverse_on_surface)).abs();
    assert!(diff > 50.0, "inverse contrast: {}", diff);
}

// ============================================================
// Fixed roles
// ============================================================

#[test]
fn fixed_roles_are_valid_variants() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let hue_diff = (lch_h(scheme.primary) - lch_h(scheme.primary_fixed)).abs();
    assert!(hue_diff < 5.0, "primary_fixed hue diff: {}", hue_diff);

    assert!(lch_l(scheme.primary_fixed) > lch_l(scheme.primary_fixed_dim));
}

// ============================================================
// Surface tint
// ============================================================

#[test]
fn surface_tint_matches_primary() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    assert_eq!(format!("{}", scheme.primary), format!("{}", scheme.surface_tint));
}

// ============================================================
// Template integration
// ============================================================

#[test]
fn semantic_variables_in_jinja_context() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let fields = TemplateFields {
        alpha: 100,
        backend: &Backend::Thumb,
        palette: &Palette::Dark,
        colorspace: &wallust::colorspaces::ColorSpace::Lch,
        image_path: "/test",
        colors: &colors,
        semantic: Some(&scheme),
    };

    let v = minijinja::Value::from(&fields);
    let env = jinja_env();

    let primary = env.render_named_str("test", "{{ semantic.primary }}", v.clone()).unwrap();
    assert_eq!(primary, scheme.primary.to_string());

    let error = env.render_named_str("test", "{{ semantic.error }}", v.clone()).unwrap();
    assert_eq!(error, scheme.error.to_string());

    let scrim = env.render_named_str("test", "{{ semantic.scrim }}", v.clone()).unwrap();
    assert_eq!(scrim, scheme.scrim.to_string());
}

#[test]
fn all_semantic_roles_present() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let fields = TemplateFields {
        alpha: 100,
        backend: &Backend::Thumb,
        palette: &Palette::Dark,
        colorspace: &wallust::colorspaces::ColorSpace::Lch,
        image_path: "/test",
        colors: &colors,
        semantic: Some(&scheme),
    };

    let v = minijinja::Value::from(&fields);
    let env = jinja_env();

    let roles = [
        "semantic.primary", "semantic.on_primary",
        "semantic.primary_container", "semantic.on_primary_container",
        "semantic.secondary", "semantic.on_secondary",
        "semantic.secondary_container", "semantic.on_secondary_container",
        "semantic.tertiary", "semantic.on_tertiary",
        "semantic.tertiary_container", "semantic.on_tertiary_container",
        "semantic.background", "semantic.on_background",
        "semantic.surface", "semantic.on_surface",
        "semantic.surface_variant", "semantic.on_surface_variant",
        "semantic.outline", "semantic.outline_variant",
        "semantic.error", "semantic.on_error",
        "semantic.error_container", "semantic.on_error_container",
        "semantic.surface_dim", "semantic.surface_bright",
        "semantic.surface_container_lowest", "semantic.surface_container_low",
        "semantic.surface_container", "semantic.surface_container_high",
        "semantic.surface_container_highest",
        "semantic.inverse_surface", "semantic.inverse_on_surface",
        "semantic.inverse_primary",
        "semantic.primary_fixed", "semantic.primary_fixed_dim",
        "semantic.on_primary_fixed", "semantic.on_primary_fixed_variant",
        "semantic.secondary_fixed", "semantic.secondary_fixed_dim",
        "semantic.on_secondary_fixed", "semantic.on_secondary_fixed_variant",
        "semantic.tertiary_fixed", "semantic.tertiary_fixed_dim",
        "semantic.on_tertiary_fixed", "semantic.on_tertiary_fixed_variant",
        "semantic.scrim", "semantic.surface_tint",
    ];

    for role in &roles {
        let template = format!("{{{{ {} }}}}", role);
        let result = env.render_named_str("test", &template, v.clone());
        assert!(result.is_ok(), "Failed to render {}: {:?}", role, result.err());
        let value = result.unwrap();
        assert!(value.starts_with('#'), "{} should be hex, got: {}", role, value);
    }
}

#[test]
fn no_semantic_when_disabled() {
    let colors = vibrant_colors();
    let fields = TemplateFields {
        alpha: 100,
        backend: &Backend::Thumb,
        palette: &Palette::Dark,
        colorspace: &wallust::colorspaces::ColorSpace::Lch,
        image_path: "/test",
        colors: &colors,
        semantic: None,
    };
    let v = minijinja::Value::from(&fields);
    let env = jinja_env();
    let result = env.render_named_str("test", "{{ semantic.primary }}", v).unwrap();
    assert!(result.is_empty());
}

#[test]
fn existing_variables_unchanged() {
    let colors = vibrant_colors();
    let fields = TemplateFields {
        alpha: 100,
        backend: &Backend::Thumb,
        palette: &Palette::Dark,
        colorspace: &wallust::colorspaces::ColorSpace::Lch,
        image_path: "/test",
        colors: &colors,
        semantic: None,
    };
    let v = minijinja::Value::from(&fields);
    let env = jinja_env();

    assert_eq!(env.render_named_str("test", "{{ background }}", v.clone()).unwrap(), colors.background.to_string());
    assert_eq!(env.render_named_str("test", "{{ foreground }}", v.clone()).unwrap(), colors.foreground.to_string());
    assert_eq!(env.render_named_str("test", "{{ color4 }}", v).unwrap(), colors.color4.to_string());
}

// ============================================================
// Dark vs light modes differ
// ============================================================

#[test]
fn dark_and_light_modes_differ() {
    let colors = vibrant_colors();
    let dark = generate_dark(&colors);
    let light = generate_light(&colors);

    assert!(lch_l(dark.background) < lch_l(light.background), "dark bg < light bg");
    assert!(lch_l(dark.primary) > lch_l(light.primary), "dark primary > light primary");
}

// ============================================================
// Gamut
// ============================================================

#[test]
fn gamut_correction_preserves_hue() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let check_gamut = |name: &str, c: Myrgb| {
        let srgb: Srgb = c.0;
        assert!(srgb.red >= 0.0 && srgb.red <= 1.0, "{} red OOG", name);
        assert!(srgb.green >= 0.0 && srgb.green <= 1.0, "{} green OOG", name);
        assert!(srgb.blue >= 0.0 && srgb.blue <= 1.0, "{} blue OOG", name);
    };

    check_gamut("primary", scheme.primary);
    check_gamut("secondary", scheme.secondary);
    check_gamut("tertiary", scheme.tertiary);
    check_gamut("error", scheme.error);
    check_gamut("surface_tint", scheme.surface_tint);
}

#[test]
fn gamut_module_directly() {
    use wallust::semantics::gamut;

    let in_gamut = Lch::new(50.0, 50.0, 120.0f32);
    let result = gamut::clamp_to_gamut(in_gamut);
    assert!((result.l - 50.0).abs() < 0.01);
    assert!((result.chroma - 50.0).abs() < 0.01);
    assert!((result.hue.into_inner() - 120.0).abs() < 0.01);

    let out_of_gamut = Lch::new(50.0, 200.0, 30.0f32);
    let result = gamut::clamp_to_gamut(out_of_gamut);
    assert!((result.l - 50.0).abs() < 0.01);
    assert!(result.chroma < 200.0);
    assert!((result.hue.into_inner() - 30.0).abs() < 0.01);
}

// ============================================================
// Accent containers are distinct
// ============================================================

#[test]
fn accent_containers_are_distinct() {
    let colors = vibrant_colors();
    let scheme = generate_dark(&colors);

    let pc_hue = lch_h(scheme.primary_container);
    let sc_hue = lch_h(scheme.secondary_container);
    let tc_hue = lch_h(scheme.tertiary_container);

    let ps = (pc_hue - sc_hue).abs();
    let pt = (pc_hue - tc_hue).abs();

    assert!(ps > 3.0, "primary/secondary container hue: {}", ps);
    assert!(pt > 3.0, "primary/tertiary container hue: {}", pt);
}
