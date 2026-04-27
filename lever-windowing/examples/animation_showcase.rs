use lever_core::animation::{AnimationController, Ease};
use lever_core::app::{App, Context, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, Point, SideOffsets};
use lever_core::widgets::*;
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
enum Message {
    ToggleVisibility,
    ThemeModeChanged(ThemeMode),
}

struct AnimationShowcase {
    opacity_ctrl: AnimationController,
    slide_ctrl: AnimationController,
    is_visible: bool,
    theme_mode: ThemeMode,
}

impl AnimationShowcase {
    fn new() -> Self {
        Self {
            opacity_ctrl: AnimationController::new(1.0),
            slide_ctrl: AnimationController::new(0.0),
            is_visible: true,
            theme_mode: ThemeMode::Dark,
        }
    }
}

impl App for AnimationShowcase {
    type Message = Message;

    fn init(&mut self, _ctx: &mut Context<Self::Message>) {}

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ToggleVisibility => {
                self.is_visible = !self.is_visible;
                let (opacity, slide) = if self.is_visible {
                    (1.0, 0.0)
                } else {
                    (0.0, 100.0)
                };
                self.opacity_ctrl.animate_to(opacity, 0.5, Ease::QuadInOut);
                self.slide_ctrl.animate_to(slide, 0.5, Ease::CubicOut);
            }
            Message::ThemeModeChanged(mode) => {
                self.theme_mode = mode;
            }
        }
    }

    fn tick(&mut self, dt: f32) {
        self.opacity_ctrl.tick(dt);
        self.slide_ctrl.tick(dt);
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(Center::new(Box::new(Flex::column(vec![
                    Box::new(
                        Flex::row(vec![
                            Box::new(
                                Flex::column(vec![
                                    Box::new(Label::new("Animation Showcase", 32.0, theme.text)),
                                    Box::new(Label::new(
                                        "Testing AnimatedOpacity and AnimatedTranslation",
                                        16.0,
                                        theme.text_muted,
                                    )),
                                ])
                                .with_flex(1),
                            ),
                            Box::new(
                                ThemeToggle::new("theme-toggle", self.theme_mode)
                                    .on_changed(|mode| Message::ThemeModeChanged(mode)),
                            ),
                        ])
                        .with_gap(20.0),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 40.0)),
                    Box::new(
                        Button::new("Toggle Animation")
                            .with_color(theme.primary)
                            .on_click(|| Message::ToggleVisibility),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 40.0)),
                    Box::new(AnimatedTranslation::new(
                        Point {
                            x: 0.0,
                            y: self.slide_ctrl.value(),
                        },
                        Box::new(AnimatedOpacity::new(
                            self.opacity_ctrl.value(),
                            Box::new(
                                BoxWidget::new(theme.primary)
                                    .with_size(240.0, 240.0)
                                    .with_radius(24.0)
                                    .with_child(Box::new(Center::new(Box::new(Label::new(
                                        "Animated Card",
                                        18.0,
                                        Color::WHITE,
                                    ))))),
                            ),
                        )),
                    )),
                    Box::new(Spacer::new().with_size(10.0, 40.0)),
                    Box::new(Label::new(
                        format!("Opacity: {:.2}", self.opacity_ctrl.value()),
                        14.0,
                        theme.text_muted,
                    )),
                    Box::new(Label::new(
                        format!("Offset Y: {:.2}", self.slide_ctrl.value()),
                        14.0,
                        theme.text_muted,
                    )),
                ]))))),
        )
    }
}

fn main() {
    let app = AnimationShowcase::new();
    let config = AppConfig {
        title: "Lever UI - Animation Showcase".to_string(),
        width: 900,
        height: 700,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };

    let application = Application::new(config, app);
    application.run();
}
