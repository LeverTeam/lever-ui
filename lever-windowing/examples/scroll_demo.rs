use lever_core::app::App;
use lever_core::types::{Color, Point, SideOffsets};
use lever_core::widgets::{BoxWidget, Center, Flex, Label, ScrollWidget, Spacer};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct ScrollDemo {
    target_offset: Point,
    current_offset: Point,
}

#[derive(Debug, Clone)]
enum Message {
    ScrollTo(Point),
}

impl App for ScrollDemo {
    type Message = Message;

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ScrollTo(offset) => {
                self.target_offset = offset;
            }
        }
    }

    fn tick(&mut self, dt: f32) {
        let lerp_factor = 1.0 - (-15.0 * dt).exp();
        self.current_offset.x += (self.target_offset.x - self.current_offset.x) * lerp_factor;
        self.current_offset.y += (self.target_offset.y - self.current_offset.y) * lerp_factor;
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let mut items = Vec::new();
        for i in 1..=50 {
            items.push(Box::new(
                BoxWidget::new(if i % 2 == 0 {
                    Color::rgb(0.2, 0.2, 0.2)
                } else {
                    Color::rgb(0.15, 0.15, 0.15)
                })
                .with_padding(SideOffsets::all(15.0))
                .with_radius(4.0)
                .with_child(Box::new(Label::new(
                    format!("Item #{}", i),
                    16.0,
                    Color::rgb(0.9, 0.9, 0.9),
                ))),
            )
                as Box<dyn lever_core::widget::Widget<Self::Message>>);
            items.push(Box::new(Spacer::height(5.0)));
        }

        let content = Flex::column(items);

        Box::new(Center::new(Box::new(
            BoxWidget::new(Color::rgb(0.1, 0.1, 0.1))
                .with_radius(8.0)
                .with_padding(SideOffsets::all(20.0))
                .with_child(Box::new(
                    ScrollWidget::new(Box::new(content))
                        .with_offset(self.current_offset)
                        .on_scroll(Message::ScrollTo),
                )),
        )))
    }
}

fn main() {
    let app = ScrollDemo {
        target_offset: Point::default(),
        current_offset: Point::default(),
    };
    let config = AppConfig {
        title: "Lever UI - Smooth Scroll Demo".to_string(),
        width: 600,
        height: 500,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}
