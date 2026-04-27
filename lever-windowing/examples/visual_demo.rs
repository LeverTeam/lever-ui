use lever_core::app::{App, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{BoxShadow, Color, Gradient, Point, SideOffsets};
use lever_core::widgets::{BoxWidget, Center, Flex, Label, Spacer, ThemeToggle};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
enum Message {
    ThemeModeChanged(ThemeMode),
}

struct VisualDemo {
    theme_mode: ThemeMode,
}

impl App for VisualDemo {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ThemeModeChanged(mode) => self.theme_mode = mode,
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(Center::new(Box::new(Flex::column(vec![
                    Box::new(Flex::row(vec![
                        Box::new(
                            Flex::column(vec![
                                Box::new(Label::new("Visual Effects", 32.0, theme.text)),
                                Box::new(Label::new(
                                    "Gradients, Shadows, and Shapes",
                                    16.0,
                                    theme.text_muted,
                                )),
                            ])
                            .with_flex(1),
                        ),
                        Box::new(
                            ThemeToggle::new("visual-theme", self.theme_mode)
                                .on_changed(|mode| Message::ThemeModeChanged(mode)),
                        ),
                    ])),
                    Box::new(Spacer::new().with_size(10.0, 40.0)),
                    // Solid card with shadow
                    Box::new(
                        BoxWidget::new(theme.surface)
                            .with_radius(theme.radius_lg)
                            .with_shadow(BoxShadow::new(
                                Point { x: 0.0, y: 6.0 },
                                15.0,
                                theme.shadow_color,
                            ))
                            .with_padding(SideOffsets::all(30.0))
                            .with_child(Box::new(Label::new(
                                "Theme Surface with Shadow",
                                18.0,
                                theme.text,
                            ))),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 30.0)),
                    // Gradient card with shadow
                    Box::new(
                        BoxWidget::new(Color::BLACK)
                            .with_gradient(Gradient::new(theme.primary, theme.success))
                            .with_radius(theme.radius_lg)
                            .with_shadow(BoxShadow::new(
                                Point { x: 0.0, y: 8.0 },
                                25.0,
                                Color::rgba(theme.primary.r, theme.primary.g, theme.primary.b, 0.3),
                            ))
                            .with_padding(SideOffsets::all(30.0))
                            .with_child(Box::new(Label::new(
                                "Linear Gradient",
                                18.0,
                                Color::WHITE,
                            ))),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 30.0)),
                    // Circular button
                    Box::new(
                        BoxWidget::new(theme.danger)
                            .with_radius(40.0)
                            .with_size(80.0, 80.0)
                            .with_shadow(BoxShadow::new(
                                Point { x: 0.0, y: 4.0 },
                                12.0,
                                Color::rgba(theme.danger.r, theme.danger.g, theme.danger.b, 0.4),
                            ))
                            .with_child(Box::new(Center::new(Box::new(Label::new(
                                "!",
                                24.0,
                                Color::WHITE,
                            ))))),
                    ),
                ]))))),
        )
    }
}

fn main() {
    let config = AppConfig {
        title: "Lever Visual Demo".to_string(),
        width: 800,
        height: 800,
        clear_color: Color::rgb(0.02, 0.02, 0.03),
    };

    let app = VisualDemo {
        theme_mode: ThemeMode::Dark,
    };

    let application = Application::new(config, app);
    application.run();
}
