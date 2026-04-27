use lever_core::app::{App, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Center, Flex, Label, Spacer, ThemeToggle};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
enum Message {
    ThemeModeChanged(ThemeMode),
}

struct TextApp {
    theme_mode: ThemeMode,
}

impl App for TextApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ThemeModeChanged(mode) => self.theme_mode = mode,
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        let content = Flex::column(vec![
            Box::new(Label::new("Lever UI", 64.0, theme.primary)),
            Box::new(Spacer::new().with_size(10.0, 10.0)),
            Box::new(Label::new(
                "High-Performance Text Rendering",
                28.0,
                theme.text,
            )),
            Box::new(Spacer::new().with_size(10.0, 20.0)),
            Box::new(Label::new(
                "Lever uses a custom GPU atlas batching system to render text with extreme efficiency, ensuring smooth frame rates even with thousands of characters.",
                16.0,
                theme.text_muted,
            )),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            Box::new(
                Flex::row(vec![
                    Box::new(Label::new("Switch Theme:", 14.0, theme.text_muted)),
                    Box::new(Spacer::new().with_size(15.0, 10.0)),
                    Box::new(
                        ThemeToggle::new("text-theme-toggle", self.theme_mode)
                            .on_changed(|mode| Message::ThemeModeChanged(mode)),
                    ),
                ])
            ),
        ]);

        Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(60.0))
                .with_child(Box::new(Center::new(Box::new(
                    BoxWidget::new(theme.surface)
                        .with_radius(theme.radius_lg)
                        .with_padding(SideOffsets::all(40.0))
                        .with_child(Box::new(content)),
                )))),
        )
    }
}

fn main() {
    let app = TextApp {
        theme_mode: ThemeMode::Dark,
    };
    let config = AppConfig {
        title: "Lever UI - Text Demo".to_string(),
        width: 900,
        height: 700,
        clear_color: Color::rgb(0.02, 0.02, 0.03),
    };
    let application = Application::new(config, app);
    application.run();
}
