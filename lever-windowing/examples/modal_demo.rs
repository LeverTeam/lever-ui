use lever_core::app::{App, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, SideOffsets};
use lever_core::widget::Widget;
use lever_core::widgets::{
    BoxWidget, Button, Center, Flex, Label, Overlay, Spacer, Stack, ThemeToggle,
};
use lever_windowing::{AppConfig, Application};

struct ModalDemoApp {
    theme_mode: ThemeMode,
    is_modal_open: bool,
    counter: i32,
}

#[derive(Clone, Debug)]
enum Message {
    ToggleTheme(ThemeMode),
    OpenModal,
    CloseModal,
    Increment,
    Decrement,
}

impl App for ModalDemoApp {
    type Message = Message;

    fn update(&mut self, message: Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ToggleTheme(mode) => {
                self.theme_mode = mode;
                _ctx.set_theme(mode);
            }
            Message::OpenModal => self.is_modal_open = true,
            Message::CloseModal => self.is_modal_open = false,
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter -= 1,
        }
    }

    fn view(&self) -> Box<dyn Widget<Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        // Main Application Content
        let main_content = Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(Flex::column(vec![
                    Box::new(Flex::row(vec![
                        Box::new(
                            Flex::column(vec![
                                Box::new(Label::new("Modal & Dialog System", 32.0, theme.text)),
                                Box::new(Label::new(
                                    "Lever UI Overlay & Event Blocking",
                                    16.0,
                                    theme.text_muted,
                                )),
                            ])
                            .with_flex(1),
                        ),
                        Box::new(
                            ThemeToggle::new("modal-theme", self.theme_mode)
                                .on_changed(|mode| Message::ToggleTheme(mode)),
                        ),
                    ])),
                    Box::new(Spacer::new().with_size(10.0, 60.0)),
                    Box::new(Center::new(Box::new(
                        BoxWidget::new(theme.surface)
                            .with_radius(theme.radius_lg)
                            .with_padding(SideOffsets::all(40.0))
                            .with_child(Box::new(Flex::column(vec![
                                Box::new(Label::new(
                                    format!("Counter: {}", self.counter),
                                    24.0,
                                    theme.text,
                                )),
                                Box::new(Spacer::new().with_size(10.0, 20.0)),
                                Box::new(Flex::row(vec![
                                    Box::new(
                                        Button::new("Decrement").on_click(|| Message::Decrement),
                                    ),
                                    Box::new(Spacer::new().with_size(10.0, 10.0)),
                                    Box::new(
                                        Button::new("Increment").on_click(|| Message::Increment),
                                    ),
                                ])),
                                Box::new(Spacer::new().with_size(10.0, 40.0)),
                                Box::new(
                                    Button::new("Open Confirm Dialog")
                                        .with_color(theme.primary)
                                        .on_click(|| Message::OpenModal),
                                ),
                            ]))),
                    ))),
                ]))),
        );

        // Modal Overlay Layer
        let mut layers: Vec<Box<dyn Widget<Message>>> = vec![main_content];

        if self.is_modal_open {
            layers.push(Box::new(
                Overlay::new(Color::rgba(0.0, 0.0, 0.0, 0.6)).on_dismiss(|| Message::CloseModal),
            ));

            layers.push(Box::new(Center::new(Box::new(
                BoxWidget::new(theme.surface)
                    .with_radius(theme.radius_lg)
                    .with_padding(SideOffsets::all(30.0))
                    .with_size(400.0, 250.0)
                    .with_child(Box::new(Flex::column(vec![
                        Box::new(Label::new("Confirm Action", 20.0, theme.text)),
                        Box::new(Spacer::new().with_size(10.0, 15.0)),
                        Box::new(Label::new(
                            "Are you sure you want to proceed? This will block background interactions until closed.",
                            14.0,
                            theme.text_muted,
                        )),
                        Box::new(Spacer::new().with_flex(1)),
                        Box::new(Flex::row(vec![
                            Box::new(Spacer::new().with_flex(1)),
                            Box::new(
                                Button::new("Cancel")
                                    .with_color(theme.surface)
                                    .on_click(|| Message::CloseModal),
                            ),
                            Box::new(Spacer::new().with_size(10.0, 10.0)),
                            Box::new(
                                Button::new("Proceed")
                                    .with_color(theme.primary)
                                    .on_click(|| Message::CloseModal),
                            ),
                        ])),
                    ]))),
            ))));
        }

        Box::new(Stack::new(layers))
    }
}

fn main() {
    let app = ModalDemoApp {
        theme_mode: ThemeMode::Dark,
        is_modal_open: false,
        counter: 0,
    };

    let config = AppConfig {
        title: "Lever UI - Modal Demo".to_string(),
        width: 1000,
        height: 800,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };

    Application::new(config, app).run();
}
