use lever_core::app::{App, UpdateContext};
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Button, Center, Flex, Label, Spacer};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct CounterApp {
    count: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

impl App for CounterApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        Box::new(Center::new(Box::new(
            BoxWidget::new(Color::rgb(0.1, 0.1, 0.1))
                .with_radius(12.0)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(Flex::column(vec![
                    Box::new(Label::new(
                        format!("Count: {}", self.count),
                        48.0,
                        Color::rgb(1.0, 1.0, 1.0),
                    )),
                    Box::new(Spacer::height(20.0)),
                    Box::new(Flex::row(vec![
                        Box::new(
                            Button::new("Decrement")
                                .with_color(Color::rgb(0.8, 0.2, 0.2))
                                .on_click(|| Message::Decrement),
                        ),
                        Box::new(Spacer::width(10.0)),
                        Box::new(
                            Button::new("Increment")
                                .with_color(Color::rgb(0.2, 0.6, 0.2))
                                .on_click(|| Message::Increment),
                        ),
                    ])),
                ]))),
        )))
    }
}

fn main() {
    let app = CounterApp { count: 0 };
    let config = AppConfig {
        title: "Lever Counter Demo".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}

