use lever_core::app::App;
use lever_core::types::Color;
use lever_core::widgets::{BoxWidget, Center, Label};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct HelloApp;

#[derive(Debug, Clone)]
enum Message {}

impl App for HelloApp {
    type Message = Message;

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        Box::new(Center::new(Box::new(
            BoxWidget::new(Color::rgb(0.2, 0.4, 0.2))
                .with_radius(8.0)
                .with_child(Box::new(Label::new(
                    "Hello Lever!",
                    32.0,
                    Color::rgb(1.0, 1.0, 1.0),
                ))),
        )))
    }
}

fn main() {
    let app = HelloApp;
    let config = AppConfig {
        title: "Lever UI - Hello".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}
