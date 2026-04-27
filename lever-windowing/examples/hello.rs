use lever_core::app::{App, UpdateContext};
use lever_core::theme::Theme;
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Center, Label};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct HelloApp;

#[derive(Debug, Clone)]
enum Message {}

impl App for HelloApp {
    type Message = Message;

    fn update(&mut self, _message: Self::Message, _ctx: &mut UpdateContext) {}

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = Theme::dark();

        Box::new(Center::new(Box::new(
            BoxWidget::new(theme.primary)
                .with_radius(theme.radius_lg)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(Label::new("Hello Lever!", 32.0, Color::WHITE))),
        )))
    }
}

fn main() {
    let app = HelloApp;
    let config = AppConfig {
        title: "Lever UI - Hello".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.02, 0.02, 0.03),
    };
    let application = Application::new(config, app);
    application.run();
}
