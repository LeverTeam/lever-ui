use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Button, Flex, Label};
use lever_windowing::{AppConfig, Application};

fn main() {
    let config = AppConfig {
        title: "Lever UI - Theme Demo (Press 'T' to toggle)".to_string(),
        width: 600,
        height: 400,
        ..Default::default()
    };

    let app = Application::new(
        config,
        Box::new(|_| {
            let title = Label::new("Design System", 48.0);
            let subtitle = Label::new("Dynamic Dark/Light Mode Support", 24.0);

            let btn_primary = Button::new().with_click(|| println!("Primary clicked"));

            let btn_custom = Button::new()
                .with_colors(Color::rgb(0.8, 0.2, 0.2), Color::rgb(1.0, 0.3, 0.3))
                .with_click(|| println!("Custom clicked"));

            let content = Flex::column(vec![
                Box::new(title),
                Box::new(subtitle),
                Box::new(Flex::row(vec![
                    Box::new(
                        BoxWidget::new(Color::rgba(0.0, 0.0, 0.0, 0.0))
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(btn_primary)),
                    ),
                    Box::new(
                        BoxWidget::new(Color::rgba(0.0, 0.0, 0.0, 0.0))
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(btn_custom)),
                    ),
                ])),
            ]);

            Box::new(
                BoxWidget::new(Color::rgba(0.0, 0.0, 0.0, 0.0))
                    .with_padding(SideOffsets::all(40.0))
                    .with_child(Box::new(content)),
            )
        }),
    );

    app.run();
}
