use lever_core::types::Color;
use lever_core::widgets::{Button, Flex};
use lever_windowing::{AppConfig, Application};

fn main() {
    let config = AppConfig {
        title: "Lever UI - Interaction".to_string(),
        ..Default::default()
    };

    let app = Application::new(
        config,
        Box::new(|_| {
            let button1 = Button::new()
                .with_colors(Color::rgb(0.3, 0.4, 0.6), Color::rgb(0.4, 0.5, 0.8))
                .with_click(|| println!("Button 1 clicked"));

            let button2 = Button::new()
                .with_colors(Color::rgb(0.6, 0.3, 0.3), Color::rgb(0.8, 0.4, 0.4))
                .with_click(|| println!("Button 2 clicked"));

            Box::new(Flex::column(vec![Box::new(button1), Box::new(button2)]))
        }),
    );

    app.run();
}
