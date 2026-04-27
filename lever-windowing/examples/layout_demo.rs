use lever_core::app::{App, UpdateContext};
use lever_core::layout::GridTrack;
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Expanded, Flex, Grid, Label, Spacer, ThemeToggle};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
enum Message {
    ThemeModeChanged(ThemeMode),
}

struct LayoutApp {
    theme_mode: ThemeMode,
}

impl App for LayoutApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ThemeModeChanged(mode) => self.theme_mode = mode,
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        let sidebar = BoxWidget::new(theme.surface)
            .with_padding(SideOffsets::all(24.0))
            .with_child(Box::new(Flex::column(vec![
                Box::new(Label::new("Lever UI", 20.0, theme.primary)),
                Box::new(Spacer::new().with_size(10.0, 32.0)),
                Box::new(Label::new("Dashboard", 14.0, theme.text)),
                Box::new(Spacer::new().with_size(10.0, 16.0)),
                Box::new(Label::new("Settings", 14.0, theme.text_muted)),
                Box::new(Spacer::new().with_size(10.0, 16.0)),
                Box::new(Label::new("Profile", 14.0, theme.text_muted)),
            ])));

        let main_content = Flex::column(vec![
            Box::new(
                Flex::row(vec![
                    Box::new(Label::new("Complex Layout Demo", 32.0, theme.text).with_flex(1)),
                    Box::new(
                        ThemeToggle::new("layout-theme", self.theme_mode)
                            .on_changed(|mode| Message::ThemeModeChanged(mode)),
                    ),
                ])
                .with_gap(20.0),
            ),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            Box::new(Label::new("Nested Flex Sections", 18.0, theme.text)),
            Box::new(Spacer::new().with_size(10.0, 15.0)),
            Box::new(Flex::row(vec![
                Box::new(Expanded::new(Box::new(
                    BoxWidget::new(theme.primary)
                        .with_radius(theme.radius_md)
                        .with_padding(SideOffsets::all(20.0))
                        .with_child(Box::new(Label::new("Flex 1", 14.0, Color::WHITE))),
                ))),
                Box::new(Spacer::new().with_size(15.0, 10.0)),
                Box::new(
                    Expanded::new(Box::new(
                        BoxWidget::new(theme.secondary)
                            .with_radius(theme.radius_md)
                            .with_padding(SideOffsets::all(20.0))
                            .with_child(Box::new(Label::new(
                                "Flex 2 (Higher Weight)",
                                14.0,
                                Color::WHITE,
                            ))),
                    ))
                    .with_flex(2),
                ),
            ])),
            Box::new(Spacer::new().with_size(10.0, 40.0)),
            Box::new(Label::new("Grid Integration", 18.0, theme.text)),
            Box::new(Spacer::new().with_size(10.0, 15.0)),
            Box::new(
                Grid::new(
                    vec![GridTrack::Flex(1), GridTrack::Flex(1), GridTrack::Flex(1)],
                    vec![
                        Box::new(
                            BoxWidget::new(theme.surface_variant)
                                .with_radius(8.0)
                                .with_padding(SideOffsets::all(15.0))
                                .with_child(Box::new(Label::new("A", 12.0, theme.text))),
                        ),
                        Box::new(
                            BoxWidget::new(theme.surface_variant)
                                .with_radius(8.0)
                                .with_padding(SideOffsets::all(15.0))
                                .with_child(Box::new(Label::new("B", 12.0, theme.text))),
                        ),
                        Box::new(
                            BoxWidget::new(theme.surface_variant)
                                .with_radius(8.0)
                                .with_padding(SideOffsets::all(15.0))
                                .with_child(Box::new(Label::new("C", 12.0, theme.text))),
                        ),
                    ],
                )
                .with_gap(10.0),
            ),
            Box::new(Spacer::new().with_flex(1)),
            Box::new(Label::new("Bottom Anchored Text", 14.0, theme.text_muted)),
        ]);

        let root = Grid::new(
            vec![GridTrack::Fixed(240.0), GridTrack::Flex(1)],
            vec![
                Box::new(sidebar),
                Box::new(
                    BoxWidget::new(theme.background)
                        .with_padding(SideOffsets::all(40.0))
                        .with_child(Box::new(main_content)),
                ),
            ],
        );

        Box::new(root)
    }
}

fn main() {
    let config = AppConfig {
        title: "Lever UI - Layout Demo".to_string(),
        width: 1000,
        height: 750,
        clear_color: Color::rgb(0.02, 0.02, 0.03),
    };

    let app = LayoutApp {
        theme_mode: ThemeMode::Dark,
    };

    let application = Application::new(config, app);
    application.run();
}
