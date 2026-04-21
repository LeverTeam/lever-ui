use lever_core::types::{Color, Rect, SideOffsets};
use lever_core::widgets::{BoxWidget, Button, Flex, Label};
use lever_windowing::{AppConfig, Application};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

fn main() {
    let config = AppConfig {
        title: "Lever UI - Counter Example".to_string(),
        width: 400,
        height: 300,
        ..Default::default()
    };

    let counter = Arc::new(AtomicI32::new(0));

    let app = Application::new(
        config,
        Box::new(move |_cursor_pos| {
            let count = counter.load(Ordering::Relaxed);

            let label = Label::new(format!("Count: {}", count), 48.0);

            let btn = Button::new()
                .with_colors(Color::rgb(0.2, 0.6, 0.2), Color::rgb(0.3, 0.8, 0.3))
                .with_click({
                    let counter = counter.clone();
                    move || {
                        counter.fetch_add(1, Ordering::Relaxed);
                        println!(
                            "Button clicked! New count: {}",
                            counter.load(Ordering::Relaxed)
                        );
                    }
                });

            let content = Flex::column(vec![Box::new(label), Box::new(btn)]);

            Box::new(
                BoxWidget::new(Color::rgb(0.1, 0.1, 0.1))
                    .with_padding(SideOffsets::all(40.0))
                    .with_child(Box::new(content)),
            )
        }),
    );

    app.run();
}
