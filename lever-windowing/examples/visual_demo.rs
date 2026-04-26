use lever_core::app::{App, UpdateContext};
use lever_core::types::{BoxShadow, Color, Gradient, Point, SideOffsets};
use lever_core::widgets::{BoxWidget, Center, Flex, Label, Spacer};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct VisualDemo;

#[derive(Debug, Clone)]
enum Message {}

impl App for VisualDemo {
    type Message = Message;

    fn update(&mut self, _message: Self::Message, _ctx: &mut UpdateContext) {}

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        Box::new(Center::new(Box::new(
            Flex::column(vec![
                // Solid card with shadow
                Box::new(
                    BoxWidget::new(Color::rgb(0.15, 0.15, 0.15))
                        .with_radius(12.0)
                        .with_shadow(BoxShadow::new(
                            Point { x: 0.0, y: 4.0 },
                            12.0,
                            Color::rgba(0.0, 0.0, 0.0, 0.5),
                        ))
                        .with_padding(SideOffsets::all(20.0))
                        .with_child(Box::new(Label::new(
                            "Solid with Shadow",
                            18.0,
                            Color::rgb(1.0, 1.0, 1.0),
                        ))),
                ),
                Box::new(Spacer::height(30.0)),
                // Gradient card with shadow
                Box::new(
                    BoxWidget::new(Color::rgb(0.0, 0.0, 0.0))
                        .with_gradient(Gradient::new(
                            Color::rgb(0.2, 0.4, 0.8), // Blueish
                            Color::rgb(0.5, 0.2, 0.7), // Purplish
                        ))
                        .with_radius(12.0)
                        .with_shadow(BoxShadow::new(
                            Point { x: 0.0, y: 8.0 },
                            20.0,
                            Color::rgba(0.0, 0.0, 0.0, 0.4),
                        ))
                        .with_padding(SideOffsets::all(20.0))
                        .with_child(Box::new(Label::new(
                            "Gradient with Shadow",
                            18.0,
                            Color::rgb(1.0, 1.0, 1.0),
                        ))),
                ),
                Box::new(Spacer::height(30.0)),
                // Circular gradient
                Box::new(
                    BoxWidget::new(Color::rgb(0.0, 0.0, 0.0))
                        .with_gradient(Gradient::new(
                            Color::rgb(1.0, 0.5, 0.0), // Orange
                            Color::rgb(1.0, 0.2, 0.2), // Red
                        ))
                        .with_radius(50.0)
                        .with_size(100.0, 100.0)
                        .with_child(Box::new(Center::new(Box::new(Label::new(
                            "!",
                            32.0,
                            Color::rgb(1.0, 1.0, 1.0),
                        ))))),
                ),
            ])
            .with_gap(20.0),
        )))
    }
}

fn main() {
    let app = VisualDemo;
    let config = AppConfig {
        title: "Lever Visual Demo".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}

