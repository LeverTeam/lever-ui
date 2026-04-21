use lever_core::types::{Color, Rect};
use lever_windowing::{AppConfig, Application};

fn main() {
    let config = AppConfig {
        title: "Hello Lever UI".to_string(),
        ..Default::default()
    };

    let app = Application::new(
        config,
        Box::new(|draw_list| {
            // Background
            draw_list.colored_rect(
                Rect {
                    x: 50.0,
                    y: 50.0,
                    width: 700.0,
                    height: 500.0,
                },
                Color::rgb(0.2, 0.2, 0.2),
                0.0,
            );

            // Center rect
            draw_list.colored_rect(
                Rect {
                    x: 300.0,
                    y: 200.0,
                    width: 200.0,
                    height: 200.0,
                },
                Color::rgb(0.3, 0.5, 0.8),
                0.0,
            );

            // Accent rect
            draw_list.colored_rect(
                Rect {
                    x: 350.0,
                    y: 250.0,
                    width: 100.0,
                    height: 100.0,
                },
                Color::rgb(0.9, 0.4, 0.3),
                0.0,
            );
        }),
    );

    app.run();
}
