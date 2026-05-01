use crate::types::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Dark
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color,
    pub surface: Color,
    pub surface_variant: Color,
    pub primary: Color,
    pub primary_hover: Color,
    pub on_primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub danger: Color,
    pub text: Color,
    pub text_muted: Color,
    pub border: Color,
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
    pub padding_sm: f32,
    pub padding_md: f32,
    pub shadow_color: Color,
    pub font_size_sm: f32,
    pub font_size_md: f32,
    pub font_size_lg: f32,
}

impl Theme {
    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
        }
    }

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
            text: Color::rgb(0.02, 0.02, 0.05),
            text_muted: Color::rgb(0.35, 0.35, 0.45),
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
