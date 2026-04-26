use crate::types::Color;

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
}

impl Theme {
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
            text: Color::rgb(0.05, 0.05, 0.1),
            text_muted: Color::rgb(0.4, 0.4, 0.5),
            border: Color::rgba(0.0, 0.0, 0.0, 0.08),
            radius_sm: 6.0,
            radius_md: 10.0,
            radius_lg: 16.0,
            padding_sm: 8.0,
            padding_md: 16.0,
            shadow_color: Color::rgba(0.0, 0.0, 0.0, 0.1),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
