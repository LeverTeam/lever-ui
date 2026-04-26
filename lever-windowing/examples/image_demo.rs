use lever_core::app::{App, Context, UpdateContext};
use lever_core::types::{Color, Size, TextureId};
use lever_core::widgets::{BoxWidget, Center, Flex, ImageWidget, Label};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Clone)]
struct ImageDemo {
    texture: Option<TextureId>,
}

impl App for ImageDemo {
    type Message = ();

    fn init(&mut self, ctx: &mut Context<Self::Message>) {
        let bytes = include_bytes!("test.png");
        self.texture = Some(ctx.load_image(bytes));
    }

    fn update(&mut self, _message: Self::Message, _ctx: &mut UpdateContext) {}

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        if let Some(tex) = self.texture {
            Box::new(Center::new(Box::new(
                Flex::column(vec![
                    Box::new(Label::new(
                        "Loaded Asset (1x1 Red PNG scaled):",
                        18.0,
                        Color::rgb(1.0, 1.0, 1.0),
                    )),
                    Box::new(
                        BoxWidget::new(Color::rgb(0.2, 0.2, 0.2))
                            .with_radius(8.0)
                            .with_child(Box::new(ImageWidget::new(
                                tex,
                                Size {
                                    width: 200.0,
                                    height: 200.0,
                                },
                            ))),
                    ),
                    Box::new(Label::new(
                        "With Tint (Greenish):",
                        18.0,
                        Color::rgb(1.0, 1.0, 1.0),
                    )),
                    Box::new(
                        ImageWidget::new(
                            tex,
                            Size {
                                width: 100.0,
                                height: 100.0,
                            },
                        )
                        .with_tint(Color::rgb(0.0, 1.0, 0.5)),
                    ),
                ])
                .with_gap(20.0),
            )))
        } else {
            Box::new(Center::new(Box::new(Label::new(
                "Loading...",
                24.0,
                Color::rgb(1.0, 1.0, 1.0),
            ))))
        }
    }
}

fn main() {
    let app = ImageDemo { texture: None };
    let config = AppConfig {
        title: "Lever UI - Image Demo".to_string(),
        width: 600,
        height: 600,
        clear_color: Color::rgb(0.1, 0.1, 0.1),
    };
    let application = Application::new(config, app);
    application.run();
}
