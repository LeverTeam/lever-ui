use crate::types::Color;

/// Defines the color scheme mode for the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    /// Dark color scheme with deep backgrounds and light text.
    Dark,
    /// Light color scheme with bright backgrounds and dark text.
    Light,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Dark
    }
}

/// A comprehensive set of visual tokens defining the application's design system.
///
/// `Theme` includes colors, border radii, and spacing tokens used by standard widgets
/// to ensure a consistent look and feel across the application.
#[derive(Debug, Clone)]
pub struct Theme {
    /// The root background color of the window.
    pub background: Color,
    /// The color for card-like surfaces and containers.
    pub surface: Color,
    /// A secondary surface color for grouping or distinguishing elements.
    pub surface_variant: Color,
    /// The primary brand color used for call-to-action elements.
    pub primary: Color,
    /// The primary color when hovered.
    pub primary_hover: Color,
    /// High-contrast color for text or icons placed on a primary background.
    pub on_primary: Color,
    /// Neutral secondary color for less prominent elements.
    pub secondary: Color,
    /// Color indicating success or positive status.
    pub success: Color,
    /// Color indicating danger, errors, or destructive actions.
    pub danger: Color,
    /// The standard text color.
    pub text: Color,
    /// A lower-contrast text color for labels or metadata.
    pub text_muted: Color,
    /// The color for outlines and separators.
    pub border: Color,
    /// Small border radius (e.g., for buttons).
    pub radius_sm: f32,
    /// Medium border radius (e.g., for cards).
    pub radius_md: f32,
    /// Large border radius (e.g., for modal dialogs).
    pub radius_lg: f32,
    /// Small padding/spacing token.
    pub padding_sm: f32,
    /// Medium padding/spacing token.
    pub padding_md: f32,
    /// The color used for box shadows.
    pub shadow_color: Color,
    /// Small font size token.
    pub font_size_sm: f32,
    /// Medium (default) font size token.
    pub font_size_md: f32,
    /// Large font size token.
    pub font_size_lg: f32,
}

impl Theme {
    /// Returns the default theme for the specified mode.
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
        }
    }

    /// The default dark theme.
    pub fn dark() -> Self {
        Self {
            background: Color::rgb(0.02, 0.02, 0.03),
            surface: Color::rgb(0.08, 0.08, 0.1),
            surface_variant: Color::rgb(0.12, 0.12, 0.15),
            primary: Color::rgb(0.3, 0.5, 1.0),
            primary_hover: Color::rgb(0.4, 0.6, 1.0),
            on_primary: Color::rgb(1.0, 1.0, 1.0),
            secondary: Color::rgb(0.5, 0.5, 0.6),
            success: Color::rgb(0.1, 0.7, 0.3),
            danger: Color::rgb(0.9, 0.2, 0.2),
            text: Color::rgb(0.95, 0.95, 1.0),
            text_muted: Color::rgb(0.5, 0.5, 0.6),
            border: Color::rgba(1.0, 1.0, 1.0, 0.1),
            radius_sm: 6.0,
            radius_md: 10.0,
            radius_lg: 16.0,
            padding_sm: 8.0,
            padding_md: 16.0,
            shadow_color: Color::rgba(0.0, 0.0, 0.0, 0.5),
            font_size_sm: 12.0,
            font_size_md: 14.0,
            font_size_lg: 18.0,
        }
    }

    /// The default light theme.
    pub fn light() -> Self {
        Self {
            background: Color::rgb(0.96, 0.96, 0.98),
            surface: Color::rgb(1.0, 1.0, 1.0),
            surface_variant: Color::rgb(0.94, 0.94, 0.96),
            primary: Color::rgb(0.2, 0.4, 0.9),
            primary_hover: Color::rgb(0.3, 0.5, 1.0),
            on_primary: Color::rgb(1.0, 1.0, 1.0),
            secondary: Color::rgb(0.4, 0.4, 0.5),
            success: Color::rgb(0.0, 0.6, 0.2),
            danger: Color::rgb(0.8, 0.1, 0.1),
            text: Color::rgb(0.05, 0.05, 0.1),
            text_muted: Color::rgb(0.4, 0.4, 0.5),
            border: Color::rgba(0.0, 0.0, 0.0, 0.08),
            radius_sm: 6.0,
            radius_md: 10.0,
            radius_lg: 16.0,
            padding_sm: 8.0,
            padding_md: 16.0,
            shadow_color: Color::rgba(0.0, 0.0, 0.0, 0.1),
            font_size_sm: 12.0,
            font_size_md: 14.0,
            font_size_lg: 18.0,
        }
    }

    /// Linearly interpolates between two themes. Used for smooth theme transitions.
    pub fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Self {
            background: Color::lerp(a.background, b.background, t),
            surface: Color::lerp(a.surface, b.surface, t),
            surface_variant: Color::lerp(a.surface_variant, b.surface_variant, t),
            primary: Color::lerp(a.primary, b.primary, t),
            primary_hover: Color::lerp(a.primary_hover, b.primary_hover, t),
            on_primary: Color::lerp(a.on_primary, b.on_primary, t),
            secondary: Color::lerp(a.secondary, b.secondary, t),
            success: Color::lerp(a.success, b.success, t),
            danger: Color::lerp(a.danger, b.danger, t),
            text: Color::lerp(a.text, b.text, t),
            text_muted: Color::lerp(a.text_muted, b.text_muted, t),
            border: Color::lerp(a.border, b.border, t),
            radius_sm: a.radius_sm + (b.radius_sm - a.radius_sm) * t,
            radius_md: a.radius_md + (b.radius_md - a.radius_md) * t,
            radius_lg: a.radius_lg + (b.radius_lg - a.radius_lg) * t,
            padding_sm: a.padding_sm + (b.padding_sm - a.padding_sm) * t,
            padding_md: a.padding_md + (b.padding_md - a.padding_md) * t,
            shadow_color: Color::lerp(a.shadow_color, b.shadow_color, t),
            font_size_sm: a.font_size_sm + (b.font_size_sm - a.font_size_sm) * t,
            font_size_md: a.font_size_md + (b.font_size_md - a.font_size_md) * t,
            font_size_lg: a.font_size_lg + (b.font_size_lg - a.font_size_lg) * t,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
