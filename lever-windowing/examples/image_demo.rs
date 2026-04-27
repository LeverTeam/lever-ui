use lever_core::app::{App, Context, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, SideOffsets, Size, TextureId};
use lever_core::widgets::{BoxWidget, Center, Flex, ImageWidget, Label, Spacer, ThemeToggle};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

struct ImageDemo {
    texture: Option<TextureId>,
    theme_mode: ThemeMode,
}

#[derive(Clone, Debug)]
enum Message {
    ThemeModeChanged(ThemeMode),
}

impl App for ImageDemo {
    type Message = Message;

    fn init(&mut self, ctx: &mut Context<Self::Message>) {
        let bytes = include_bytes!("test.png");
        self.texture = Some(ctx.load_image(bytes));
    }

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ThemeModeChanged(mode) => self.theme_mode = mode,
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        if let Some(tex) = self.texture {
            Box::new(
                BoxWidget::new(theme.background)
                    .with_padding(SideOffsets::all(40.0))
                    .with_child(Box::new(Center::new(Box::new(Flex::column(vec![
                        Box::new(
                            Flex::row(vec![
                                Box::new(
                                    Flex::column(vec![
                                        Box::new(Label::new(
                                            "Image Rendering Demo",
                                            32.0,
                                            theme.text,
                                        )),
                                        Box::new(Label::new(
                                            "Asset loading and tinting",
                                            16.0,
                                            theme.text_muted,
                                        )),
                                    ])
                                    .with_flex(1),
                                ),
                                Box::new(
                                    ThemeToggle::new("image-theme", self.theme_mode)
                                        .on_changed(|mode| Message::ThemeModeChanged(mode)),
                                ),
                            ])
                            .with_gap(20.0),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 40.0)),
                        Box::new(Label::new("Original Asset (200x200):", 18.0, theme.text)),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(
                            BoxWidget::new(theme.surface)
                                .with_radius(theme.radius_md)
                                .with_padding(SideOffsets::all(20.0))
                                .with_child(Box::new(ImageWidget::new(
                                    tex,
                                    Size {
                                        width: 200.0,
                                        height: 200.0,
                                    },
                                ))),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 30.0)),
                        Box::new(Label::new("With Tint (Theme Primary):", 18.0, theme.text)),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(
                            ImageWidget::new(
                                tex,
                                Size {
                                    width: 120.0,
                                    height: 120.0,
                                },
                            )
                            .with_tint(theme.primary),
                        ),
                    ]))))),
            )
        } else {
            Box::new(Center::new(Box::new(Label::new(
                "Loading Asset...",
                24.0,
                theme.text,
            ))))
        }
    }
}

fn main() {
    let app = ImageDemo {
        texture: None,
        theme_mode: ThemeMode::Dark,
    };
    let config = AppConfig {
        title: "Lever UI - Image Demo".to_string(),
        width: 800,
        height: 800,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };
    let application = Application::new(config, app);
    application.run();
}
