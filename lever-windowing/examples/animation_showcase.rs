use lever_core::animation::{AnimationController, Ease};
use lever_core::app::{App, Context};
use lever_core::types::{Color, Point, };
use lever_core::widgets::*;
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
enum Message {
    ToggleVisibility,
}

struct AnimationShowcase {
    opacity_ctrl: AnimationController,
    slide_ctrl: AnimationController,
    is_visible: bool,
}

impl AnimationShowcase {
    fn new() -> Self {
        Self {
            opacity_ctrl: AnimationController::new(1.0),
            slide_ctrl: AnimationController::new(0.0),
            is_visible: true,
        }
    }
}

impl App for AnimationShowcase {
    type Message = Message;

    fn init(&mut self, _ctx: &mut Context<Self::Message>) {}

    fn update(&mut self, message: Self::Message) {
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
        }
    }

    fn tick(&mut self, dt: f32) {
        self.opacity_ctrl.tick(dt);
        self.slide_ctrl.tick(dt);
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        Box::new(Center::new(Box::new(Flex::column(vec![
            Box::new(Label::new(
                "Click the button to fade and slide".to_string(),
                24.0,
                Color::WHITE,
            )),
            Box::new(Spacer::height(20.0)),
            Box::new(
                Button::new("Toggle Animation".to_string()).on_click(|| Message::ToggleVisibility),
            ),
            Box::new(Spacer::height(20.0)),
            Box::new(AnimatedTranslation::new(
                Point {
                    x: 0.0,
                    y: self.slide_ctrl.value(),
                },
                Box::new(AnimatedOpacity::new(
                    self.opacity_ctrl.value(),
                    Box::new(
                        BoxWidget::new(Color::rgb(0.2, 0.6, 1.0))
                            .with_size(200.0, 200.0)
                            .with_radius(20.0),
                    ),
                )),
            )),
            Box::new(Spacer::height(20.0)),
            Box::new(Label::new(
                format!("Offset Y: {:.2}", self.slide_ctrl.value()),
                18.0,
                Color::rgba(1.0, 1.0, 1.0, 0.7),
            )),
        ]))))
    }
}

fn main() {
    let app = AnimationShowcase::new();
    let config = AppConfig {
        title: "Lever UI - Animation Showcase".to_string(),
        width: 800,
        height: 600,
        clear_color: Color::rgb(0.1, 0.1, 0.1),
    };

    let application = Application::new(config, app);
    application.run();
}

