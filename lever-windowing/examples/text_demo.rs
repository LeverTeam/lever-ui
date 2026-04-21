use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Flex, Label};
use lever_windowing::{AppConfig, Application};

fn main() {
    let config = AppConfig {
        title: "Lever UI - Text Demo".to_string(),
        width: 800,
        height: 600,
        ..Default::default()
    };

    let app = Application::new(
        config,
        Box::new(|_| {
            let label1 = Label::new("Hello, Lever UI!", 48.0);
            let label2 = Label::new("Unified GPU Atlas Batching", 24.0);

            let content = Flex::column(vec![Box::new(label1), Box::new(label2)]);

            Box::new(
                BoxWidget::new(Color::rgb(0.1, 0.1, 0.1))
                    .with_padding(SideOffsets::all(20.0))
                    .with_radius(8.0)
                    .with_child(Box::new(content)),
            )
        }),
    );

    app.run();
}
