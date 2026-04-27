use lever_core::app::{App, UpdateContext};
use lever_core::layout::{Alignment, CrossAxisAlignment};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, SideOffsets};
use lever_core::widget::Widget;
use lever_core::widgets::{
    child, BoxWidget, Button, ConstraintLayout, Flex, Label, Positioned, Spacer, Stack, PARENT,
};
use lever_windowing::{AppConfig, Application};

struct LayoutDemoApp {
    theme_mode: ThemeMode,
}

#[derive(Clone, Debug)]
enum Message {
    ToggleTheme,
}

impl App for LayoutDemoApp {
    type Message = Message;

    fn update(&mut self, message: Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ToggleTheme => {
                self.theme_mode = match self.theme_mode {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                };
                _ctx.set_theme(self.theme_mode);
            }
        }
    }

    fn view(&self) -> Box<dyn Widget<Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        let header = Box::new(
            BoxWidget::new(theme.surface)
                .with_padding(SideOffsets::new(16.0, 24.0, 16.0, 24.0))
                .with_child(Box::new(Flex::row(vec![
                    Box::new(Label::new("Layout Enhancements", 24.0, theme.text))
                        as Box<dyn Widget<Message>>,
                    Box::new(Spacer::new().with_flex(1)) as Box<dyn Widget<Message>>,
                    Box::new(Button::new("Toggle Theme").on_press(|| Message::ToggleTheme))
                        as Box<dyn Widget<Message>>,
                ]))),
        ) as Box<dyn Widget<Message>>;

        let content = Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(24.0))
                .with_child(Box::new(
                    Flex::column(vec![
                        // Stack Section
                        self.section_title("Stack & Positioned", &theme),
                        Box::new(
                            BoxWidget::new(Color::rgb(0.2, 0.2, 0.3))
                                .with_radius(8.0)
                                .with_height(200.0)
                                .with_child(Box::new(
                                    Stack::new(vec![
                                        // Background (stretching)
                                        Box::new(
                                            Positioned::new(Box::new(
                                                BoxWidget::new(Color::rgb(0.3, 0.4, 0.6))
                                                    .with_radius(8.0),
                                            ))
                                            .top(0.0)
                                            .bottom(0.0)
                                            .left(0.0)
                                            .right(0.0),
                                        )
                                            as Box<dyn Widget<Message>>,
                                        // Top Left
                                        Box::new(
                                            Positioned::new(Box::new(
                                                BoxWidget::new(Color::rgb(0.8, 0.2, 0.2))
                                                    .with_size(40.0, 40.0)
                                                    .with_radius(20.0),
                                            ))
                                            .top(10.0)
                                            .left(10.0),
                                        )
                                            as Box<dyn Widget<Message>>,
                                        // Bottom Right
                                        Box::new(
                                            Positioned::new(Box::new(
                                                BoxWidget::new(Color::rgb(0.2, 0.8, 0.2))
                                                    .with_size(40.0, 40.0)
                                                    .with_radius(4.0),
                                            ))
                                            .bottom(10.0)
                                            .right(10.0),
                                        )
                                            as Box<dyn Widget<Message>>,
                                        // Centered Overlay
                                        Box::new(
                                            BoxWidget::new(Color::rgb(1.0, 1.0, 1.0))
                                                .with_padding(SideOffsets::all(8.0))
                                                .with_radius(4.0)
                                                .with_child(Box::new(Label::new(
                                                    "Centered Overlay",
                                                    12.0,
                                                    Color::BLACK,
                                                ))),
                                        )
                                            as Box<dyn Widget<Message>>,
                                    ])
                                    .with_alignment(Alignment::Center),
                                )),
                        ) as Box<dyn Widget<Message>>,
                        // ConstraintLayout Section
                        self.section_title("ConstraintLayout", &theme),
                        Box::new(
                            BoxWidget::new(Color::rgb(0.2, 0.3, 0.2))
                                .with_radius(8.0)
                                .with_height(200.0)
                                .with_child(Box::new(
                                    ConstraintLayout::new()
                                        .with_child(
                                            Box::new(
                                                BoxWidget::new(Color::rgb(0.5, 0.2, 0.7))
                                                    .with_size(60.0, 60.0)
                                                    .with_radius(4.0),
                                            ),
                                            |c| c.center_x(PARENT, 0.0).top_to_top(PARENT, 20.0),
                                        )
                                        .with_child(
                                            Box::new(Label::new(
                                                "Anchored label",
                                                12.0,
                                                theme.text,
                                            )),
                                            |c| {
                                                c.center_x(PARENT, 0.0)
                                                    .top_to_bottom(child(0), 10.0)
                                            },
                                        )
                                        .with_child(Box::new(Button::new("Left")), |c| {
                                            c.right_to_left(child(0), -10.0).center_y(child(0), 0.0)
                                        })
                                        .with_child(Box::new(Button::new("Right")), |c| {
                                            c.left_to_right(child(0), 10.0).center_y(child(0), 0.0)
                                        }),
                                )),
                        ) as Box<dyn Widget<Message>>,
                    ])
                    .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
                    .with_gap(20.0),
                )),
        ) as Box<dyn Widget<Message>>;

        Box::new(Flex::column(vec![header, content]).with_flex(1))
    }
}

impl LayoutDemoApp {
    fn section_title(&self, text: &str, theme: &Theme) -> Box<dyn Widget<Message>> {
        Box::new(
            Flex::column(vec![
                Box::new(Label::new(text, 16.0, theme.text)) as Box<dyn Widget<Message>>,
                Box::new(BoxWidget::new(theme.border).with_height(1.0)) as Box<dyn Widget<Message>>,
            ])
            .with_gap(4.0),
        )
    }
}

fn main() {
    let app = LayoutDemoApp {
        theme_mode: ThemeMode::Dark,
    };

    let config = AppConfig {
        title: "Lever Layout Debug".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };

    Application::new(config, app).run();
}
