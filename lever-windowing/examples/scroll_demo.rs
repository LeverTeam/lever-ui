use lever_core::app::{App, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, Point, SideOffsets};
use lever_core::widgets::{BoxWidget, Flex, Label, ScrollWidget, Spacer, ThemeToggle};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
enum Message {
    ScrollTo(Point),
    ThemeModeChanged(ThemeMode),
}

struct ScrollDemo {
    target_offset: Point,
    current_offset: Point,
    theme_mode: ThemeMode,
}

impl App for ScrollDemo {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ScrollTo(offset) => {
                self.target_offset = offset;
            }
            Message::ThemeModeChanged(mode) => {
                self.theme_mode = mode;
            }
        }
    }

    fn tick(&mut self, dt: f32) {
        let lerp_factor = 1.0 - (-15.0 * dt).exp();
        self.current_offset.x += (self.target_offset.x - self.current_offset.x) * lerp_factor;
        self.current_offset.y += (self.target_offset.y - self.current_offset.y) * lerp_factor;
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        let mut items = Vec::new();
        for i in 1..=100 {
            items.push(Box::new(
                BoxWidget::new(if i % 2 == 0 {
                    theme.surface
                } else {
                    theme.surface_variant
                })
                .with_padding(SideOffsets::all(20.0))
                .with_radius(theme.radius_md)
                .with_child(Box::new(Label::new(
                    format!("Scroll Item #{}", i),
                    16.0,
                    theme.text,
                ))),
            )
                as Box<dyn lever_core::widget::Widget<Self::Message>>);
            items.push(Box::new(Spacer::new().with_size(10.0, 8.0)));
        }

        let content = Flex::column(items);

        Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(Flex::column(vec![
                    Box::new(Flex::row(vec![
                        Box::new(
                            Flex::column(vec![
                                Box::new(Label::new("Smooth Scroll Demo", 32.0, theme.text)),
                                Box::new(Label::new(
                                    "Scroll with mouse wheel or trackpad",
                                    16.0,
                                    theme.text_muted,
                                )),
                            ])
                            .with_flex(1),
                        ),
                        Box::new(
                            ThemeToggle::new("scroll-theme", self.theme_mode)
                                .on_changed(|mode| Message::ThemeModeChanged(mode)),
                        ),
                    ])),
                    Box::new(Spacer::new().with_size(10.0, 30.0)),
                    Box::new(
                        ScrollWidget::new(Box::new(content))
                            .with_offset(self.current_offset)
                            .on_scroll(Message::ScrollTo)
                            .with_flex(1),
                    ),
                ]))),
        )
    }
}

fn main() {
    let app = ScrollDemo {
        target_offset: Point::default(),
        current_offset: Point::default(),
        theme_mode: ThemeMode::Dark,
    };
    let config = AppConfig {
        title: "Lever UI - Smooth Scroll Demo".to_string(),
        width: 800,
        height: 700,
        clear_color: Color::rgb(0.02, 0.02, 0.03),
    };
    let application = Application::new(config, app);
    application.run();
}
