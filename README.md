# Matiz

> Wallpaper-derived semantic colors for Linux.

Matiz is a wallpaper-driven color system based on Wallust v3.5.2, extended with semantic color generation using perceptual color transformations.

It preserves Wallust's existing palette workflow while adding a structured semantic color layer for desktop shells, applications, widgets, and other UI components.

**Wallpaper provides the personality. Semantics provide the structure.**

## What is Matiz?

Traditional wallpaper theming is good at extracting colors, but applications often need more than a collection of palette indexes.

A UI needs to know things like:

- Which color is the primary action?
- Which color belongs on top of it?
- Which color should be used for containers?
- Which colors represent surfaces?
- Which color should be used for outlines?
- Which color represents an error?

Matiz adds those relationships while keeping the colors derived from the wallpaper.

The result is a theme that can adapt to the wallpaper without forcing every wallpaper into the same visual appearance.

## Philosophy

Matiz is built around a simple idea:

> **Keep the identity of the source colors, then give them meaning.**

A colorful wallpaper can produce colorful semantic colors.

A muted wallpaper can produce muted semantic colors.

A mostly white or neutral wallpaper can naturally produce neutral colors.

Matiz does not attempt to turn every wallpaper into the same predefined aesthetic.

The wallpaper remains responsible for the character of the theme, while semantic roles determine how that character is used throughout the UI.

## How it works

```text
Wallpaper
    │
    ▼
Wallust palette extraction
    │
    ▼
LAB palette
    │
    ▼
Wallust palette color
    │
    ▼
LAB → LCH
    │
    ▼
Semantic transformations
    │
    ▼
Semantic roles
    │
    ▼
Shells / applications / UI
```

Wallust's palette generation and Matiz's semantic generation are separate layers.

The traditional Wallust palette remains available for existing templates and terminal colors, while the semantic engine derives additional colors for structured UI theming.

## Color generation

The semantic engine uses LCH for its color transformations.

LCH provides separate control over:

- Lightness
- Chroma
- Hue

This allows semantic roles to be derived from the extracted colors while maintaining relationships with the source color.

For example, a primary color can retain its source hue while its semantic variants adjust lightness and chroma according to their intended role.

The goal is not to replace the extracted palette.

The goal is to derive a structured semantic system from it.

## Semantic roles

Matiz provides a set of semantic roles for applications and desktop shells.

### Primary

```text
primary
on_primary
primary_container
on_primary_container
```

### Secondary

```text
secondary
on_secondary
secondary_container
on_secondary_container
```

### Tertiary

```text
tertiary
on_tertiary
tertiary_container
on_tertiary_container
```

### Error

```text
error
on_error
error_container
on_error_container
```

### Background and surfaces

```text
background
on_background

surface
on_surface

surface_variant
on_surface_variant

surface_dim
surface_bright

surface_container_lowest
surface_container_low
surface_container
surface_container_high
surface_container_highest
```

### Additional roles

```text
surface_tint

outline
outline_variant

inverse_surface
inverse_on_surface
inverse_primary

scrim
```

### Fixed roles

```text
primary_fixed
primary_fixed_dim
on_primary_fixed
on_primary_fixed_variant

secondary_fixed
secondary_fixed_dim
on_secondary_fixed
on_secondary_fixed_variant

tertiary_fixed
tertiary_fixed_dim
on_tertiary_fixed
on_tertiary_fixed_variant
```

These roles allow applications to consume colors according to their purpose rather than relying exclusively on arbitrary palette indexes.

## Templates

Semantic colors are available through the `semantic` template namespace.

For example:

```js
const theme = {
  primary: "{{ semantic.primary }}",
  onPrimary: "{{ semantic.on_primary }}",

  primaryContainer: "{{ semantic.primary_container }}",
  onPrimaryContainer: "{{ semantic.on_primary_container }}",

  secondary: "{{ semantic.secondary }}",
  onSecondary: "{{ semantic.on_secondary }}",

  tertiary: "{{ semantic.tertiary }}",
  onTertiary: "{{ semantic.on_tertiary }}",

  background: "{{ semantic.background }}",
  onBackground: "{{ semantic.on_background }}",

  surface: "{{ semantic.surface }}",
  onSurface: "{{ semantic.on_surface }}",

  outline: "{{ semantic.outline }}",
  error: "{{ semantic.error }}",
  onError: "{{ semantic.on_error }}",

  color0: "{{ color0 }}",
  color4: "{{ color4 }}",
  color8: "{{ color8 }}",

  foreground: "{{ foreground }}",
};
```

This allows the same generated theme to be consumed by shells, applications, widgets, and other components.

## Wallust palette

Matiz keeps the traditional Wallust palette available.

Existing palette variables such as:

