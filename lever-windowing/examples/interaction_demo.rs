use lever_core::app::{App, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{
    BoxWidget, Button, Center, Flex, Label, Spacer, Stack, TextInput, ThemeToggle,
};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(String, usize),
    ButtonClicked(String),
    ThemeModeChanged(ThemeMode),
}

struct DemoApp {
    input_text: String,
    cursor_index: usize,
    theme_mode: ThemeMode,
}

impl App for DemoApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::TextChanged(new_text, new_cursor) => {
                self.input_text = new_text;
                self.cursor_index = new_cursor;
            }
            Message::ButtonClicked(name) => {
                println!("Button clicked: {}", name);
            }
            Message::ThemeModeChanged(mode) => {
                self.theme_mode = mode;
            }
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(Center::new(Box::new(
                    Flex::column(vec![
                        Box::new(
                            Flex::row(vec![
                                Box::new(
                                    Flex::column(vec![
                                        Box::new(Label::new("Interaction Demo", 32.0, theme.text)),
                                        Box::new(Label::new(
                                            "Testing event handling and focus",
                                            16.0,
                                            theme.text_muted,
                                        )),
                                    ])
                                    .with_flex(1),
                                ),
                                Box::new(
                                    ThemeToggle::new("interaction-theme", self.theme_mode)
                                        .on_changed(|mode| Message::ThemeModeChanged(mode)),
                                ),
                            ])
                            .with_gap(20.0),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 40.0)),
                        Box::new(Label::new(
                            "TextInput (Focus and Key Events):",
                            18.0,
                            theme.text,
                        )),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(
                            TextInput::new("input-1")
                                .with_placeholder("Type something here...")
                                .with_text(&self.input_text)
                                .with_cursor(self.cursor_index)
                                .on_input(|text, cursor| Message::TextChanged(text, cursor)),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 20.0)),
                        Box::new(
                            Flex::row(vec![
                                Box::new(
                                    Button::new("Primary Action")
                                        .with_color(theme.primary)
                                        .on_click(|| Message::ButtonClicked("Primary".to_string())),
                                ),
                                Box::new(
                                    Button::new("Secondary")
                                        .with_color(theme.secondary)
                                        .on_click(|| {
                                            Message::ButtonClicked("Secondary".to_string())
                                        }),
                                ),
                            ])
                            .with_gap(15.0),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 40.0)),
                        Box::new(Label::new("Stack and Overlays:", 18.0, theme.text)),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(Stack::new(vec![
                            Box::new(
                                BoxWidget::new(theme.surface)
                                    .with_radius(theme.radius_lg)
                                    .with_padding(SideOffsets::all(60.0))
                                    .with_child(Box::new(Label::new(
                                        "Base Layer",
                                        16.0,
                                        theme.text_muted,
                                    ))),
                            ),
                            Box::new(Center::new(Box::new(
                                BoxWidget::new(theme.success)
                                    .with_radius(100.0)
                                    .with_padding(SideOffsets::all(20.0))
                                    .with_child(Box::new(Label::new(
                                        "Overlay",
                                        14.0,
                                        Color::WHITE,
                                    ))),
                            ))),
                        ])),
                    ])
                    .with_gap(10.0),
                )))),
        )
    }
}

fn main() {
    let config = AppConfig {
        title: "Lever Interaction Demo".to_string(),
        width: 900,
        height: 800,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };

    let app = DemoApp {
        input_text: String::new(),
        cursor_index: 0,
        theme_mode: ThemeMode::Dark,
    };

    let application = Application::new(config, app);
    application.run();
}
