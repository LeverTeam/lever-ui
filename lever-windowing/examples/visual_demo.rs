use lever_core::types::{BoxShadow, Color, Gradient, Point, SideOffsets};
use lever_core::widgets::{BoxWidget, Center, Flex, Label, Spacer};
use lever_windowing::{AppConfig, Application};

fn main() {
    let config = AppConfig {
        title: "Lever UI - Visual Effects Demo".to_string(),
        width: 800,
        height: 600,
        ..Default::default()
    };

    let app = Application::new(
        config,
        Box::new(|_| {
            let gradient_card = BoxWidget::new(Color::rgb(0.2, 0.2, 0.2))
                .with_size(250.0, 80.0)
                .with_radius(12.0)
                .with_gradient(Gradient::new(
                    Color::rgb(0.3, 0.4, 0.8),
                    Color::rgb(0.5, 0.2, 0.6),
                ))
                .with_padding(SideOffsets::all(20.0))
                .with_child(Box::new(Center::new(Box::new(Label::new(
                    "Gradient Card",
                    20.0,
                )))));

            let shadowed_card = BoxWidget::new(Color::rgb(0.2, 0.2, 0.2))
                .with_size(250.0, 80.0)
                .with_radius(12.0)
                .with_shadow(BoxShadow::new(
                    Point { x: 5.0, y: 5.0 },
                    10.0,
                    Color::rgba(0.0, 0.0, 0.0, 0.5),
                ))
                .with_padding(SideOffsets::all(20.0))
                .with_child(Box::new(Center::new(Box::new(Label::new(
                    "Shadowed Card",
                    20.0,
                )))));

            let combo_card = BoxWidget::new(Color::rgb(0.2, 0.2, 0.2))
                .with_size(250.0, 80.0)
                .with_radius(12.0)
                .with_gradient(Gradient::new(
                    Color::rgb(0.8, 0.4, 0.2),
                    Color::rgb(0.8, 0.2, 0.2),
                ))
                .with_shadow(BoxShadow::new(
                    Point { x: 0.0, y: 8.0 },
                    15.0,
                    Color::rgba(0.0, 0.0, 0.0, 0.6),
                ))
                .with_padding(SideOffsets::all(20.0))
                .with_child(Box::new(Center::new(Box::new(Label::new(
                    "Combined Effects",
                    20.0,
                )))));

            let root = BoxWidget::new(Color::rgb(0.05, 0.05, 0.05))
                .with_padding(SideOffsets::all(50.0))
                .with_child(Box::new(Flex::column(vec![
                    Box::new(gradient_card),
                    Box::new(Spacer::new().with_flex(1)),
                    Box::new(shadowed_card),
                    Box::new(Spacer::new().with_flex(1)),
                    Box::new(combo_card),
                ])));

            Box::new(root)
        }),
    );

    app.run();
}