```text
color0
color1
color2
color3
color4
color5
color6
color7
color8
color9
color10
color11
color12
color13
color14
color15

background
foreground
```

can continue to be used alongside the semantic colors.

This is intentional.

Terminal applications and existing Wallust templates can continue to use the palette they expect, while newer UI components can use semantic roles.

## Semantic colors and Material-style terminology

Matiz uses semantic role names such as:

```text
primary
secondary
tertiary
surface
outline
error
```

These names are also found in modern semantic UI color systems.

However, Matiz is not a Material 3 implementation and does not use Matugen's color-generation pipeline.

The semantic vocabulary provides structure; Matiz's color generation is based on its own transformations of Wallust-derived colors.

## Desktop use

Matiz can be used as the color layer for a Linux desktop.

For example:

```text
                    Matiz
                      │
          ┌───────────┼───────────┐
          │           │           │
          ▼           ▼           ▼
      Quickshell    GTK apps   Terminal
          │           │           │
          └───────────┼───────────┘
                      ▼
                   Desktop
```

A shell can use semantic roles for its UI while terminal applications continue consuming the traditional palette.

This allows different parts of the desktop to share the same wallpaper-derived theme without requiring every component to independently interpret the extracted palette.

## Example workflow

```text
                    Wallpaper
                        │
                        ▼
                 Color extraction
                        │
             ┌──────────┴──────────┐
             ▼                     ▼
      Wallust palette            Matiz
             │                     │
             ▼                     ▼
     Terminal colors        Semantic colors
             │                     │
             └──────────┬──────────┘
                        ▼
                     Desktop
```

The two outputs complement each other:

- Traditional palette colors for existing Wallust workflows
- Semantic colors for structured UI theming

## Features

- Wallpaper-derived semantic colors
- LCH-based semantic transformations
- Primary, secondary, and tertiary roles
- Container roles
- Background and surface roles
- Error roles
- Inverse roles
- Fixed roles
- Contrast handling
- Traditional Wallust palette preservation
- Template support
- Light and dark color schemes
- Rust implementation

## Installation

### Build from source

Clone the repository:

```bash
git clone https://github.com/Jatinchd777/wallust.git
cd wallust
```

Build the project:

```bash
cargo build --release
```

The release binary will be generated under:

```text
target/release/
```

You can then install the binary using your preferred method.

### Requirements

A working Rust toolchain is required to build Matiz from source.

## Configuration

Matiz currently inherits the configuration structure from the Wallust codebase.

Configuration and template paths are being transitioned from the original Wallust naming toward Matiz-specific paths.

During this transition, compatibility with the existing Wallust workflow is a priority.

## Compatibility

Matiz is based on Wallust v3.5.2.

The existing Wallust palette and template workflow is preserved while semantic colors are added as an additional capability.

The goal is for existing configurations and templates to continue working without requiring users to immediately rewrite their entire theming setup.

Matiz-specific configuration paths and CLI naming may change as the project moves toward a standalone identity.

## Development

Clone the repository and enter the project directory:

```bash
git clone https://github.com/Jatinchd777/wallust.git
cd wallust
```

Check the project:

```bash
cargo check
```

Build a release version:

```bash
cargo build --release
```

Run tests:

```bash
cargo test
```

## Roadmap

- [x] Wallpaper-derived palette extraction
- [x] Semantic color generation
- [x] LCH-based semantic transformations
- [x] Primary roles
- [x] Secondary roles
- [x] Tertiary roles
- [x] Surface roles
- [x] Error roles
- [x] Inverse roles
- [x] Fixed roles
- [x] Contrast handling
- [x] Preserve traditional Wallust palette variables
- [ ] Complete Matiz-specific naming
- [ ] Finalize configuration path migration
- [ ] Finalize CLI naming
- [ ] Improve documentation
- [ ] Expand template examples
- [ ] Expand testing across different wallpapers
- [ ] Additional packaging

## Contributing

Contributions, testing, bug reports, and ideas are welcome.

When reporting a problem with semantic colors, useful information includes:

- The wallpaper used
- The generated colors
- The configuration
- The affected semantic roles
- Screenshots showing the issue

For development, please use the existing Rust development workflow.

## Credits

Matiz is based on Wallust v3.5.2.

The original Wallust project can be found at:

https://codeberg.org/explosion-mental/wallust

Matiz retains and extends code from the upstream project. Please see the repository's license and attribution information for the applicable terms.

## License

Matiz is distributed under the MIT License, subject to the applicable copyright and license notices inherited from the upstream Wallust codebase.

See [LICENSE](LICENSE) for the full license text.

---

<p align="center">
  <strong>Wallpaper → Palette → Semantics → UI</strong>
</p>

<p align="center">
  Let the wallpaper keep its character.
</p>
