use lever_core::types::{Color, Rect, Size};
use lever_core::widget::Widget;
use lever_core::widgets::{Button, Flex};
use lever_windowing::{AppConfig, Application};

fn main() {
    let config = AppConfig {
        title: "Lever UI - Interaction".to_string(),
        ..Default::default()
    };

    let app = Application::new(
        config,
        Box::new(|cursor_pos| {
            let b1_rect = Rect {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 40.0,
            };
            let b2_rect = Rect {
                x: 0.0,
                y: 40.0,
                width: 100.0,
                height: 40.0,
            };

            let mut button1 = Button::new(Color::rgb(0.3, 0.4, 0.6), Color::rgb(0.4, 0.5, 0.8));
            button1.is_hovered = b1_rect.contains(cursor_pos);

            let mut button2 = Button::new(Color::rgb(0.6, 0.3, 0.3), Color::rgb(0.8, 0.4, 0.4));
            button2.is_hovered = b2_rect.contains(cursor_pos);

            Box::new(Flex::column(vec![Box::new(button1), Box::new(button2)]))
        }),
    );

    app.run();
}
