use lever_core::app::{App, UpdateContext};
use lever_core::layout::GridTrack;
use lever_core::theme::ThemeMode;
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{BoxWidget, Button, Flex, Grid, Label, Spacer, ThemeToggle, Toggle};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(String, usize),
    ButtonClicked(String),
    ToggleChanged(bool),
    SliderChanged(f32),
    CheckboxChanged(bool),
    ThemeModeChanged(ThemeMode),
}

struct GalleryApp {
    input_text: String,
    cursor_index: usize,
    toggle_on: bool,
    slider_value: f32,
    checkbox_checked: bool,
    theme_mode: ThemeMode,
}

impl App for GalleryApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _context: &mut UpdateContext) {
        match message {
            Message::TextChanged(text, cursor) => {
                self.input_text = text;
                self.cursor_index = cursor;
            }
            Message::ButtonClicked(label) => {
                println!("Button clicked: {}", label);
            }
            Message::ToggleChanged(val) => {
                self.toggle_on = val;
            }
            Message::SliderChanged(val) => {
                self.slider_value = val;
            }
            Message::CheckboxChanged(val) => {
                self.checkbox_checked = val;
            }
            Message::ThemeModeChanged(mode) => {
                self.theme_mode = mode;
            }
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = lever_core::theme::Theme::for_mode(self.theme_mode);

        Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(
                    Flex::column(vec![
                        // Header row
                        Box::new(
                            Flex::row(vec![
                                Box::new(
                                    Flex::column(vec![
                                        Box::new(Label::new("Lever UI Gallery", 32.0, theme.text)),
                                        Box::new(Label::new(
                                            "A showcase of core widgets.",
                                            16.0,
                                            theme.text_muted,
                                        )),
                                    ])
                                    .with_flex(1),
                                ),
                                Box::new(
                                    ThemeToggle::new("theme-toggle", self.theme_mode)
                                        .on_changed(|mode| Message::ThemeModeChanged(mode)),
                                ),
                            ])
                            .with_gap(20.0),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 30.0)),
                        // Grid Section
                        Box::new(Label::new("Grid Layout System", 20.0, theme.text)),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(
                            Grid::new(
                                vec![GridTrack::Flex(1), GridTrack::Flex(1), GridTrack::Flex(1)],
                                vec![
                                    Box::new(
                                        BoxWidget::new(theme.primary)
                                            .with_radius(8.0)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_child(Box::new(Label::new(
                                                "Column 1",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                    Box::new(
                                        BoxWidget::new(theme.success)
                                            .with_radius(8.0)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_child(Box::new(Label::new(
                                                "Column 2",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                    Box::new(
                                        BoxWidget::new(theme.danger)
                                            .with_radius(8.0)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_child(Box::new(Label::new(
                                                "Column 3",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                ],
                            )
                            .with_gap(15.0),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 30.0)),
                        // Interactive Section
                        Box::new(Label::new("Interactive Widgets", 20.0, theme.text)),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(
                            Flex::row(vec![
                                Box::new(
                                    Button::new("Primary Button")
                                        .with_color(theme.primary)
                                        .on_click(|| Message::ButtonClicked("Primary".into())),
                                ),
                                Box::new(
                                    Button::new("Success Button")
                                        .with_color(theme.success)
                                        .on_click(|| Message::ButtonClicked("Success".into())),
                                ),
                                Box::new(
                                    Toggle::new("gallery-toggle", self.toggle_on)
                                        .on_changed(|val| Message::ToggleChanged(val)),
                                ),
                            ])
                            .with_gap(15.0),
                        ),
                    ])
                    .with_gap(20.0),
                )),
        )
    }
}

fn main() {
    let config = AppConfig {
        title: "Lever UI Gallery".to_string(),
        width: 900,
        height: 700,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };

    let app = GalleryApp {
        input_text: "Hello Lever!".to_string(),
        cursor_index: 12,
        toggle_on: true,
        slider_value: 0.5,
        checkbox_checked: false,
        theme_mode: ThemeMode::Dark,
    };

    let application = Application::new(config, app);
    application.run();
}
