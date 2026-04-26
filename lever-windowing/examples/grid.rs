use lever_core::app::{App, UpdateContext};
use lever_core::layout::GridTrack;
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Center, Grid, Label};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
pub enum Message {}

struct GridApp;

impl App for GridApp {
    type Message = Message;

    fn update(&mut self, _message: Self::Message, _ctx: &mut UpdateContext) {}

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        Box::new(Center::new(Box::new(
            Grid::new(
                vec![GridTrack::Fixed(150.0), GridTrack::Flex(1), GridTrack::Auto],
                vec![
                    Box::new(
                        BoxWidget::new(Color::rgb(0.8, 0.2, 0.2))
                            .with_radius(8.0)
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(Label::new("Fixed 150px", 14.0, Color::WHITE))),
                    ),
                    Box::new(
                        BoxWidget::new(Color::rgb(0.2, 0.8, 0.2))
                            .with_radius(8.0)
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(Label::new("Flex 1", 14.0, Color::WHITE))),
                    ),
                    Box::new(
                        BoxWidget::new(Color::rgb(0.2, 0.2, 0.8))
                            .with_radius(8.0)
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(Label::new(
                                "Auto Track (Content size)",
                                14.0,
                                Color::WHITE,
                            ))),
                    ),
                    Box::new(
                        BoxWidget::new(Color::rgb(0.8, 0.8, 0.2))
                            .with_radius(8.0)
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(Label::new("Row 2, Col 1", 14.0, Color::BLACK))),
                    ),
                    Box::new(
                        BoxWidget::new(Color::rgb(0.2, 0.8, 0.8))
                            .with_radius(8.0)
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(Label::new(
                                "Flex filling the middle",
                                14.0,
                                Color::BLACK,
                            ))),
                    ),
                    Box::new(
                        BoxWidget::new(Color::rgb(0.8, 0.2, 0.8))
                            .with_radius(8.0)
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(Label::new("End", 14.0, Color::WHITE))),
                    ),
                    Box::new(
                        BoxWidget::new(Color::rgb(0.5, 0.5, 0.5))
                            .with_radius(8.0)
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(Label::new("Row 3", 14.0, Color::WHITE))),
                    ),
                    Box::new(
                        BoxWidget::new(Color::rgb(0.3, 0.3, 0.3))
                            .with_radius(8.0)
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(Label::new(
                                "Grid Layout Example",
                                14.0,
                                Color::WHITE,
                            ))),
                    ),
                    Box::new(
                        BoxWidget::new(Color::rgb(0.1, 0.1, 0.1))
                            .with_radius(8.0)
                            .with_padding(SideOffsets::all(10.0))
                            .with_child(Box::new(Label::new("Final cell", 14.0, Color::WHITE))),
                    ),
                ],
            )
            .with_gap(15.0)
            .with_rows(vec![
                GridTrack::Fixed(60.0),
                GridTrack::Flex(1),
                GridTrack::Auto,
            ]),
        )))
    }
}

fn main() {
    let config = AppConfig {
        title: "Lever UI Grid Example".to_string(),
        width: 1000,
        height: 800,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };

    let app = GridApp;

    let application = Application::new(config, app);
    application.run();
}
