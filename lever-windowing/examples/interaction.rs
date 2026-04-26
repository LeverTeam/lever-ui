use lever_core::app::{App, UpdateContext};
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Button, Center, Flex, Spacer};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct InteractionApp;

#[derive(Debug, Clone)]
enum Message {
    ButtonClicked(u32),
}

impl App for InteractionApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ButtonClicked(id) => println!("Button {} clicked!", id),
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        Box::new(Center::new(Box::new(
            BoxWidget::new(Color::rgb(0.12, 0.12, 0.12))
                .with_radius(8.0)
                .with_padding(SideOffsets::all(30.0))
                .with_child(Box::new(Flex::column(vec![
                    Box::new(
                        Button::new("Primary Action")
                            .with_color(Color::rgb(0.2, 0.4, 0.8))
                            .on_click(|| Message::ButtonClicked(1)),
                    ),
                    Box::new(Spacer::height(15.0)),
                    Box::new(
                        Button::new("Secondary Action")
                            .with_color(Color::rgb(0.4, 0.4, 0.4))
                            .on_click(|| Message::ButtonClicked(2)),
                    ),
                ]))),
        )))
    }
}

fn main() {
    let app = InteractionApp;
    let config = AppConfig {
        title: "Lever UI - Interaction".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}

