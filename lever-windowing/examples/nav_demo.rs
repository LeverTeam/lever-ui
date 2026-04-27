use lever_core::app::{App, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{BoxShadow, Color, Point, SideOffsets};
use lever_core::widget::Widget;
use lever_core::widgets::{
    BoxWidget, Button, Dropdown, Flex, Label, ScrollWidget, Spacer, Tabs,
};
use lever_windowing::{AppConfig, Application};

struct NavDemoApp {
    theme_mode: ThemeMode,
    active_tab: usize,
    selected_option: Option<usize>,
}

#[derive(Clone, Debug)]
enum Message {
    ToggleTheme,
    TabChanged(usize),
    OptionSelected(usize),
}

impl App for NavDemoApp {
    type Message = Message;

    fn update(&mut self, message: Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ToggleTheme => {
                self.theme_mode = match self.theme_mode {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                };
                _ctx.set_theme(self.theme_mode);
            }
            Message::TabChanged(idx) => self.active_tab = idx,
            Message::OptionSelected(idx) => self.selected_option = Some(idx),
        }
    }

    fn view(&self) -> Box<dyn Widget<Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        Box::new(
            BoxWidget::new(theme.background)
                .with_child(Box::new(Flex::column(vec![
                    // Top Navigation Bar
                    Box::new(
                        BoxWidget::new(theme.surface)
                            .with_height(64.0)
                            .with_shadow(BoxShadow {
                                offset: Point { x: 0.0, y: 4.0 },
                                blur: 20.0,
                                color: Color::rgba(0.0, 0.0, 0.0, 0.2),
                            })
                            .with_padding(SideOffsets {
                                left: 24.0,
                                right: 24.0,
                                top: 0.0,
                                bottom: 0.0,
                            })
                            .with_child(Box::new(
                                Flex::row(vec![
                                    // Vertical centering container for Title
                                    Box::new(
                                        Flex::column(vec![Box::new(Label::new(
                                            "Lever UI",
                                            24.0,
                                            theme.text,
                                        ))])
                                        .with_main_axis_alignment(
                                            lever_core::layout::MainAxisAlignment::Center,
                                        ),
                                    ),
                                    Box::new(Spacer::new().with_flex(1)),
                                    // Navigation
                                    Box::new(
                                        Tabs::new(
                                            "main_tabs",
                                            vec![
                                                "Dashboard".into(),
                                                "Projects".into(),
                                                "Settings".into(),
                                            ],
                                            self.active_tab,
                                        )
                                        .on_change(Message::TabChanged),
                                    ),
                                    Box::new(Spacer::new().with_size(24.0, 10.0)),
                                    // Vertical centering container for Button
                                    Box::new(
                                        Flex::column(vec![Box::new(
                                            Button::new("Theme")
                                                .with_id("theme_btn")
                                                .on_click(|| Message::ToggleTheme),
                                        )])
                                        .with_main_axis_alignment(
                                            lever_core::layout::MainAxisAlignment::Center,
                                        ),
                                    ),
                                ])
                                .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch),
                            )),
                    ),

                    // Main Content
                    Box::new(
                        ScrollWidget::new(Box::new(
                            BoxWidget::new(Color::rgba(0.0, 0.0, 0.0, 0.0))
                                .with_padding(SideOffsets::all(40.0))
                                .with_child(Box::new(Flex::column(vec![
                                    Box::new(Label::new(
                                        match self.active_tab {
                                            0 => "Welcome to the Dashboard",
                                            1 => "Your Active Projects",
                                            2 => "Application Settings",
                                            _ => "Documentation",
                                        },
                                        32.0,
                                        theme.text
                                    )),
                                    Box::new(Spacer::new().with_size(10.0, 20.0)),
                                    Box::new(Label::new(
                                        "This demo showcases the new Overlay system. Open the dropdown below to see it float over the content.",
                                        16.0,
                                        theme.text_muted
                                    )),

                                    Box::new(Spacer::new().with_size(10.0, 40.0)),

                                    Box::new(Flex::row(vec![
                                        Box::new(
                                            BoxWidget::new(theme.surface)
                                                .with_radius(theme.radius_md)
                                                .with_padding(SideOffsets::all(24.0))
                                                .with_child(Box::new(Flex::column(vec![
                                                    Box::new(Label::new("Select Region", 18.0, theme.text)),
                                                    Box::new(Spacer::new().with_size(10.0, 12.0)),
                                                    Box::new(Dropdown::new(
                                                        "region_dropdown",
                                                        vec![
                                                            "North America".into(),
                                                            "Europe".into(),
                                                            "Asia Pacific".into(),
                                                            "South America".into(),
                                                            "Middle East".into(),
                                                            "Africa".into(),
                                                        ],
                                                        self.selected_option
                                                    ).on_select(Message::OptionSelected)),
                                                    Box::new(Spacer::new().with_size(10.0, 20.0)),
                                                    Box::new(Label::new(
                                                        format!("Selected: {}", self.selected_option.map_or("None", |i| {
                                                            match i {
                                                                0 => "North America",
                                                                1 => "Europe",
                                                                2 => "Asia Pacific",
                                                                3 => "South America",
                                                                4 => "Middle East",
                                                                _ => "Africa",
                                                            }
                                                        })),
                                                        14.0,
                                                        theme.primary
                                                    )),
                                                ]))),
                                        ),
                                        Box::new(Spacer::new().with_flex(1)),
                                    ])),

                                    // Add some vertical space to test scrolling
                                    Box::new(Spacer::new().with_size(10.0, 1000.0)),
                                ])))
                        ))
                        .with_flex(1)
                    ),
                ])))
        )
    }
}

fn main() {
    let app = NavDemoApp {
        theme_mode: ThemeMode::Dark,
        active_tab: 0,
        selected_option: None,
    };

    let config = AppConfig {
        title: "Lever Navigation Demo".to_string(),
        width: 1000,
        height: 800,
        ..Default::default()
    };

    Application::new(config, app).run();
}
