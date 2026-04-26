use lever_core::app::{App, UpdateContext};
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Center, Flex, Label, Spacer};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct TextApp;

#[derive(Debug, Clone)]
enum Message {}

impl App for TextApp {
    type Message = Message;

    fn update(&mut self, _message: Self::Message, _ctx: &mut UpdateContext) {}

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let label1 = Label::new("Hello, Lever UI!", 48.0, Color::rgb(1.0, 1.0, 1.0));
        let label2 = Label::new(
            "Unified GPU Atlas Batching",
            24.0,
            Color::rgb(0.7, 0.7, 0.7),
        );
        let label3 = Label::new("Fast. Smooth. Agentic.", 18.0, Color::rgb(0.2, 0.6, 0.9));

        let content = Flex::column(vec![
            Box::new(label1),
            Box::new(Spacer::height(10.0)),
            Box::new(label2),
            Box::new(Spacer::height(20.0)),
            Box::new(label3),
        ]);

        Box::new(Center::new(Box::new(
            BoxWidget::new(Color::rgb(0.1, 0.1, 0.1))
                .with_padding(SideOffsets::all(40.0))
                .with_radius(12.0)
                .with_child(Box::new(content)),
        )))
    }
}

fn main() {
    let app = TextApp;
    let config = AppConfig {
        title: "Lever UI - Text Demo".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}

