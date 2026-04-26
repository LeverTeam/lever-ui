use lever_core::animation::{Animation, Ease};
use lever_core::app::{App, UpdateContext};
use lever_core::types::Color;
use lever_core::widgets::{Button, Center, Flex, Label, Spacer};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct AnimationDemo {
    hover_anim: Animation,
}

#[derive(Debug, Clone)]
enum Message {
    Click,
}

impl App for AnimationDemo {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::Click => {
                println!("Button clicked!");
            }
        }
    }

    fn tick(&mut self, dt: f32) {
        self.hover_anim.update(dt);
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        // Animate color between blue and purple
        let start_color = Color::rgb(0.2, 0.4, 0.8);
        let end_color = Color::rgb(0.6, 0.2, 0.9);

        let button_color = self.hover_anim.color(start_color, end_color);

        Box::new(Center::new(Box::new(Flex::column(vec![
            Box::new(Label::new(
                "Animation & Transitions",
                24.0,
                Color::rgb(1.0, 1.0, 1.0),
            )),
            Box::new(Spacer::height(40.0)),
            Box::new(
                Button::new("Animated Button")
                    .with_color(button_color)
                    .on_click(|| Message::Click),
            ),
            Box::new(Spacer::height(20.0)),
            Box::new(Label::new(
                format!("Progress: {:.2}", self.hover_anim.progress()),
                14.0,
                Color::rgb(0.6, 0.6, 0.6),
            )),
        ]))))
    }
}

fn main() {
    let mut app = AnimationDemo {
        hover_anim: Animation::new(2.0, Ease::CubicInOut).with_loop(),
    };
    app.hover_anim.start();

    let config = AppConfig {
        title: "Lever Animation Demo".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}

