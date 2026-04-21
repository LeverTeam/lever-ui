use lever_core::types::Color;
use lever_core::widgets::BoxWidget;
use lever_windowing::{AppConfig, Application};

fn main() {
    let config = AppConfig {
        title: "Lever UI - Hello".to_string(),
        ..Default::default()
    };

    let app = Application::new(
        config,
        Box::new(|_cursor_pos| Box::new(BoxWidget::new(Color::rgb(0.2, 0.4, 0.2)))),
    );

    app.run();
}
