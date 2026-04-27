use lever_core::app::{App, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Button, Center, Flex, Label, Spacer, ThemeToggle};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    ThemeModeChanged(ThemeMode),
}

struct CounterApp {
    count: i32,
    theme_mode: ThemeMode,
}

impl App for CounterApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
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
                    BoxWidget::new(theme.surface)
                        .with_radius(theme.radius_lg)
                        .with_padding(SideOffsets::all(40.0))
                        .with_child(Box::new(Flex::column(vec![
                            Box::new(Flex::row(vec![
                                Box::new(
                                    Label::new("Counter App", 16.0, theme.text_muted).with_flex(1),
                                ),
                                Box::new(
                                    ThemeToggle::new("counter-theme", self.theme_mode)
                                        .on_changed(|mode| Message::ThemeModeChanged(mode)),
                                ),
                            ])),
                            Box::new(Spacer::new().with_size(10.0, 30.0)),
                            Box::new(Label::new(format!("{}", self.count), 72.0, theme.text)),
                            Box::new(Spacer::new().with_size(10.0, 30.0)),
                            Box::new(Flex::row(vec![
                                Box::new(
                                    Button::new("Decrement")
                                        .with_color(theme.danger)
                                        .on_click(|| Message::Decrement),
                                ),
                                Box::new(Spacer::new().with_size(15.0, 10.0)),
                                Box::new(
                                    Button::new("Increment")
                                        .with_color(theme.success)
                                        .on_click(|| Message::Increment),
                                ),
                            ])),
                        ]))),
                )))),
        )
    }
}

fn main() {
    let app = CounterApp {
        count: 0,
        theme_mode: ThemeMode::Dark,
    };
    let config = AppConfig {
        title: "Lever Counter Demo".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}
