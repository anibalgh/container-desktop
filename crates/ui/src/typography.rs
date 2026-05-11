use iced::Font;

/// Scales font sizes proportionally to a user-configured base size.
///
/// Default base is 14 px. When the user selects e.g. 20 px, all explicit sizes
/// are multiplied by 20/14 ≈ 1.43, preserving the visual hierarchy.
#[derive(Debug, Clone, Copy)]
pub struct FontScale {
    factor: f32,
}

impl FontScale {
    /// Creates a new scaler for the given base font size (in pixels).
    pub fn new(base_size: u16) -> Self {
        Self {
            factor: base_size as f32 / 14.0,
        }
    }

    /// Returns the scaled size for a given design-time base size as `f32`.
    ///
    /// `iced::Pixels` only implements `From<f32>` and `From<u32>`.
    /// Returning `f32` allows direct use with `.size(fs.size(N))`.
    ///
    /// # Example
    /// ```ignore
    /// let fs = FontScale::new(20); // user wants 20 px base
    /// assert_eq!(fs.size(14), 20.0); // design-time 14 → 20.0
    /// assert_eq!(fs.size(20), 29.0); // design-time 20 → ~28.6
    /// ```
    pub fn size(&self, design_base: u16) -> f32 {
        design_base as f32 * self.factor
    }
}

impl Default for FontScale {
    fn default() -> Self {
        Self::new(14)
    }
}

/// Curated list of popular monospace font families available for selection.
pub const MONOSPACE_FONTS: &[&str] = &[
    "Monospace",
    "Fira Code",
    "JetBrains Mono",
    "Hack",
    "Source Code Pro",
    "Cascadia Code",
    "Iosevka",
    "Monoid",
    "Ubuntu Mono",
    "DejaVu Sans Mono",
    "Liberation Mono",
    "Noto Sans Mono",
    "IBM Plex Mono",
    "Inconsolata",
    "Anonymous Pro",
    "Courier New",
    "Consolas",
    "SF Mono",
    "Menlo",
    "Droid Sans Mono",
];

/// Resolves a font family string to an `iced::Font`.
///
/// - `"Monospace"` → `Font::MONOSPACE` (system default monospace)
/// - Any other name → `Font` with `Family::Name`, falling back to `Font::DEFAULT` for weight/stretch/style
///
/// Uses `Box::leak` to satisfy `Family::Name(&'static str)`. The leaked memory is negligible
/// (a few bytes) and lives for the process lifetime, which is acceptable for startup-only font config.
pub fn resolve_font(family: &str) -> Font {
    if family.eq_ignore_ascii_case("monospace") {
        Font::MONOSPACE
    } else {
        let leaked: &'static str = Box::leak(family.to_string().into_boxed_str());
        Font {
            family: iced::font::Family::Name(leaked),
            ..Font::DEFAULT
        }
    }
}
