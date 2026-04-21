use lever_core::types::Color;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub clear_color: Color,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "Lever UI".to_string(),
            width: 800,
            height: 600,
            clear_color: Color::rgb(0.1, 0.1, 0.1),
        }
    }
}
