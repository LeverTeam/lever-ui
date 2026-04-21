use lever_core::app::App;
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Expanded, Flex, Label, Spacer};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct LayoutApp;

#[derive(Debug, Clone)]
enum Message {}

impl App for LayoutApp {
    type Message = Message;

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let sidebar = BoxWidget::new(Color::rgb(0.15, 0.15, 0.15))
            .with_padding(SideOffsets::all(20.0))
            .with_child(Box::new(Label::new(
                "Sidebar",
                24.0,
                Color::rgb(0.8, 0.8, 0.8),
            )));

        let main_content = Flex::column(vec![
            Box::new(Label::new(
                "Main Content Area",
                32.0,
                Color::rgb(1.0, 1.0, 1.0),
            )),
            Box::new(Spacer::new().with_flex(1)),
            Box::new(Flex::row(vec![
                Box::new(Expanded::new(Box::new(
                    BoxWidget::new(Color::rgb(0.2, 0.4, 0.2))
                        .with_radius(4.0)
                        .with_child(Box::new(Label::new(
                            "Flex 1",
                            16.0,
                            Color::rgb(1.0, 1.0, 1.0),
                        ))),
                ))),
                Box::new(Spacer::width(10.0)),
                Box::new(
                    Expanded::new(Box::new(
                        BoxWidget::new(Color::rgb(0.2, 0.2, 0.4))
                            .with_radius(4.0)
                            .with_child(Box::new(Label::new(
                                "Flex 2",
                                16.0,
                                Color::rgb(1.0, 1.0, 1.0),
                            ))),
                    ))
                    .with_flex(2),
                ),
            ])),
        ]);

        let root = Flex::row(vec![
            Box::new(sidebar),
            Box::new(Expanded::new(Box::new(
                BoxWidget::new(Color::rgb(0.1, 0.1, 0.1))
                    .with_padding(SideOffsets::all(20.0))
                    .with_child(Box::new(main_content)),
            ))),
        ]);

        Box::new(root)
    }
}

fn main() {
    let app = LayoutApp;
    let config = AppConfig {
        title: "Lever UI - Layout Demo".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}
