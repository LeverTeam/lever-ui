use lever_core::app::{App, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Button, Center, Flex, Label, Spacer, ThemeToggle};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
enum Message {
    ThemeModeChanged(ThemeMode),
    Log(String),
}

struct ThemeApp {
    theme_mode: ThemeMode,
}

impl App for ThemeApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ThemeModeChanged(mode) => self.theme_mode = mode,
            Message::Log(msg) => println!("{}", msg),
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        let color_swatch = |name: &str, color: Color| {
            Box::new(Flex::row(vec![
                Box::new(BoxWidget::new(color).with_size(24.0, 24.0).with_radius(6.0)),
                Box::new(Spacer::new().with_size(12.0, 10.0)),
                Box::new(Label::new(name, 14.0, theme.text)),
            ])) as Box<dyn lever_core::widget::Widget<Self::Message>>
        };

        let content = Flex::column(vec![
            Box::new(Flex::row(vec![
                Box::new(
                    Flex::column(vec![
                        Box::new(Label::new("Design System", 32.0, theme.text)),
                        Box::new(Label::new("Core Theme Tokens", 16.0, theme.text_muted)),
                    ])
                    .with_flex(1),
                ),
                Box::new(
                    ThemeToggle::new("theme-demo-toggle", self.theme_mode)
                        .on_changed(|mode| Message::ThemeModeChanged(mode)),
                ),
            ])),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            Box::new(Label::new("Brand Colors", 18.0, theme.text)),
            Box::new(Spacer::new().with_size(10.0, 15.0)),
            color_swatch("Primary", theme.primary),
            Box::new(Spacer::new().with_size(10.0, 8.0)),
            color_swatch("Secondary", theme.secondary),
            Box::new(Spacer::new().with_size(10.0, 8.0)),
            color_swatch("Success", theme.success),
            Box::new(Spacer::new().with_size(10.0, 8.0)),
            color_swatch("Danger", theme.danger),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            Box::new(Label::new("Interactive Components", 18.0, theme.text)),
            Box::new(Spacer::new().with_size(10.0, 15.0)),
            Box::new(Flex::row(vec![
                Box::new(
                    Button::new("Primary Button")
                        .with_color(theme.primary)
                        .on_click(|| Message::Log("Primary".into())),
                ),
                Box::new(Spacer::new().with_size(15.0, 10.0)),
                Box::new(
                    Button::new("Secondary")
                        .with_color(theme.secondary)
                        .on_click(|| Message::Log("Secondary".into())),
                ),
            ])),
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
    let app = ThemeApp {
        theme_mode: ThemeMode::Dark,
    };
    let config = AppConfig {
        title: "Lever UI - Theme Demo".to_string(),
        width: 800,
        height: 800,
        clear_color: Color::rgb(0.02, 0.02, 0.03),
    };
    let application = Application::new(config, app);
    application.run();
}
