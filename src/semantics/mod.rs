pub mod gamut;
pub mod material_you;

use serde::Deserialize;

use crate::colors::{Colors, Myrgb};

/// Supported semantic scheme kinds.
#[derive(Debug, Deserialize, Clone, Copy)]
pub enum SemanticSchemeKind {
    #[serde(rename = "material-you")]
    MaterialYou,
}

/// Whether to generate a dark or light semantic scheme.
#[derive(Debug, Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum SemanticStyle {
    #[default]
    Dark,
    Light,
}

/// Configuration for semantic color generation.
/// All fields are optional; the feature is disabled when `scheme` is `None`.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct SemanticConfig {
    pub scheme: Option<SemanticSchemeKind>,
    /// Palette slot to use as seed (e.g. "color4"). Defaults to "color4".
    pub seed: Option<String>,
    pub style: Option<SemanticStyle>,
}

/// The generated semantic color scheme.
#[derive(Debug, Clone)]
pub struct SemanticScheme {
    pub primary: Myrgb,
    pub on_primary: Myrgb,
    pub primary_container: Myrgb,
    pub on_primary_container: Myrgb,
    pub secondary: Myrgb,
    pub on_secondary: Myrgb,
    pub secondary_container: Myrgb,
    pub on_secondary_container: Myrgb,
    pub tertiary: Myrgb,
    pub on_tertiary: Myrgb,
    pub tertiary_container: Myrgb,
    pub on_tertiary_container: Myrgb,
    pub background: Myrgb,
    pub on_background: Myrgb,
    pub surface: Myrgb,
    pub on_surface: Myrgb,
    pub surface_variant: Myrgb,
    pub on_surface_variant: Myrgb,
    pub outline: Myrgb,
    pub outline_variant: Myrgb,

    pub error: Myrgb,
    pub on_error: Myrgb,
    pub error_container: Myrgb,
    pub on_error_container: Myrgb,

    pub surface_dim: Myrgb,
    pub surface_bright: Myrgb,
    pub surface_container_lowest: Myrgb,
    pub surface_container_low: Myrgb,
    pub surface_container: Myrgb,
    pub surface_container_high: Myrgb,
    pub surface_container_highest: Myrgb,

    pub inverse_surface: Myrgb,
    pub inverse_on_surface: Myrgb,
    pub inverse_primary: Myrgb,

    pub primary_fixed: Myrgb,
    pub primary_fixed_dim: Myrgb,
    pub on_primary_fixed: Myrgb,
    pub on_primary_fixed_variant: Myrgb,
    pub secondary_fixed: Myrgb,
    pub secondary_fixed_dim: Myrgb,
    pub on_secondary_fixed: Myrgb,
    pub on_secondary_fixed_variant: Myrgb,
    pub tertiary_fixed: Myrgb,
    pub tertiary_fixed_dim: Myrgb,
    pub on_tertiary_fixed: Myrgb,
    pub on_tertiary_fixed_variant: Myrgb,

    pub scrim: Myrgb,
    pub surface_tint: Myrgb,
}

/// Resolve a palette slot name to a `Myrgb` from the `Colors` struct.
pub fn resolve_seed(slot: &str, colors: &Colors) -> Option<Myrgb> {
    match slot {
        "color0" => Some(colors.color0),
        "color1" => Some(colors.color1),
        "color2" => Some(colors.color2),
        "color3" => Some(colors.color3),
        "color4" => Some(colors.color4),
        "color5" => Some(colors.color5),
        "color6" => Some(colors.color6),
        "color7" => Some(colors.color7),
        "color8" => Some(colors.color8),
        "color9" => Some(colors.color9),
        "color10" => Some(colors.color10),
        "color11" => Some(colors.color11),
        "color12" => Some(colors.color12),
        "color13" => Some(colors.color13),
        "color14" => Some(colors.color14),
        "color15" => Some(colors.color15),
        "background" => Some(colors.background),
        "foreground" => Some(colors.foreground),
        _ => None,
    }
}

/// Main entry point: generate a semantic scheme from config and palette.
/// Returns `None` if semantics are disabled (no scheme configured).
pub fn generate(config: Option<&SemanticConfig>, colors: &Colors) -> Option<SemanticScheme> {
    let config = config?;
    let scheme_kind = config.scheme?;
    let slot = config.seed.as_deref().unwrap_or("color4");
    let style = config.style.unwrap_or_default();
    let seed = resolve_seed(slot, colors)?;

    match scheme_kind {
        SemanticSchemeKind::MaterialYou => Some(material_you::generate(seed, style)),
    }
}
