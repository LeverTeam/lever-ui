use lever_core::app::{App, UpdateContext};
use lever_core::layout::GridTrack;
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Center, Flex, Grid, Label, ThemeToggle};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
pub enum Message {
    ThemeModeChanged(ThemeMode),
}

struct GridApp {
    theme_mode: ThemeMode,
}

impl App for GridApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ThemeModeChanged(mode) => self.theme_mode = mode,
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(Flex::column(vec![
                    Box::new(Flex::row(vec![
                        Box::new(Label::new("Grid Layout Demo", 32.0, theme.text).with_flex(1)),
                        Box::new(
                            ThemeToggle::new("grid-theme", self.theme_mode)
                                .on_changed(|mode| Message::ThemeModeChanged(mode)),
                        ),
                    ])),
                    Box::new(
                        Center::new(Box::new(
                            Grid::new(
                                vec![GridTrack::Fixed(200.0), GridTrack::Flex(1), GridTrack::Auto],
                                vec![
                                    Box::new(
                                        BoxWidget::new(theme.primary)
                                            .with_radius(theme.radius_md)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_child(Box::new(Label::new(
                                                "Fixed 200px",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                    Box::new(
                                        BoxWidget::new(theme.success)
                                            .with_radius(theme.radius_md)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_child(Box::new(Label::new(
                                                "Flex 1",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                    Box::new(
                                        BoxWidget::new(theme.danger)
                                            .with_radius(theme.radius_md)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_child(Box::new(Label::new(
                                                "Auto Track",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                    Box::new(
                                        BoxWidget::new(theme.secondary)
                                            .with_radius(theme.radius_md)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_child(Box::new(Label::new(
                                                "Row 2, Col 1",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                    Box::new(
                                        BoxWidget::new(theme.surface_variant)
                                            .with_radius(theme.radius_md)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_child(Box::new(Label::new(
                                                "Flex filling the middle",
                                                14.0,
                                                theme.text,
                                            ))),
                                    ),
                                    Box::new(
                                        BoxWidget::new(theme.surface)
                                            .with_radius(theme.radius_md)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_child(Box::new(Label::new(
                                                "End", 14.0, theme.text,
                                            ))),
                                    ),
                                ],
                            )
                            .with_gap(20.0)
                            .with_rows(vec![GridTrack::Fixed(80.0), GridTrack::Flex(1)]),
                        ))
                        .with_flex(1),
                    ),
                ]))),
        )
    }
}

fn main() {
    let config = AppConfig {
        title: "Lever UI Grid Example".to_string(),
        width: 1000,
        height: 800,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };

    let app = GridApp {
        theme_mode: ThemeMode::Dark,
    };

    let application = Application::new(config, app);
    application.run();
}
