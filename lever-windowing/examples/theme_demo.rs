use lever_core::app::App;
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Button, Center, Flex, Label, Spacer};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct ThemeApp;

#[derive(Debug, Clone)]
enum Message {
    Log(String),
}

impl App for ThemeApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Log(msg) => println!("{}", msg),
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let title = Label::new("Design System", 48.0, Color::rgb(0.9, 0.9, 0.9));
        let subtitle = Label::new(
            "Dynamic Dark/Light Mode Support",
            24.0,
            Color::rgb(0.6, 0.6, 0.6),
        );

        let btn_primary =
            Button::new("Primary Action").on_click(|| Message::Log("Primary clicked".to_string()));

        let btn_custom = Button::new("Danger Action")
            .with_color(Color::rgb(0.8, 0.2, 0.2))
            .on_click(|| Message::Log("Custom clicked".to_string()));

        let content = Flex::column(vec![
            Box::new(title),
            Box::new(Spacer::height(10.0)),
            Box::new(subtitle),
            Box::new(Spacer::height(30.0)),
            Box::new(Flex::row(vec![
                Box::new(btn_primary),
                Box::new(Spacer::width(15.0)),
                Box::new(btn_custom),
            ])),
        ]);

        Box::new(Center::new(Box::new(
            BoxWidget::new(Color::rgba(0.1, 0.1, 0.1, 1.0))
                .with_radius(12.0)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(content)),
        )))
    }
}

fn main() {
    let app = ThemeApp;
    let config = AppConfig {
        title: "Lever UI - Theme Demo (Press 'T' to toggle)".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}
