use lever_core::animated::{animated_color, animated_spring};
use lever_core::animation::Spring;
use lever_core::app::{App, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{BoxShadow, Color, Point, SideOffsets};
use lever_core::widget::Widget;
use lever_core::widgets::{BoxWidget, Button, Center, Flex, Label, Spacer};
use lever_windowing::application::Application;
use lever_windowing::AppConfig;

struct AnimationV2App {
    theme_mode: ThemeMode,
    is_active: bool,
}

#[derive(Clone, Debug)]
enum Message {
    ToggleTheme,
    ToggleActive,
}

impl App for AnimationV2App {
    type Message = Message;

    fn update(&mut self, message: Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ToggleTheme => {
                self.theme_mode = match self.theme_mode {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                };
            }
            Message::ToggleActive => {
                self.is_active = !self.is_active;
            }
        }
    }

    fn view(&self) -> Box<dyn Widget<Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        // Animate a custom box position and color using the new implicit API
        let target_x = if self.is_active { 600.0 } else { 0.0 };
        let target_color = if self.is_active {
            theme.success
        } else {
            theme.primary
        };

        let animated_x = animated_spring("box_x", target_x, Spring::BOUNCY).round();
        let animated_color = animated_color("box_color", target_color, 0.3);

        Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(Flex::column(vec![
                    Box::new(Label::new("Advanced Animation System", 32.0, theme.text)),
                    Box::new(Label::new(
                        "Physics-based springs and implicit animations",
                        16.0,
                        theme.text_muted,
                    )),
                    Box::new(Spacer::new().with_size(10.0, 40.0)),
                    // Button with built-in spring scale and color animation
                    Box::new(Flex::row(vec![
                        Box::new(
                            Button::new("Toggle Active State")
                                .with_id("main_button")
                                .on_click(|| Message::ToggleActive),
                        ),
                        Box::new(Spacer::new().with_size(20.0, 10.0)),
                        Box::new(
                            Button::new("Switch Theme")
                                .with_id("theme_button")
                                .on_click(|| Message::ToggleTheme),
                        ),
                    ])),
                    Box::new(Spacer::new().with_size(10.0, 60.0)),
                    // The custom animated box
                    Box::new(Flex::row(vec![
                        Box::new(Spacer::new().with_size(animated_x, 10.0)),
                        Box::new(
                            BoxWidget::<Message>::new(animated_color)
                                .with_size(120.0, 120.0)
                                .with_radius(20.0)
                                .with_shadow(BoxShadow::new(
                                    Point { x: 0.0, y: 10.0 },
                                    30.0,
                                    Color::rgba(
                                        animated_color.r,
                                        animated_color.g,
                                        animated_color.b,
                                        0.4,
                                    ),
                                ))
                                .with_child(Box::new(Center::new(Box::new(Label::new(
                                    "Spring!",
                                    18.0,
                                    Color::WHITE,
                                ))))),
                        ),
                    ])),
                ]))),
        )
    }
}

fn main() {
    let app = AnimationV2App {
        theme_mode: ThemeMode::Dark,
        is_active: false,
    };

    let config = AppConfig {
        title: "Lever Animation v2".to_string(),
        width: 1000,
        height: 800,
        ..Default::default()
    };

    Application::new(config, app).run();
}
