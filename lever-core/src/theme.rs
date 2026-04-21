use crate::types::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color,
    pub surface: Color,
    pub primary: Color,
    pub primary_hover: Color,
    pub on_primary: Color,
    pub text: Color,
    pub text_muted: Color,
    pub border: Color,
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
    pub padding_md: f32,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: Color::rgb(0.05, 0.05, 0.05),
            surface: Color::rgb(0.12, 0.12, 0.12),
            primary: Color::rgb(0.2, 0.6, 1.0),
            primary_hover: Color::rgb(0.3, 0.7, 1.0),
            on_primary: Color::rgb(1.0, 1.0, 1.0),
            text: Color::rgb(0.9, 0.9, 0.9),
            text_muted: Color::rgb(0.6, 0.6, 0.6),
            border: Color::rgb(0.2, 0.2, 0.2),
            radius_sm: 4.0,
            radius_md: 8.0,
            radius_lg: 12.0,
            padding_md: 12.0,
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color::rgb(0.98, 0.98, 0.98),
            surface: Color::rgb(1.0, 1.0, 1.0),
            primary: Color::rgb(0.0, 0.5, 1.0),
            primary_hover: Color::rgb(0.1, 0.6, 1.0),
            on_primary: Color::rgb(1.0, 1.0, 1.0),
            text: Color::rgb(0.1, 0.1, 0.1),
            text_muted: Color::rgb(0.4, 0.4, 0.4),
            border: Color::rgb(0.8, 0.8, 0.8),
            radius_sm: 4.0,
            radius_md: 8.0,
            radius_lg: 12.0,
            padding_md: 12.0,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
