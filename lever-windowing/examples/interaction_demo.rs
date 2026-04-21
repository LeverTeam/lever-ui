use lever_core::app::App;
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Button, Center, Flex, Label, Spacer, Stack, TextInput};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(String),
    ButtonClicked(String),
}

struct DemoApp {
    input_text: String,
}

impl App for DemoApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextChanged(new_text) => {
                self.input_text = new_text;
            }
            Message::ButtonClicked(name) => {
                println!("Button clicked: {}", name);
            }
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        Box::new(Center::new(Box::new(
            BoxWidget::new(Color::rgb(0.12, 0.12, 0.12))
                .with_padding(SideOffsets::all(30.0))
                .with_child(Box::new(
                    Flex::column(vec![
                        Box::new(Label::new(
                            "Interaction Demo",
                            32.0,
                            Color::rgb(1.0, 1.0, 1.0),
                        )),
                        Box::new(Label::new(
                            "Type something below:",
                            18.0,
                            Color::rgba(1.0, 1.0, 1.0, 0.6),
                        )),
                        Box::new(
                            TextInput::new("input-1")
                                .with_placeholder("Type here...")
                                .with_text(&self.input_text)
                                .on_input(|text| Message::TextChanged(text)),
                        ),
                        Box::new(
                            Flex::row(vec![
                                Box::new(
                                    Button::new("Primary")
                                        .with_color(Color::rgb(0.2, 0.4, 0.8))
                                        .on_click(|| Message::ButtonClicked("Primary".to_string())),
                                ),
                                Box::new(
                                    Button::new("Secondary")
                                        .with_color(Color::rgb(0.3, 0.3, 0.3))
                                        .on_click(|| {
                                            Message::ButtonClicked("Secondary".to_string())
                                        }),
                                ),
                            ])
                            .with_gap(10.0),
                        ),
                        Box::new(Spacer::new().with_flex(1)),
                        Box::new(
                            Stack::new(vec![
                                Box::new(
                                    BoxWidget::new(Color::rgb(0.18, 0.18, 0.18))
                                        .with_radius(12.0)
                                        .with_padding(SideOffsets::all(40.0))
                                        .with_child(Box::new(Label::new(
                                            "Background",
                                            16.0,
                                            Color::rgba(1.0, 1.0, 1.0, 0.2),
                                        ))),
                                ),
                                Box::new(Center::new(Box::new(
                                    BoxWidget::new(Color::rgb(0.7, 0.1, 0.1))
                                        .with_radius(100.0)
                                        .with_padding(SideOffsets::all(20.0))
                                        .with_child(Box::new(Label::new(
                                            "Overlay",
                                            14.0,
                                            Color::rgb(1.0, 1.0, 1.0),
                                        ))),
                                ))),
                            ])
                            .with_id("stack-1"),
                        ),
                    ])
                    .with_gap(20.0),
                )),
        )))
    }
}

fn main() {
    let config = AppConfig {
        title: "Lever Interaction Demo".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.1, 0.1, 0.1),
    };

    let app = DemoApp {
        input_text: String::new(),
    };

    let application = Application::new(config, app);
    application.run();
}
